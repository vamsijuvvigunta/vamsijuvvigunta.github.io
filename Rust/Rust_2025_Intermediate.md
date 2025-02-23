# Intermediate rust usage

Rust is a fairly new langiage to me. However, as I refactor code over time, certain code snippets are worth talking about. I hope these provide some inspiration in your rust journey.        

## Use Case - Macros to perform enum variant filtering

The context: _A message passing system where the message is of type `AgenticMessages`. The message receiver has to select a specific type of message: something that might normally be done using custom pattern matching code._

```rust
#[derive(From, Debug, Clone)]
pub enum AgenticMessages {
    #[from]
    AMPrompt(LLMPromptMsg),

    #[from]
    AMPromptResponse(LLMResponseMsg),

    #[from]    
    AMToolCall(ToolCallMsg),

    #[from]
    AMToolResponse(ToolResponseMsg),
}
```

Say I wanted to select `AMPromptMsgs`: I would use something like this..

```rust
// These methods 
impl AgenticMessages 
{
    pub fn func_take_llmprompt_msgs(mut msgs: Vec<AgenticMessages>) 
        -> (Vec<LLMPromptMsg>, Vec<AgenticMessages>) 
    {
        build_block_pattern_partition!(
            msgs,
            AgenticMessages::AMPrompt(inner @ _), inner)
    }

    pub fn func_take_llmresponse_msgs(mut msgs: Vec<AgenticMessages>) 
        -> (Vec<LLMResponseMsg>, Vec<AgenticMessages>) 
    {
        build_block_pattern_partition!(
            msgs,
            AgenticMessages::AMPromptResponse(inner @ _), inner)
    }

    pub fn func_take_toolcall_msgs(mut msgs: Vec<AgenticMessages>) 
        -> (Vec<ToolCallMsg>, Vec<AgenticMessages>) 
    {
        build_block_pattern_partition!(
            msgs,
            AgenticMessages::AMToolCall(inner @ _), inner)
    }

    pub fn func_take_toolresponse_msgs(mut msgs: Vec<AgenticMessages>) 
        -> (Vec<ToolResponseMsg>, Vec<AgenticMessages>) 
    {
        build_block_pattern_partition!(
            msgs,
            AgenticMessages::AMToolResponse(inner @ _), inner)
    }
}
```

These functions wrap the `build_block_pattern_partition` macro. For instance,

```rust
build_block_pattern_partition!(
            msgs,
            AgenticMessages::AMToolResponse(inner @ _), inner)
```

The pattern `AgenticMessages::AMToolResponse(inner @ _)` is used to match incoming messages. If matched, the binding `inner` is returned as the `left` partition _(i.e, the first `Vec<ToolResponseMsg>` of the tuple)_. It is beautiful to get full static typing with these macros.

## Macro to use a pattern match to partition a vector of items.

```rust
//---------------------------------------------------------------------------
/// Send the approp pattern when matching
///   Enum::Variant
///   Enum::TupleVariant(_)
///   Enum::StructVariant{..}
///
/// $pattern and $binding work together. For eg..
/// 
///   pattern = outer @ SomeEnum                   , binding = outer
///             outer @ SomeStructEnum{_}          , binding = outer
///                     SomeStructEnum{inner @ _}  , binding = inner
///                     SomeTupleEnum (inner @ _)  , binding = inner
///---------------------------------------------------------------------------
/// Returns a partioned vectors (wanted, unwanted) with 
///    wanted   ← containing the bound portion of the items that match the pattern
///    unwanted ← the items that do not match the pattern
#[macro_export]
macro_rules! build_block_pattern_partition { 

    ($vals:expr, $pattern:pat, $binding:tt) => {
        {
            let (wanted, unwanted) : (Vec<_>, Vec<_>) = $vals.drain(..)
                .partition_map(|v| match v {
                    $pattern => Either::Left($binding),
                    fail @ _ => Either::Right(fail)
                });
            
            (wanted, unwanted)
        }
    };
}
```

Pretty straightforwrd to understand once you know the macro types

 - `$vals:expr` is the first part of the macro: an expression.
 - `$pattern:pat` is the second argument which is expected to be a pattern
 - `$binding:tt` is a token stream representing the binding.

If the pattern is mathced, the bound value is returned as the left `Either::Left` arm, otherwise, the `Either::Right`. This way, `partition_map` splits the vector into two.

Note the `vals.drain(..)` which consumes the vector. 

## Wrpper around the macro

```rust
pub struct MsgSource<T> {
    pub msgs: Vec<T>,
}

impl<T> MsgSource<T> {

    pub fn new() -> Self {
        MsgSource{ msgs: vec![]}
    }

    pub fn from(src: Vec<T>) -> Self {
        MsgSource{ msgs: src}
    }

    /// Left/Right Split
    /// L : The retained (left) split teed'd off an extended into `taken`. 
    ///     Note that Vec<T> implements the Extend<T> trait
    /// 
    /// T : The remaining ones retained in Self that is also 
    ///     returned to allow further chaining
    pub fn split_take<
        F : Fn(Vec<T>) -> (Vec<L> /*left-side:taken*/, Vec<T> /*right-side:remaining*/),        
        L
        >(mut self, take_into:&mut impl Extend<L>, filter_func:F) -> MsgSource<T> {
            let (take, remain) = filter_func(self.msgs.drain(..).collect());

            take_into.extend(take);

            self.msgs.extend(remain);
            self            
    }

    /// Left/Right Split
    /// L : The retained (left) split teed'd off into `taken`
    /// T : The remaining ones 
    /// 
    /// Returns (Vec<L>, Vec<T>)
    pub fn split_drain<
        F : Fn(Vec<T>) -> (Vec<L> /*left-side:taken*/, Vec<T> /*right-side:remaining*/),
        L
        >(mut self, filter_func:F) -> (Vec<L>, Vec<T>) {
            let (left, right) = filter_func(self.msgs.drain(..).collect());

            (left, right)
    }
}

// conversions from plain vectors
impl<T> From<Vec<T>> for MsgSource<T> {
    fn from(value: Vec<T>) -> Self {
        MsgSource::from(value)
    }
}
```

This packages the macro into a struct that allows composition of various `split_take` calls allowing multiple types of matched messages to be taken out of a single msg source.

# Functional chaining

```rust
let mut resp = my_msgs.drain(..).map( |tm| {            
    serde_json::from_value::<S>(tm.tool_call.fn_arguments.clone())
    .map_err(|se| 
        AgenticError::ToolCallFunctionArgDeserializationError(
            format!("{:?}:{:?}", tm.tool_call.fn_arguments, se)))
    .and_then(|args| {
        debug!("Calling Tool Closure on args: {:?}", args);
        (self.tool_func)(args)
    })
    .and_then(|tool_str_response| {
        debug!("Tool: {:?} responded with {:?}", self.name, tool_str_response);
        Ok(ToolResponseMsg{                    
            call_response: ToolResponse::new(&tm.tool_call.call_id, tool_str_response),
            call_msg: tm,
        }.into())
    })
})
.collect_vec();
```

----

```rust
async fn exec(
        &self,
        ctx: Arc<Mutex<PregelContext>>,
        in_msgs: Option<Vec<Self::Message>>,
    ) -> Result<Self::Message, Self::Error> {
        debug!("Exec called on MsgTransformWrapperNode({:?})", self.name);

        self.wrapped_node
            .exec(ctx, in_msgs)
            .await
            .map_err(|err| AgenticError::NestedNodeExecFailed(
                format!("Inner Node:{:?} errored during exec [{:?}]", self.wrapped_node.name(), err)
            ).into())
            .inspect(|inner_msg| debug!("Inner Node suceeded: Transforming it's output Msg: {:?}", inner_msg))
            .and_then(|inner_msg| (self.xform_func)(inner_msg))
            .inspect(|xformed_msg| debug!("Transformed to Msg: {:?}", xformed_msg))
    }
```

There is an inordinate sense of joy when I am able to code up something like this.
 - Minimal to no syntactical boiler plate
 - The intended logic flow is very clear
 - No need to invent new function names
 - Monadic chaining: so elegant when the syntacting sugaring and language design gets it right

There are methods to convert a `Result` ⇄ `Option` so you can build your chains in either case. 
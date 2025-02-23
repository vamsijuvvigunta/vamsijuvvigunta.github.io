
use async_trait::async_trait;
use genai::chat::ToolResponse;
use itertools::{Either, Itertools};
use serde::de::DeserializeOwned;
use std::marker::PhantomData;
use std::{result::Result, sync::Arc};
use tokio::sync::Mutex;
use tracing::{debug, warn};

use lib_pregel::pregel_types::{PregelContext, PregelNode};

use crate::agentic_errors::AgenticError;
use crate::agentic_msgs::AgenticMessages;
use crate::util::macros_prelude::*;
use crate::nodes::tool::ToolResponseMsg;


//- Node ----------------------------------------------------
/// The S used must derive from Deserialize so that serde can 
/// deserialize the stringified JSON into it. Likely also derives
/// from `schemars::JsonSchema` so we can generate schema automatically.
///------------------------------------------------------------
/// 👉 TODO: Make F async!
#[derive(Debug)]
pub struct GenericToolNode<F, S>
{
    // Pregel node fields
    pub name        : String,
    pub description : String,    

    // Tool specific
    tool_func       : F,    
    tool_arg_obj    : PhantomData<S>,
}


impl<F, S> GenericToolNode<F, S>
where 
    S : DeserializeOwned
{
    pub fn new(func: F) -> Self {
        GenericToolNode{
            name           : "GenericToolNode".to_string(),
            description    : "Generic Tool Node that uses a closure to implement the tool".to_string(),
            tool_func      : func,            
            tool_arg_obj   : PhantomData,
        }        
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }    
}


//-- Impl PregelNode -------------------------------------------
// For now, only tools that can return string response. Later, it 
// will be anything that can fit into genai's ToolResponse.
#[async_trait]
impl<F, S> PregelNode for GenericToolNode<F, S>
where    
    S    : Send + Sync + Clone + std::fmt::Debug + DeserializeOwned, 
    F    : Fn(S) -> std::result::Result<String, AgenticError> + Sync, 
{
    type Error   = AgenticError;
    type Message = AgenticMessages;

    fn name(&self) -> &String {
        &self.name
    }

    fn description(&self) -> &String {
        &self.description
    }

    async fn exec(
        &self,
        _ctx: Arc<Mutex<PregelContext>>,
        mut in_msgs: Option<Vec<Self::Message>>,
    ) -> Result<Self::Message, Self::Error> {
        debug!("Exec called on GenericToolNode({:?})", self.name);        

        // -- Input Wrangling ------------------------
        // get all ToolCallMessages        
        let mut tool_msgs = build_block_extract_left_partition_warn!(
            in_msgs,
            format!("{}[GenericToolNode]", self.name),
            AgenticMessages::AMToolCall(inner @ _), inner,
            WarnFlags::WARN_IF_ANY_REJECTED)
            .ok_or(AgenticError::MissingInputMessage(
                format!("No ToolCallMessage for {}[GenericToolNode]", self.name).to_string()
            ))?;

        // filter out tool-specific msgs.
        // TODO: Rust_Macros.md has examples to implement `if conds`` in pattern macros
        let (mut my_msgs, other_tool_msgs):(Vec<_>, Vec<_>) = tool_msgs.drain(..).partition_map(|tm| {
            if tm.tool_call.fn_name == self.name {
                Either::Left(tm)
            } else {
                Either::Right(tm)
            }}
        );

        // warn about unwanted msgs - Is this really needed ?
        // All msgs will be duplicated and sent to all nodes - Wasteful but optimizable.
        // The LLMPrompterNode should, at it's next-superstep error out if all 
        //      tool-calls have not returned. We should not be warning at this level.
        if other_tool_msgs.len() > 0 {
            warn!("{}[GenericToolNode] received msgs:[{:?}] that are not actionable.", self.name, other_tool_msgs);
        }

        // -- Actual processing -----------------------        
        let mut resp = my_msgs.drain(..).map( |tm| {            
            serde_json::from_value::<S>(tm.tool_call.fn_arguments.clone())
            .map_err(|se| 
                AgenticError::ToolCallFunctionArgDeserializationError(format!("{:?}:{:?}", tm.tool_call.fn_arguments, se)))
            .and_then(|args| {
                debug!("Calling Tool Closure on args: {:?}", args);
                (self.tool_func)(args)
            })
            .and_then(|tool_str_response| {
                debug!("Tool: {:?} responded with {:?}", self.name, tool_str_response
            );
                Ok(ToolResponseMsg{                    
                    call_response: ToolResponse::new(&tm.tool_call.call_id, tool_str_response),
                    call_msg: tm,
                }.into())
            })
        })
        .collect_vec();

        // FIXME: Update this this func to send Result<Vec<Message>, Error>
        //        for now, sending just one.
        if resp.len() > 1 {
            Err(AgenticError::Error("Internal LOGIC error. Truncating ToolResponse list. Allow multi".to_string()))?;
        }

        resp.pop().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use tracing::warn;
    use itertools::{Itertools, Either};
    use crate::agentic_msgs::AgenticMessages;

    use super::GenericToolNode;


    #[test]
    // Hello world of tool calling
    fn test_get_weather() {
        
        // create tool
        // to put out dummy output.        

        // call exec with dummy context and approp msg
        // Hmm.. has to be a AgenticMessage enum!! Design Problem
        // What else can it be ? 
        //   impl From<NodeMessage> instead of AgenticMessage enum ?
        //
        // This will simiplify the DX for each node infra significantly
        // at the expense of complxifying the Pregel graph code which has 
        // to handle the transformations and matching in-port/out-port

        // validate output.
    }
}
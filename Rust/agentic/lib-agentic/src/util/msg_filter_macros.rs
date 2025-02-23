
bitflags::bitflags! {
    /// Default is no warnings
    #[derive(Default)]
    pub struct WarnFlags : u8 {        
        const NO_WARNINGS           = 0b00000000;
        const WARN_IF_ANY_REJECTED  = 0b00000001;
        const WARN_IF_MORE_THAN_ONE = 0b00000010;
    }
}


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

/// Variant of `build_block_pattern_partition` which adds warnings
/// when the Msgs are dropped because they fail the filter.
/// 
///- opt_vec     : `Option<Vec<Message>>` _will be consumed_
/// 
///- log_source  : `&str` 
///  - Log source used in warning msgs. 
///  - Warn when
///     - Msgs fail filter
/// 
/// - pattern     : pattern
///   - 👉 See `build_pattern_partition_block` for detais on $pattern and $binding
/// 
/// - warn_flags  : bitwise ORd msg_filter_macros::WarnFlags
///                 WarnFlags | 
/// 
/// Returns: `Option<Message>`
#[macro_export]
macro_rules! build_block_extract_left_partition_warn {
    ($opt_vec:expr, $log_source:expr, $pattern:pat, $binding:ident, $warn_flags:expr) => {
        {
            if let Some(mut vec_vals) = $opt_vec.take() {
                let (wanted, unwanted) = build_block_pattern_partition!(vec_vals, $pattern, $binding);
                
                // Warn if dropping msgs
                if unwanted.len() > 0 && $warn_flags.contains(WarnFlags::WARN_IF_ANY_REJECTED) {
                    warn!("{:?}: Rejected the following messages\n------\n", $log_source);
                    warn!("{:?}\n------\n", unwanted);
                }                
                
                Some(wanted)
            } else{
                None
            }
        }    
    };    
}

/// Variant of `build_block_extract_left_partition_warn` which adds warnings
/// when the Msgs are dropped because they are excessive or if we have 
/// no messages
/// 
///- opt_vec     : `Option<Vec<Message>>` _will be consumed_
/// 
///- log_source  : `&str` 
///  - Log source used in warning msgs. 
///  - Warn when
///     - Msgs fail filter
///     - or pass filter but are dropped since we only pick one msg
/// 
/// Returns: `Option<Message>`
/// 
/// 👉 See `build_pattern_partition_block` for detais on $pattern and $binding
/// 👉 FIXME: Use bitflags to control warnings.
#[macro_export]
macro_rules! build_block_extract_one_left_partition_warn {
    ($opt_vec:expr, $log_source:expr, $pattern:pat, $binding:ident, $warn_flags:expr) => {
        {
            if let Some(mut vec_vals) = $opt_vec.take() {
                let (mut wanted, unwanted) = build_block_pattern_partition!(vec_vals, $pattern, $binding);
                
                // Warn if dropping msgs                
                // 1) Warn if unwanted.len() > 0
                if unwanted.len() > 0 && $warn_flags.contains(WarnFlags::WARN_IF_ANY_REJECTED) {
                    warn!("{:?}: Rejected the following messages\n------\n", $log_source);
                    warn!("{:?}\n------\n", unwanted);
                }

                // 2) Warn if wanted.len() > 1
                if wanted.len() > 1 && $warn_flags.contains(WarnFlags::WARN_IF_MORE_THAN_ONE) {
                    warn!("{:?}: has DROPPED ALL EXCEPT THE LAST of the following messages\n------\n", $log_source);
                    warn!("{:?}\n------\n", wanted);                     
                }

                // Take out the last msg in the list that has been accepted by the filter.
                wanted.pop()
            } else{
                None
            }
        }    
    };    
}


/// Wraps build_block_extract_one_left_partition_warn in a closure.
/// 
/// Closure( opt_msgs: `Option<Vec<MsgType>`, log_source: `&str`)
/// 
/// - 👉 Consumes the incoming msg vector. 
/// - 👉 See `build_pattern_partition_block` for detais on $pattern and $binding
#[macro_export]
macro_rules! build_msg_input_pipe_filter_func_warn {
        
    ($msg_ty:ty, $pattern:pat, $binding:ident, $warn_flags:expr) => {
    
        |mut opt_msgs:Option<Vec<$msg_ty>>, log_source:&str| {            
            build_block_extract_one_left_partition_warn!(opt_msgs, log_source, $pattern, $binding, $warn_flags)
        }    
    };
}

#[cfg(test)]
mod tests {
    use genai::chat::MessageContent;
    use tracing::warn;
    use itertools::{Itertools, Either};
    use crate::agentic_msgs::AgenticMessages;
    use super::WarnFlags;
    use crate::nodes::llm::{LLMPromptMsg, LLMResponseMsg};    
    use assert_matches::*;

    //-- build_block_pattern_partition --------------------------------
    #[test]
    fn test_build_block_pattern_partition_no_msg() {
        let mut msgs = vec![];

        // Build a closure that filters messages which match 
        // AMPrompt(). For now, takes the outer enum and not 
        // the inner LLMPrompt
        let (pass_f, fail_f) = build_block_pattern_partition!(
            msgs,
            outer@AgenticMessages::AMPrompt(_),
            outer);        
        
        assert_eq!(pass_f.len(), 0);
        assert_eq!(fail_f.len(), 0);
    }

    #[test]
    fn test_build_block_pattern_partition_pass_fail_msg() {
        let mut msgs = vec![
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Hello")),
            AgenticMessages::AMPromptResponse(LLMResponseMsg {
                prompt  : LLMPromptMsg::default_system().append_user("Hello"),
                response: "How are you!".to_string(),
            })
        ];

        // Build a closure that filters messages which match 
        // AMPrompt(). For now, takes the outer enum and not 
        // the inner LLMPrompt
        let (pass_f, fail_f) = build_block_pattern_partition!(
            msgs,
            outer@AgenticMessages::AMPrompt(_),
            outer);        
        
        assert_eq!(pass_f.len(), 1);
        assert_matches!(
            match &pass_f[0] {
                AgenticMessages::AMPrompt(LLMPromptMsg{chat_request:r,..}) => r.messages[0].content.clone(),
                _ => "".into()
            },
            MessageContent::Text(txt@_) if txt == "Hello");

        assert_eq!(fail_f.len(), 1);
        assert_eq!(
            match &fail_f[0] {
                AgenticMessages::AMPromptResponse(LLMResponseMsg{prompt:_, response: r}) => r.as_ref(),
                _ => ""
            },
            "How are you!");
    }    

    //---- build_block_extract_one_left_partition_warn --------------
    #[test]
    fn test_build_block_extract_one_left_partition_warn_none() {        
        let mut msgs: Option<Vec<AgenticMessages>> = None;

        // Random prompt. SHouldn't matter with None arg
        let opt_msg = build_block_extract_one_left_partition_warn!(
            msgs,
            "test_build_block_extract_one_left_partition_warn_one_of_many",
            AgenticMessages::AMPrompt(inner @ _),
            inner,
            WarnFlags::NO_WARNINGS);
        
        // Of the three supplied (Hello, Goodbye and World). It should pick 
        // the last one: "World"
        assert!(opt_msg.is_none());
    }    

    #[test]
    fn test_build_block_extract_one_left_partition_warn_empty() {
        let msgs = vec![];
            
        // Build a closure that filters messages which match 
        // AMPrompt(). Take the inner LLMPrompt
        let opt_msg = build_block_extract_one_left_partition_warn!(
            Some(msgs),
            "test_build_block_extract_one_left_partition_warn_one_of_many",
            AgenticMessages::AMPrompt(inner @ _),
            inner,
            WarnFlags::NO_WARNINGS);
                
        assert!(opt_msg.is_none());        
    }    

    #[test]
    fn test_build_block_extract_one_left_partition_warn_one_of_many() {
        let msgs = vec![
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Hello")),
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Goodbye")),
            // 👇 Last of the AMPrompt should be picked
            // if it passes the filter.
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("World")),
            AgenticMessages::AMPromptResponse(LLMResponseMsg {
                prompt: LLMPromptMsg::default_system().append_user("Hello"),
                response: "How are you!".to_string(),
            })
        ];

        // Build a closure that filters messages which match 
        // AMPrompt(). Take the inner LLMPrompt
        let opt_msg = build_block_extract_one_left_partition_warn!(
            Some(msgs),
            "test_build_block_extract_one_left_partition_warn_one_of_many",
            AgenticMessages::AMPrompt(inner @ _),
            inner,
            WarnFlags::NO_WARNINGS);
        
        // Of the three supplied (Hello, Goodbye and World). It should pick 
        // the last one: "World"
        assert!(opt_msg.is_some());        

        let opt_msg = opt_msg.unwrap();
        assert_matches!(
            &opt_msg.chat_request.messages[0].content, 
            MessageContent::Text(txt@_) if txt == "World");
    }    

    #[test]
    fn test_build_block_extract_one_left_partition_warn_one_of_one() {
        let msgs = vec![
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Hello")),
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Goodbye")),
            // 👇 Last of the AMPrompt should be picked
            // if it passes the filter.
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("World")),
            AgenticMessages::AMPromptResponse(LLMResponseMsg {
                prompt: LLMPromptMsg::default_system().append_user("Hello"),
                response: "How are you!".to_string(),
            })
        ];

        // Build a closure that filters messages which match 
        // AMPromptResponse(). Take the inner LLMPromptResponse
        let opt_msg = build_block_extract_one_left_partition_warn!(
            Some(msgs),
            "test_build_block_extract_one_left_partition_warn_one_of_one",
            AgenticMessages::AMPromptResponse(inner @ _),
            inner,
            WarnFlags::NO_WARNINGS);
        
        assert!(opt_msg.is_some());
        assert_eq!(opt_msg.unwrap().response, "How are you!");
    }    


    //-- build_msg_input_pipe_filter_func_warn -----------------
    #[test]
    fn test_pipe_no_msg() {
        let msgs = Some(vec![]);

        // Build a closure that filters messages which match 
        // AMPrompt(). For now, takes the outer enum and not 
        // the inner LLMPrompt
        let func = build_msg_input_pipe_filter_func_warn!(
            AgenticMessages,             
            outer@AgenticMessages::AMPrompt(_),
            outer,
            WarnFlags::NO_WARNINGS);

        let ret = func(msgs, "test_pipe_no_msg");
        
        assert!(ret.is_none());
    }

    // ==== 
    #[test]
    fn test_pipe_one_msg_outer() {
        let msgs = Some(vec![
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Hello"))
        ]);

        // Build a closure that filters messages which match 
        // AMPrompt(). This matches the entire AgenticMessage... value
        // and not the inner LLMPrompt
        let func = build_msg_input_pipe_filter_func_warn!(
            AgenticMessages,             
            outer@AgenticMessages::AMPrompt(_),
            outer,
            WarnFlags::NO_WARNINGS);

        let ret = func(msgs, "test_pipe_one_msg");
        
        // Should work.
        assert!(ret.is_some());

        // Should get the type indicated by `outer@`
        // this is the fully-name spaced version and includes the crate so I'm         
        // using the trailing AgenticMessages
        assert!(std::any::type_name_of_val(&ret).ends_with("::AgenticMessages>"));

        // And picks the only msg.
        assert_matches!(
            match ret.unwrap() {
                AgenticMessages::AMPrompt(LLMPromptMsg{chat_request:r,..}) => r.messages[0].content.clone(),
                _ => "".into()
            },
            MessageContent::Text(txt@_) if txt == "Hello");        
    }

    #[test]
    fn test_pipe_one_msg_inner() {
        let msgs = Some(vec![
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Hello"))
        ]);

        // Build a closure that filters messages which match 
        // AgenticMessage::AMPrompt() and returns the wrapped LLMPrompt        
        let func = build_msg_input_pipe_filter_func_warn!(
            AgenticMessages,             
            AgenticMessages::AMPrompt(inner @ _),
            inner,
            WarnFlags::NO_WARNINGS);

        let ret = func(msgs, "test_pipe_one_msg");
        
        // Should work.
        assert!(ret.is_some());

        // Should get the type indicated by `inner@`
        // this is the fully-name spaced version and includes the crate so I'm 
        // just matching the trailing portion that shows that the inner LLMPrompt is being
        // used and not the outer AgenticMessages::AMPrompt(..)
        // ret is an Option, hence will be wrapped and hence `LLMPrompt>`
        println!("{:?}", std::any::type_name_of_val(&ret));
        assert!(std::any::type_name_of_val(&ret).ends_with("llm::llm_prompt_msg::LLMPromptMsg>"));

        // And picks the only msg.        
        let prompt_msg = ret.unwrap();
        assert_matches!(
            &prompt_msg.chat_request.messages[0].content, 
            MessageContent::Text(txt@_) if txt == "Hello");
    }

    #[test]
    fn test_pipe_extra_msgs() {
        let msgs = Some(vec![           
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Hello")),
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Goodbye"))
        ]);

        // Build a closure that filters messages which match 
        // AMPrompt(). For now, takes the outer enum and not 
        // the inner LLMPrompt
        let func = build_msg_input_pipe_filter_func_warn!(
            AgenticMessages,             
            AgenticMessages::AMPrompt(inner@_),
            inner,
            WarnFlags::NO_WARNINGS);

        let ret = func(msgs, "test_pipe_extra_msgs");
        
        // Should work
        assert!(ret.is_some());

        // And pick the last msg sent.
        let prompt_msg = ret.unwrap();
        assert_matches!(
            &prompt_msg.chat_request.messages[0].content, 
            MessageContent::Text(txt@_) if txt == "Goodbye");
    }
}
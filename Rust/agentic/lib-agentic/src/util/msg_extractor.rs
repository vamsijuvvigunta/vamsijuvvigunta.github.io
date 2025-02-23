use std::marker::PhantomData;
use crate::agentic_errors::AgenticError;

//------------------------------------------------------------------------
// Note: while all this works, it is not clear where it'll get used.
//  - Can only declare it as a field when TMsg, EXtract, TErr are known
//  - If TMsg, EXtract, TErr are known only in some trait impl, then 
//    an instance of MsgExtractor cannot be stored in the struct without 
//    also making it generic on these TMsg, EXtract, TErr.
//  - Most cases, look at the tests and impl for this and simply use the 
//    Macros from msg_filter_macros.rs to generate code into your 
//    trait impls. 
//
//  Import both of these to use build_msg_input_pipe_filter_func_warn
//      use crate::build_msg_input_pipe_filter_func_warn;
//      use crate::build_pattern_partition_block;
//-------------------------------------------------------------------------
pub struct MsgExtractor<F, TMsg, TExtract, TErr>
where
    F: Fn(Option<Vec<TMsg>>, // will be consumed
          &str,                 // Identifier to use in warnings
        ) -> Option<TExtract>,
     TErr : From<AgenticError>     
{
    msg_receiver_name: String,
    filter_func    : F,

    msg_type       : PhantomData<TMsg>,
    err_type       : PhantomData<TErr>
}


impl<F, TMsg, TExtract, TErr> MsgExtractor<F, TMsg, TExtract, TErr>
where
    F: Fn(Option<Vec<TMsg>>, // will be consumed
        &str,                // Identifier to use in warnings
        ) -> Option<TExtract>,
    TErr : From<AgenticError>
{    
    pub fn new(msg_receiver_name: impl Into<String>,
               func: F) -> Self 
    {
        MsgExtractor {
            msg_receiver_name : msg_receiver_name.into(),
            filter_func    : func,
            msg_type       : PhantomData,
            err_type       : PhantomData
        }
    }

    /// Type of `in_msgs` matches the PregelNode trait's 
    /// `exec` method.
    pub fn extract_msg(&self, in_msgs: Option<Vec<TMsg>>) -> std::result::Result<TExtract, TErr> {
        (self.filter_func)(
            in_msgs,
            &self.msg_receiver_name
        )          
        .ok_or(AgenticError::MissingInputMessage(
            "No InputMessage for {self.msg_receiver_name}".to_string()).into()
        )
    }
}

//---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use itertools::{Either, Itertools};
    use tracing::warn;
    use crate::agentic_msgs::AgenticMessages;
    use crate::agentic_errors::AgenticError;
    use crate::nodes::llm::one_shot_chat_node::LLMPrompt;
    use crate::util::msg_extractor::MsgExtractor;

    use crate::util::macros_prelude::*;    

    #[allow(non_snake_case)]
    fn matches_AgenticMessages_prompt_text(msg: &AgenticMessages, expect_str: &str) -> bool {
        match msg {
            AgenticMessages::AMPrompt(LLMPrompt{prompt: p}) if p == expect_str => true,
            _ => false
        }
    }

    #[allow(non_snake_case)]
    fn matches_LLMPrompt_prompt_text(msg: &LLMPrompt, expect_str: &str) -> bool {        
        match msg {
            LLMPrompt{prompt: p} if p == expect_str => true,
            _ => false
        }
    }    

    #[test]
    fn msg_extract_no_msg() {
        let msgs = Some(vec![]);

        // extractor a single AgenticMessages::AMPrompt msg from Vec<AgenticMessage>
        let extractor = MsgExtractor::new(
                "msg_input_pipe_no_msg",            
                build_msg_input_pipe_filter_func_warn!(
                    AgenticMessages,             
                    outer@AgenticMessages::AMPrompt(_),
                    outer));
        
        let ret = extractor.extract_msg(msgs);        
        assert!(match ret {
            Err(AgenticError::MissingInputMessage(_)) => true,
            _ => false
        });
    }

    // Test that we extract the following
    //   AgenticMessages::* → AgenticMessages::AMPrompt(..)
    //
    // i.e, test out outer extraction where it gets the entire pattern match.
    #[test]
    fn msg_extract_one_msg_outer() {
        let msgs = Some(vec![
            AgenticMessages::AMPrompt(LLMPrompt{prompt: "Hello".to_string()})
        ]);

        // AgenticMessages::AMPrompt(_) ← Vec<AgenticMessage>
        let extractor = MsgExtractor::<_, _, _, AgenticError>::new(
            "msg_extract_one_msg_outer",            
            build_msg_input_pipe_filter_func_warn!(
                AgenticMessages,             
                outer@AgenticMessages::AMPrompt(_),
                outer));
    
        let ret = extractor.extract_msg(msgs); 
        
        // Should work.
        assert!(ret.is_ok());          

        // Pattern extracts outer which should be an AgenticMessages variant      
        let msg = ret.unwrap();        
        assert!(std::any::type_name_of_val(&msg).ends_with("::AgenticMessages"));        

        // And picks the only msg.
        assert!( matches_AgenticMessages_prompt_text(&msg, "Hello"));
    }

    // Test that we extract p where:
    //   AgenticMessages::* → AgenticMessages::AMPrompt( p @ LLMPrompt)
    //
    // i.e, test out inner extraction where it gets a value bound to some 
    // inner portion of the match
    #[test]
    fn test_pipe_one_msg_inner() {
        let msgs = Some(vec![
            AgenticMessages::AMPrompt(LLMPrompt{prompt: "Hello".to_string()})
        ]);

        // LLMPrompt ← AgenticMessages::AMPrompt(LLMPrompt) ← Vec<AgenticMessage>        
        let extractor = MsgExtractor::<_, _, _, AgenticError>::new(
            "msg_extract_one_msg_outer",            
            build_msg_input_pipe_filter_func_warn!(
                AgenticMessages,             
                AgenticMessages::AMPrompt(inner @ _),
                inner));
    
        let ret = extractor.extract_msg(msgs); 
        
        // Should work.
        assert!(ret.is_ok());

        // Should get the type indicated by `inner@`        
        // just matching the trailing portion of the fq type that shows that the 
        // inner LLMPrompt is being used and not the outer AgenticMessages::AMPrompt(..)        
        let msg = ret.unwrap();
        assert!(std::any::type_name_of_val(&msg).ends_with("llm::one_shot_chat_node::LLMPrompt"));

        // And picks the only msg.
        assert!( matches_LLMPrompt_prompt_text(&msg, "Hello"));
    }
}    
use derive_more::derive::From;
use itertools::{Itertools, Either};
use crate::nodes::tool::{ToolCallMsg, ToolResponseMsg};
use crate::nodes::llm::{LLMPromptMsg, LLMResponseMsg};
use crate::util::macros_prelude::*;

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
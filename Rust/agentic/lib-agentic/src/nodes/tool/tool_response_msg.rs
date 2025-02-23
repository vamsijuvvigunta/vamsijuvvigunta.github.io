use genai::chat::ToolResponse;
use crate::nodes::{llm::LLMPromptMsg, tool::ToolCallMsg};

#[derive(Debug,Clone)]
pub struct ToolResponseMsg {
    // Mainly for the tool_call_id
    pub call_msg : ToolCallMsg,

    // just a string but can be stringified json in the most generic case.
    pub call_response: ToolResponse
}

impl ToolResponseMsg {
    /// consumes this msg and it's underlying and returns the updated prompt
    pub fn into_prompt(self) -> LLMPromptMsg {
        // Will be packed as an Assistant type        
        self.call_msg.prompt.append_tool_call(
            self.call_msg.tool_call
        )
        // Will be packed as a Tool type
        .append_tool_response(
            self.call_response
        )
    }
}
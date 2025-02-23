use genai::chat::ToolCall;
use crate::nodes::llm::LLMPromptMsg;

//------------------------------------------------------------
// OpenAI/Groq tool-call response JSON
//  - "arguments" are stringified json
//-------------------------------------------------------------
// "tool_calls": [
// 	{
// 	  "function": {
// 		"arguments": "{\"format\":\"fahrenheit\",\"location\":\"San Jose, CA\"}",
// 		"name": "get_current_weather"
// 	  },
// 	  "id": "call_Vu0c1G8RZMFxebzkQfa7V8VJ",
// 	  "type": "function"
// 	}
//-------------------------------------------------------------
// TODO: Redundant! Remove this completely and consider either using 
// ToolCallMessage { ToolCall, LLMPrompt}
#[derive(Debug,Clone)]
pub struct ToolCallMsg {
    // Included in prompt. Included for 
    // validation on executor side.    
    pub prompt : LLMPromptMsg,

    // tool_call req from genai
    pub tool_call : ToolCall,    
}

impl ToolCallMsg {
    pub fn from(tool_call:ToolCall, prompt: LLMPromptMsg) -> Self {
        ToolCallMsg{
            prompt : prompt,
            tool_call : tool_call,
        }
    }    
}

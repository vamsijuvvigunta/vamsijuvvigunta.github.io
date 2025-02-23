use super::llm_prompt_msg::LLMPromptMsg;

#[derive(Debug, Clone)]
pub struct LLMResponseMsg {    
    pub prompt  : LLMPromptMsg,

    // When we get into MultiPart, this can change.
    pub response: String,

    // When genai finalizer their MetaUsage use, can include 
    // that field here.
}

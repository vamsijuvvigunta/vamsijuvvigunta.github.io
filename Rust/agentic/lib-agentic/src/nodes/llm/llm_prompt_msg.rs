use genai::chat::{ChatMessage, ChatRequest, MessageContent, Tool, ToolCall, ToolResponse};

#[derive(Debug, Clone)]
pub struct LLMPromptMsg {    
    pub chat_request  : ChatRequest,
    pub model_override: Option<&'static str>,
}

/// Note that various From conversions exist for Into<MessageContent>
/// see genai::chat::message_content.rs
///   String       → 
///   &String      → 
///   &str         → 
///   ToolResponse →
///   Multi-Part is coming..
/// 
/// Start from an existing CharRequest (history or new)
/// or system prompt.
/// 
/// Keep adding user/assistant/tool-call/tool-response messages
impl LLMPromptMsg {
    /// Build the ChatRequest fist and then add it here.
    pub fn from(chat_req: ChatRequest) -> Self {
        LLMPromptMsg {
            chat_request : chat_req,
            model_override: None
        }
    }    

    pub fn from_system(system: impl Into<String>) -> Self {
        LLMPromptMsg {
            chat_request : ChatRequest::from_system(system),
            model_override : None
        }
    }

    pub fn default_system() -> Self {
        Self::from_system("You are a helpful assistant")
    }
    
    pub fn append_user(mut self, content: impl Into<MessageContent>) -> Self {
        self.chat_request = self.chat_request.append_message(ChatMessage::user(content));
        self
    }

    pub fn append_assistant(mut self, content: impl Into<MessageContent>) -> Self {
        self.chat_request = self.chat_request.append_message(ChatMessage::assistant(content));
        self
    }    

    pub fn append_tool(mut self, tool: impl Into<Tool>) -> Self {
		self.chat_request = self.chat_request.append_tool(tool);
		self
	}

    pub fn append_tool_call(mut self, msg: ToolCall) -> Self {
        self.chat_request = self.chat_request.append_message(vec![msg]);
        self
    }

    pub fn append_tool_calls(mut self, msg: Vec<ToolCall>) -> Self {
        self.chat_request = self.chat_request.append_message(msg);
        self
    }

    pub fn append_tool_response(mut self, msg: ToolResponse) -> Self {
        self.chat_request = self.chat_request.append_message(msg);
        self
    }

    /// Replaces tool list with the supplied 
    pub fn with_tools(mut self, tools: Vec<Tool>) -> Self {
		self.chat_request = self.chat_request.with_tools(tools);
		self
	}

    /// Replaced model_override with the supplied
    pub fn with_model_override(mut self, model: impl Into<&'static str>) -> Self {
        self.model_override = Some(model.into());
        self
    }
}


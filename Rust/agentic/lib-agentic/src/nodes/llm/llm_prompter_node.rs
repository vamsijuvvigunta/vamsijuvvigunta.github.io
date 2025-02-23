use async_trait::async_trait;
use std::{result::Result, sync::Arc};
use tokio::sync::Mutex;
use tracing::{debug, error, trace};

use lib_pregel::pregel_types::{PregelContext, PregelNode};

use crate::agentic_errors::AgenticError;
use crate::agentic_msgs::AgenticMessages;
use crate::nodes::tool::{ToolCallMsg, ToolResponseMsg};
use crate::util::msg_filter::MsgSource;

use genai::chat::{ChatResponse, MessageContent, ToolCall};
use genai::Client;

use crate::nodes::llm::{LLMPromptMsg, LLMResponseMsg};

// Groq
const DEFAULT_MODEL: &str = "llama3-70b-8192";

// 👉 READ THIS: https://users.rust-lang.org/t/best-design-for-fault-tolerant-concurrent-async-applications/98072
#[derive(Debug)]
pub struct LLMPrompterNode {    
    // Pregel node fields
    pub name: String,
    pub description: String,

    // Prompter fileds ---
    llm_default_model : &'static str,
    llm_client: Client,
}

impl LLMPrompterNode {
    pub fn default() -> Self {
        LLMPrompterNode {
            name: "LLMPrompterNode".to_string(),
            description: "Send a single prompt to an LLM and returns the response".to_string(),
            llm_client: Client::default(),            
            llm_default_model : DEFAULT_MODEL,
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

    pub fn with_model(mut self, model: impl Into<&'static str>) -> Self {
        self.llm_default_model = model.into();
        self
    }    

    async fn get_chat_response(&self, llm_prompt: LLMPromptMsg) -> Result<AgenticMessages, AgenticError> {            

        // -- Process the prompt via the end-point--
        // Override model from prompt or choose default
        let model = llm_prompt.model_override
            .as_ref()
            .unwrap_or(&self.llm_default_model);

        // see https://github.com/jeremychone/rust-genai/blob/HEAD/examples/c00-readme.rs
        // for examples
        let chat_response = self.llm_client
            .exec_chat(model, llm_prompt.chat_request.clone(), None)
            .await?;            

        self._do_process_chat_response(llm_prompt, chat_response)        
    }

    fn _do_process_chat_response(&self, 
        llm_prompt: LLMPromptMsg,
        chat_response: ChatResponse) -> Result<AgenticMessages, AgenticError> {
                        
        // The API ergonomics is weird. There are two methods to consume self
        // which cannot be used in if/else as the if could have consumed self.
        // Using lower level methods
        match chat_response.content {            
            Some(MessageContent::Text(text @ _)) => {
                Ok(LLMResponseMsg {
                    prompt: llm_prompt,
                    response: text,
                }
                .into())
            },
            Some(MessageContent::ToolCalls(tool_calls @ _)) => {
                self._do_process_chat_response_toolcalls(llm_prompt, tool_calls)                
            },
            Some(unknown @ _) => Err(
                AgenticError::ChatResponseNeitherTextNotToolCall(format!("{:?}", unknown))
            ),
            None => Err(
                AgenticError::ChatResponseIsMissing
            )
        }        
    }    

    fn _do_process_chat_response_toolcalls(&self,         
        prompt: LLMPromptMsg,        
        mut tool_reqs: Vec<ToolCall>) -> Result<AgenticMessages, AgenticError> {

            // Temp limitation
            if tool_reqs.len() > 1 {                
                Err(AgenticError::Error("Can only handle 1 tool call in chat response for now!".to_string()))?
            }

            let tool_call_req = tool_reqs
                .pop()
                .inspect(|tc| debug!("Processing ToolCall: {:?}", tc))
                .expect("Expecting a ToolCall");

            Ok(ToolCallMsg::from(tool_call_req, prompt).into())
    }
}

// Replace with a derive-macro ----------------
#[async_trait]
impl PregelNode for LLMPrompterNode {
    type Error = AgenticError;
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
        in_msgs: Option<Vec<Self::Message>>,
    ) -> Result<Self::Message, Self::Error> {
        debug!("Exec called on LLMPrompterNode");                

        let (mut prompts, mut tool_responses) = in_msgs.ok_or(
            AgenticError::MissingInputMessage("LLMPrompterNode needs input messages".to_string())
        ).and_then(|msgs| {

            let mut prompts        =  Vec::<LLMPromptMsg>::new();
            let mut tool_responses =  Vec::<ToolResponseMsg>::new();

            let _remaining = MsgSource::from(msgs)
                .split_take(
                    &mut prompts, 
                    |mv| AgenticMessages::func_take_llmprompt_msgs(mv))
                .split_take(
                    &mut tool_responses, 
                    |mv| AgenticMessages::func_take_toolresponse_msgs(mv));

            // TODO: Warn if remaining has any
            Ok((prompts, tool_responses))
        })?;        

        // --- Process messages ----------
        if tool_responses.len() > 0 {
            // TODO: Error out if prompts.len() > 0
            debug!("Processing {} toolResponses:", tool_responses.len());

            // FIXME: Handle more than one tool-response
            let tool_response = tool_responses.pop().unwrap();

            // Consume the tool_response and call and add then back into the prompt                        
            debug!("LLMPrompterNode is processing tool_response: {tool_response:?}");
            let prompt = tool_response.into_prompt();            
            debug!("LLMPrompterNode is continuing with prompt= {prompt:?}");

            self.get_chat_response(prompt).await
        }
        else if prompts.len() > 0 {
            let prompt = prompts.pop().unwrap();
            trace!("LLMPrompterNode is processing prompt: {prompt:?}");
            self.get_chat_response(prompt).await
        }
        else {            
            Err(AgenticError::MissingInputMessage("LLMPrompterNode needs has no Prompt Messages".to_string()))
        }        
    }
}


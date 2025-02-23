//-----------------------------------------------------------------------
// See ./joke_gen_and_critique.md for the details of the LlamaIndex
// example I am reproducing.
//-----------------------------------------------------------------------
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use lib_pregel::pregel_driver::PregelDriver;
use lib_pregel::pregel_types::PregelAlgorithm;

use lib_agentic::agentic_errors::{AgenticError, AgenticResult};
use lib_agentic::agentic_msgs::AgenticMessages;

use lib_agentic::nodes::llm::{LLMPromptMsg, LLMResponseMsg, LLMPrompterNode};
use lib_agentic::nodes::wrap::msg_transform_wrapper_node::MsgTransformWrapperNode;

// Brings pregel trait's associated types into current module for easy use
type PregelImpl = PregelDriver::<AgenticMessages, AgenticError>;
type NodeIndex  = <PregelImpl as PregelAlgorithm>::NodeIndex;

// argument reading
use std::env;

#[tokio::main]
async fn main() -> AgenticResult<()> {

    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug")))
        .init();

    // read args
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    // Hardcode topic for now, get it from args later.
    run_pregel("sailors").await?;

    Ok(())
}

// See ./joke_gen_and_critique.md for the details. 
// This Node handles the generate_joke step
//
// @step
// async def generate_joke(self, ev: StartEvent) -> JokeEvent:
//     topic = ev.topic
// 
//     prompt = f"Write your best joke about {topic}."
//     response = await self.llm.acomplete(prompt)
//     return JokeEvent(joke=str(response))
//-----------------------------------------------------------
fn add_joke_gen_node(pg: &mut PregelImpl) -> NodeIndex {        

    let joke_gen_node = Box::new(
        MsgTransformWrapperNode::new(
            "JokeGen", 
            "Asks an LLM to Generate a joke", 
            // We use the standard LLMPrompterNode to do the actual LLM prompting
            Box::new(LLMPrompterNode::default()),

            // Then transform it's output into a prompt the connected node that will handle 
            // the prompt
            // @step
            // async def critique_joke(self, ev: JokeEvent) -> StopEvent:
            //     joke = ev.joke
            // 
            //     prompt = f"Give a thorough analysis and critique of the following joke: {joke}"
            //     response = await self.llm.acomplete(prompt)
            //     return StopEvent(result=str(response))
            |llm_response| match llm_response {                
                AgenticMessages::AMPromptResponse(LLMResponseMsg{prompt:_, response:r}) => {
                    Ok(
                        AgenticMessages::AMPrompt(
                            LLMPromptMsg::default_system()
                            .append_user(format!(
                                "Give a thorough analysis and critique of the following joke: {r}"
                            ))
                    ))
                },
                p@_ => Err(AgenticError::MsgTransformGotUnexpectedMessage { 
                        expected: "AgenticMessages::AMPromptResponse(LLMPromptResponse{..}".to_string(), 
                        got: format!("{:?}", p) })
            })
    );

    pg.add_node(joke_gen_node)
}

async fn run_pregel(joke_topic: &str) -> AgenticResult<()> {
    //-- create graph        
    let mut pg = PregelDriver::<AgenticMessages, AgenticError>::new();

    // create the nodes
    let joke_gen_id = add_joke_gen_node(&mut pg);    
    let joke_critique_id = pg.add_node(Box::new(LLMPrompterNode::default()));

    // and the edge
    pg.add_edge(joke_gen_id, joke_critique_id);
    
    
    // Set up initial message. This takes care of the joke generation 
    // prompting    
    //
    // @step
    // async def generate_joke(self, ev: StartEvent) -> JokeEvent:
    //     topic = ev.topic
    // 
    //     prompt = f"Write your best joke about {topic}."
    //     response = await self.llm.acomplete(prompt)
    //     return JokeEvent(joke=str(response))
    pg.send_message(
        joke_gen_id, 
        LLMPromptMsg::default_system().append_user(format!(
            "Write your best joke about {joke_topic}"
        )).into()        
    )?;

    //-- Process it        
    match pg.run_pregel(3).await {
        Ok(res) => {
            info!("Result is {res:?}");
            Ok(())
        },
        Err(e) => {
            error!("Pregel Error: {e:?}");
            Err(e)
        }
    }
}

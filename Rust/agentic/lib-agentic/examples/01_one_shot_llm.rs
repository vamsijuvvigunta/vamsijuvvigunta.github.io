
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use lib_pregel::pregel_driver::PregelDriver;
use lib_pregel::pregel_types::PregelAlgorithm;

use lib_agentic::agentic_errors::{AgenticError, AgenticResult};
use lib_agentic::agentic_msgs::AgenticMessages;
use lib_agentic::nodes::llm::{LLMPromptMsg, LLMPrompterNode};


#[tokio::main]
async fn main() -> AgenticResult<()> {

    tracing_subscriber::fmt()
        .without_time() // For early local development.
        .with_target(false)
        .with_env_filter(
            EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("debug")))
        .init();

    run_pregel().await?;

    Ok(())
}

async fn run_pregel() -> AgenticResult<()> {
    //-- create graph        
    let mut pg = PregelDriver::<AgenticMessages, AgenticError>::new();

    // create the nodes
    let llm_id = pg.add_node(
        Box::new(
            LLMPrompterNode::default()
        ));
    
    pg.send_message(
        llm_id, 
        LLMPromptMsg::from_system("You are a helpful assistant")
            .append_user("Why is the sky blue")
            .into()
    )?;

    //-- Process it        
    match pg.run_pregel(1).await {
        Ok(res) => {
            info!("Success: Result is {res:?}");
            Ok(())
        },
        Err(e) => {
            error!("Failure: Pregel Error: {e:?}");
            Err(e)
        }
    }
}

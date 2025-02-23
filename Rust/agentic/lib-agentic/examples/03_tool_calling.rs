use genai::chat::Tool;

use lib_agentic::nodes::tool::generic_tool_node::GenericToolNode;
//-----------------------------------------------------------------------
// See ./03_tool_calling.rs for the details of the analysis done to 
// add tool calling nodes and their DX.
//-----------------------------------------------------------------------
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

use lib_pregel::pregel_driver::PregelDriver;
use lib_pregel::pregel_types::PregelAlgorithm;

use lib_agentic::agentic_errors::{AgenticError, AgenticResult};
use lib_agentic::agentic_msgs::AgenticMessages;

use lib_agentic::nodes::llm::{LLMPromptMsg, LLMPrompterNode};
use lib_agentic::nodes::tool::schema_for_struct_param;

// argument reading
use std::env;

//===============================================================================
// Module for example tool structs 
//===============================================================================
#[allow(dead_code)]
mod example {    
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};    

    // From OpenAI example. The "hello world" of tool calling
    #[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
    pub enum TemperatureUnits {
        Celcius,
        Farenheit
    }

    #[derive(Debug, Clone, Deserialize, JsonSchema)]
    pub struct GetCurrentWeatherParams {    
        /// The city and state, e.g. San Francisco, CA
        pub location: String,

        /// The temperature unit to use. Infer this from the users location.
        pub units : TemperatureUnits,
    }       
}

use example::GetCurrentWeatherParams;
use example::TemperatureUnits;
//==============================================================================

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

    // run pregel
    run_pregel().await?;

    Ok(())
}

async fn run_pregel() -> AgenticResult<()> {

    //-- create graph        
    let mut pg = PregelDriver::<AgenticMessages, AgenticError>::new();

    let llm_id = pg.add_node(
        Box::new(
            LLMPrompterNode::default()
        ));    

    let gw_tool_id = pg.add_node(
        Box::new(
            GenericToolNode::new(|param: GetCurrentWeatherParams| {
                Ok(
                    match param.units {
                        TemperatureUnits::Celcius => "25",
                        TemperatureUnits::Farenheit => "72",
                    }.to_string()
                )                
            })
            .with_name("get_weather")
            .with_description("Function that returns the weather at a specified location and temperature unit")
        ));

    // Prompter → Tool : ToolCall
    pg.add_edge(llm_id, gw_tool_id);

    // Tool → Prompter : ToolResponse
    pg.add_edge(gw_tool_id, llm_id);
        
    //-- Initial message.
    // groq has a few models with function-calling support and that too in-progress
    // use: gpt-4o-mini by default for tool-calls.
    //
    // Should there be automatic tool discovery over the graph ? We do that and 
    // then allow the prompt to slect any subset of the tools. Prompt will pull in
    // tool-list via the Ctxt ?
    let tooled_prompt = LLMPromptMsg
        ::from_system("You are a helpful assistant that uses the supplied tools when necessary")
        .with_model_override("gpt-4o-mini")
        .append_user("What is the temperature in C, in Paris")
        .append_tool(
            Tool::new("get_weather")
            .with_description("Function that returns the weather for a location in the specified units of C or F")
            .with_schema(schema_for_struct_param::<GetCurrentWeatherParams>()));
    
    pg.send_message(llm_id, tooled_prompt.into())?;

    //-- Process graph --
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

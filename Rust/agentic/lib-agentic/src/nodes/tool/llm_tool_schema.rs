//-------------------------------------------------------------------------
// See the development of this documented in the following PR
// https://github.com/vamsi-juvvi/rust-genai/blob/function_calling_openai/docs/add-function-calling/0-AddingFunctionCallingToGenAI.md
//-------------------------------------------------------------------------

use crate::util::value_ext::ValueExt;

use schemars::JsonSchema;
use schemars::gen::{SchemaGenerator, SchemaSettings};

use serde_json::{json, Value};
use tracing::trace;

/// Generate JSON schema for a tool function usable in LLM Chats
/// The function is of the following form
///    $fn_name()
pub fn schema_for_fn_no_param(fn_name: &str, fn_desc: &str) -> Value
{    
    // Groq does not allow `"parameters" : Value::Null,`
    // while OpenAI is ok with it.
    json!({
        "type": "function",
        "function" : {
            "name" : fn_name,
            "description" : fn_desc,
            //"parameters" : Value::Null,
            },        
    })
}

/// The schemars schema-generator, by default, wants to 
/// seprate the definitions for enums. Using the weather example
/// in the OpenAI cookbook, this looks like this.
/// 
/// "definitions": {
///  "TemperatureUnits": {
///      "enum": [
///        "Celcius",
///        "Farenheit"
///      ],
///      "type": "string"
///    }
///  },
///  "properties": {
///    "format": {
///      "allOf": [
///        {
///          "$ref": "#/definitions/TemperatureUnits"
///        }
///      ],
///      "description": "The temperature unit to use. Infer this from the users location."
///    },  
///  },
/// 
/// Customizing it to inline the so-called sub-schemas results in 
/// the following form which OpenAI accepts.
/// 
///  "properties": {
///    "format": {
///        "description": "The temperature unit to use. Infer this from the users location.",      
///         "enum": [ "Celcius", "Farenheit"],
///        "type": "string"
///      },
pub(crate) fn schema_generator_for_tool() -> SchemaGenerator {
    let settings = SchemaSettings::default().with(|s| {  
        // ⚠️ Crucial setting to allow OpenAI to recognize our schemas.
        s.inline_subschemas = true;
    });

    settings.into_generator()
}

/// Generate JSON schema of a struct. This is meant for a secnario where the
/// struct is used as a function call paremeter in JSON format.
/// The function is of the following form.
///    /// $fn_desc
///    $fn_name(param:TParam)
/// 
/// The single param must derive from JsonSchema like
/// 
/// use schemars::JsonSchema;
/// #[derive(JsonSchema)]
/// struct MyParam {..}
/// 
/// Each field should have it's own `/// doc` which has to be meaningful 
/// to the LLM and will be included in the schema as `description:`
pub fn schema_for_struct<TParam>() -> Value
where 
    TParam : JsonSchema
{
    let gen = schema_generator_for_tool();    
    let param_schema = gen.into_root_schema_for::<TParam>();
    serde_json::to_value(param_schema).unwrap_or_default()    
}

pub fn schema_for_struct_param<TParam>() -> Value
where 
    TParam : JsonSchema
{
    let mut param_schema_json = schema_for_struct::<TParam>();
    json!({
        "type" : "object",
        "properties" : param_schema_json.x_take("/properties").unwrap_or(Value::Null),
        "required" : param_schema_json.x_take::<Value>("/required").unwrap_or(Value::Null)
    })
}

/// Generate JSON schema for a tool function usable in LLM Chats
/// The function is of the following form.
///    /// $fn_desc
///    $fn_name(param:TParam)
/// 
/// The single param must derive from JsonSchema like
/// 
/// use schemars::JsonSchema;
/// #[derive(JsonSchema)]
/// struct MyParam {..}
/// 
/// Each field should have it's own `/// doc` which has to be meaningful 
/// to the LLM and will be included in the schema as `description:`
pub fn schema_for_fn_single_param<TParam>(fn_name: &str, fn_desc: &str) -> Value
where 
    TParam : JsonSchema
{    
    // Generate schema of the struct itself    
    let param_schema_json = schema_for_struct_param::<TParam>();    
    trace!("{:<12} - Schemars based param schema for {}\n{}", 
        "schema_for_fn_single_param",
        std::any::type_name::<TParam>(),
        serde_json::to_string_pretty(&param_schema_json).unwrap());        

    // Insert the struct's schema into the required function schema as if the 
    // function were taking an instance of the struct.    
    let mut tool_schema_json = schema_for_fn_no_param(fn_name, fn_desc);

    tool_schema_json.x_insert(
        "/function/parameters", 
        param_schema_json).unwrap();        

    tool_schema_json
}


//========================================================================================
#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use super::schema_for_fn_single_param;

    // From OpenAI example. The "hello world" of tool calling
    #[derive(Debug, Serialize, Deserialize, JsonSchema)]
    pub enum TemperatureUnits {
        Celcius,
        Farenheit
    }

    #[derive(Debug, Deserialize, JsonSchema)]
    pub struct GetCurrentWeatherParams {    
        /// The city and state, e.g. San Francisco, CA
        pub location: String,

        /// The temperature unit to use. Infer this from the users location.
        pub format : TemperatureUnits,
    }

    #[test]
    fn test_get_weather() {        
        let tool_schema = schema_for_fn_single_param::<GetCurrentWeatherParams>(
            "get_current_weather", 
            "Get the current weather");

        // Generated JSON includes the field documentation
        // I had manually printed this out, verified it and then including it here 
        // for regression testing.    
        assert_eq!(
            tool_schema,
            json!({
                "function": {
                  "description": "Get the current weather",
                  "name": "get_current_weather",
                  "parameters": {
                    "properties": {
                      "format": {
                        "description": "The temperature unit to use. Infer this from the users location.",
                        "enum": [
                          "Celcius",
                          "Farenheit"
                        ],
                        "type": "string"
                      },
                      "location": {
                        "description": "The city and state, e.g. San Francisco, CA",
                        "type": "string"
                      }
                    },
                    "required": [
                      "format",
                      "location"
                    ],
                    "type": "object"
                  }
                },
                "type": "function"
              })
        )
    }
}
use derive_more::derive::From;
use lib_pregel::pregel_types::PregelAlgorithmError;

pub type AgenticResult<T> = core::result::Result<T, AgenticError>;

#[derive(Debug, From)]
pub enum AgenticError {
    Error(String),
	MissingInputMessage(String),
	NestedNodeExecFailed(String),
	MsgTransformGotUnexpectedMessage{expected: String, got: String},

	// chat
	ChatResponseIsMissing,
	ChatResponseNeitherTextNotToolCall(String),

	// tool calls
	ToolCallFunctionArgDeserializationError(String),
	ToolCallFailed(String),
	TollCallMessageMissing(String),

    // Pregel Algorithm error
    #[from]
    Pregel(PregelAlgorithmError),

	// 3rd party: genai error
	#[from]
	GenAI(genai::Error),

	#[from]
	Serde(serde_json::Error)
}

// region:    --- Error Boilerplate
impl core::fmt::Display for AgenticError {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for AgenticError {}
// endregion: --- Error Boilerplate
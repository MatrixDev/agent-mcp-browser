use rmcp::ErrorData;
use rmcp::model::{CallToolResult, ContentBlock};

pub type ToolResult = Result<CallToolResult, ErrorData>;

////////////////////////////////////////////////////////////////////////////////
pub fn failure(error: anyhow::Error) -> ErrorData {
    ErrorData::internal_error(error.to_string(), None)
}

////////////////////////////////////////////////////////////////////////////////
pub fn success(message: impl Into<String>) -> ToolResult {
    Ok(CallToolResult::success(vec![ContentBlock::text(message)]))
}

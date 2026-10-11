use crate::mcp::tool_result::{ToolResult, success};
use schemars::JsonSchema;
use serde::Deserialize;
use std::time::Duration;

////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, JsonSchema)]
pub struct WaitTool {
    /// Milliseconds to wait.
    pub ms: u64,
}

impl WaitTool {
    ////////////////////////////////////////////////////////////////////////////////
    pub async fn exec(self) -> ToolResult {
        tokio::time::sleep(Duration::from_millis(self.ms)).await;
        success(format!("waited {}ms", self.ms))
    }
}

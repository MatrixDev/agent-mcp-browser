use crate::browser::ChromeTab;
use crate::mcp::tool_result::{ToolResult, failure};
use rmcp::model::{CallToolResult, ContentBlock};
use schemars::JsonSchema;
use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, JsonSchema)]
pub struct ScreenshotTool {
    /// Tab id as returned by `list_tabs`.
    pub tab: String,
}

impl ScreenshotTool {
    ////////////////////////////////////////////////////////////////////////////////
    pub async fn exec(self, tab: &ChromeTab) -> ToolResult {
        let png = tab.screenshot().await.map_err(failure)?;

        let image = ContentBlock::image(png, "image/png");
        Ok(CallToolResult::success(vec![image]))
    }
}

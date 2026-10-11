use crate::browser::ChromeTab;
use crate::mcp::tool_result::{ToolResult, failure, success};
use schemars::JsonSchema;
use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, JsonSchema)]
pub struct ClickTool {
    /// Tab id as returned by `list_tabs`.
    pub tab: String,
    /// Viewport X coordinate in CSS pixels.
    pub x: f64,
    /// Viewport Y coordinate in CSS pixels.
    pub y: f64,
}

impl ClickTool {
    ////////////////////////////////////////////////////////////////////////////////
    pub async fn exec(self, tab: &ChromeTab) -> ToolResult {
        tab.click(self.x, self.y).await.map_err(failure)?;
        success(format!("clicked at ({}, {})", self.x, self.y))
    }
}

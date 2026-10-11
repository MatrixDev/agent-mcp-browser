use crate::browser::ChromeTab;
use crate::mcp::tool_result::{ToolResult, failure, success};
use schemars::JsonSchema;
use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, JsonSchema)]
pub struct ScrollTool {
    /// Tab id as returned by `list_tabs`.
    pub tab: String,
    /// Horizontal scroll amount in pixels.
    #[serde(default)]
    pub delta_x: f64,
    /// Vertical scroll amount in pixels (positive scrolls down).
    #[serde(default)]
    pub delta_y: f64,
    /// X coordinate of the scrollable element; omit together with `y` to scroll the page.
    pub x: Option<f64>,
    /// Y coordinate of the scrollable element; omit together with `x` to scroll the page.
    pub y: Option<f64>,
}

impl ScrollTool {
    ////////////////////////////////////////////////////////////////////////////////
    pub async fn exec(self, tab: &ChromeTab) -> ToolResult {
        let at = self.x.zip(self.y);
        tab.scroll(self.delta_x, self.delta_y, at).await.map_err(failure)?;

        success(format!("scrolled by ({}, {})", self.delta_x, self.delta_y))
    }
}

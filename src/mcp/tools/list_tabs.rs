use crate::browser::ChromeTab;
use crate::mcp::tool_result::{ToolResult, success};
use schemars::JsonSchema;
use serde::Deserialize;

////////////////////////////////////////////////////////////////////////////////
#[derive(Deserialize, JsonSchema)]
pub struct ListTabsTool {}

impl ListTabsTool {
    ////////////////////////////////////////////////////////////////////////////////
    pub async fn exec(self, tabs: Vec<ChromeTab>) -> ToolResult {
        if tabs.is_empty() {
            return success("no tabs are exposed");
        }

        let mut lines = Vec::with_capacity(tabs.len());
        for tab in tabs {
            let id = tab.id().as_ref().to_owned();
            lines.push(format!("{id}: {}", tab.title().await));
        }

        success(lines.join("\n"))
    }
}

mod tool_result;
mod tools;

use crate::browser::ChromeTab;
use crate::context::Context;
use crate::mcp::tool_result::ToolResult;
use crate::mcp::tools::click::ClickTool;
use crate::mcp::tools::list_tabs::ListTabsTool;
use crate::mcp::tools::screenshot::ScreenshotTool;
use crate::mcp::tools::scroll::ScrollTool;
use crate::mcp::tools::wait::WaitTool;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::{StreamableHttpServerConfig, StreamableHttpService};
use rmcp::{ErrorData, tool, tool_router};
use tokio::net::TcpListener;

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct McpServer {
    tabs: Context,
}

impl McpServer {
    ////////////////////////////////////////////////////////////////////////////////
    pub fn new(tabs: Context) -> Self {
        Self { tabs }
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub async fn run(self, listener: TcpListener) -> anyhow::Result<()> {
        let service = StreamableHttpService::new(
            move || Ok(self.clone()),
            LocalSessionManager::default().into(),
            StreamableHttpServerConfig::default(),
        );
        let router = axum::Router::new().nest_service("/mcp", service);

        Ok(axum::serve(listener, router).await?)
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn tab(&self, id: &str) -> Result<ChromeTab, ErrorData> {
        let tab = self.tabs.get(id);
        tab.ok_or_else(|| ErrorData::invalid_params(format!("tab {id} is not exposed"), None))
    }
}

#[tool_router(server_handler)]
impl McpServer {
    ////////////////////////////////////////////////////////////////////////////////
    #[tool(description = "List tabs available for control, with their ids, titles and URLs")]
    async fn list_tabs(&self, Parameters(tool): Parameters<ListTabsTool>) -> ToolResult {
        tool.exec(self.tabs.all()).await
    }

    ////////////////////////////////////////////////////////////////////////////////
    #[tool(description = "Capture a screenshot of a tab's viewport")]
    async fn screenshot(&self, Parameters(tool): Parameters<ScreenshotTool>) -> ToolResult {
        let tab = self.tab(&tool.tab)?;
        tool.exec(&tab).await
    }

    ////////////////////////////////////////////////////////////////////////////////
    #[tool(description = "Wait for the given number of milliseconds")]
    async fn wait(&self, Parameters(tool): Parameters<WaitTool>) -> ToolResult {
        tool.exec().await
    }

    ////////////////////////////////////////////////////////////////////////////////
    #[tool(description = "Left-click at viewport coordinates (CSS pixels) in a tab")]
    async fn click(&self, Parameters(tool): Parameters<ClickTool>) -> ToolResult {
        let tab = self.tab(&tool.tab)?;
        tool.exec(&tab).await
    }

    ////////////////////////////////////////////////////////////////////////////////
    #[tool(description = "Scroll a tab by delta pixels; pass x/y to scroll the element instead")]
    async fn scroll(&self, Parameters(tool): Parameters<ScrollTool>) -> ToolResult {
        let tab = self.tab(&tool.tab)?;
        tool.exec(&tab).await
    }
}

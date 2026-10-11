mod browser;
mod context;
mod mcp;
mod tui;

use crate::browser::Chrome;
use crate::context::Context;
use crate::mcp::McpServer;
use crate::tui::picker::TabPicker;
use std::net::Ipv4Addr;
use tokio::net::TcpListener;
use tokio::select;

////////////////////////////////////////////////////////////////////////////////
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let chrome = Chrome::launch().await?;
    let context = Context::default();

    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await?;
    let url = format!("http://{}/mcp", listener.local_addr()?);

    let server = McpServer::new(context.clone());
    let picker = TabPicker::new(context, url);

    select! {
        e = server.run(listener) => e?,
        e = picker.run(&chrome) => e?,
    }
    Ok(())
}

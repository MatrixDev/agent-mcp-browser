use crate::browser::ChromeTab;
use anyhow::anyhow;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
use tokio::task::JoinSet;

////////////////////////////////////////////////////////////////////////////////
pub struct Chrome {
    browser: Browser,
    _join_set: JoinSet<()>,
}

impl Chrome {
    ////////////////////////////////////////////////////////////////////////////////
    pub async fn launch() -> anyhow::Result<Self> {
        let config = BrowserConfig::builder()
            .with_head()
            .viewport(None)
            .user_data_dir(std::env::temp_dir().join("agent-mcp-browser"))
            .build()
            .map_err(|e| anyhow!(e))?;

        let (browser, mut handler) = Browser::launch(config).await?;

        let mut join_set = JoinSet::new();
        join_set.spawn(async move { while handler.next().await.is_some() {} });

        Ok(Self {
            browser,
            _join_set: join_set,
        })
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub async fn tabs(&self) -> anyhow::Result<Vec<ChromeTab>> {
        let pages = self.browser.pages().await?;
        Ok(pages.into_iter().map(ChromeTab::new).collect())
    }
}

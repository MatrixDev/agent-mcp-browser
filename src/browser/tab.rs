use std::time::Duration;

use anyhow::{Context, anyhow};
use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::input::{DispatchMouseEventParams, DispatchMouseEventType};
use chromiumoxide::cdp::browser_protocol::page::{CaptureScreenshotFormat, CaptureScreenshotParams};
use chromiumoxide::cdp::browser_protocol::target::TargetId;
use chromiumoxide::layout::Point;

const SCREENSHOT_TIMEOUT: Duration = Duration::from_secs(10);

////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct ChromeTab {
    page: Page,
}

impl ChromeTab {
    ////////////////////////////////////////////////////////////////////////////////
    pub fn new(page: Page) -> Self {
        Self { page }
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub fn id(&self) -> &TargetId {
        self.page.target_id()
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub async fn title(&self) -> String {
        let title = self.page.get_title().await;
        title.ok().flatten().unwrap_or_default()
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub async fn screenshot(&self) -> anyhow::Result<String> {
        let params = CaptureScreenshotParams::builder()
            .format(CaptureScreenshotFormat::Png)
            .build();

        let response = tokio::time::timeout(SCREENSHOT_TIMEOUT, self.page.execute(params))
            .await
            .context("screenshot timed out (tab may not be rendering)")??;

        Ok(response.result.data.into())
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub async fn click(&self, x: f64, y: f64) -> anyhow::Result<()> {
        self.page.click(Point { x, y }).await?;
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub async fn scroll(&self, delta_x: f64, delta_y: f64, at: Option<(f64, f64)>) -> anyhow::Result<()> {
        match at {
            Some((x, y)) => {
                let wheel = DispatchMouseEventParams::builder()
                    .r#type(DispatchMouseEventType::MouseWheel)
                    .x(x)
                    .y(y)
                    .delta_x(delta_x)
                    .delta_y(delta_y)
                    .build()
                    .map_err(|e| anyhow!(e))?;
                self.page.execute(wheel).await?;
            }
            None => {
                self.page
                    .evaluate(format!("window.scrollBy({delta_x}, {delta_y})"))
                    .await?;
            }
        }
        Ok(())
    }
}

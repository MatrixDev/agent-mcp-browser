use crate::browser::{Chrome, ChromeTab};
use crate::context::Context;
use crate::tui::{STYLE_BG_TEXT, STYLE_HIGHLIGHT, STYLE_SELECTION};
use chromiumoxide::cdp::browser_protocol::target::TargetId;
use futures::StreamExt;
use indexmap::IndexMap;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{Event, EventStream, KeyCode, KeyEventKind};
use ratatui::prelude::*;
use ratatui::widgets::{Block, List, ListState, Paragraph};
use std::ops::ControlFlow;
use std::time::Duration;
use tokio::select;
use tokio::time::MissedTickBehavior;

const REFRESH_EVERY: Duration = Duration::from_secs(1);
const HOTKEYS: [(&str, &str); 5] = [
    ("↑↓", "prev/next"),
    ("←→", "first/last"),
    ("enter", "toggle"),
    ("c", "copy claude launch command"),
    ("q", "quit"),
];

////////////////////////////////////////////////////////////////////////////////
pub struct TabPicker {
    exposed: Context,
    mcp_url: String,
    entries: IndexMap<TargetId, TabEntry>,
    list_state: ListState,
    header_note: Option<String>,
}

struct TabEntry {
    tab: ChromeTab,
    label: String,
    selected: bool,
    marked_for_removal: bool,
}

impl TabPicker {
    ////////////////////////////////////////////////////////////////////////////////
    pub fn new(exposed: Context, mcp_url: String) -> Self {
        Self {
            exposed,
            mcp_url,
            entries: Default::default(),
            list_state: Default::default(),
            header_note: None,
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    pub async fn run(mut self, chrome: &Chrome) -> anyhow::Result<()> {
        let mut terminal = ratatui::init();
        let result = self.event_loop(&mut terminal, chrome).await;
        ratatui::restore();
        result
    }

    ////////////////////////////////////////////////////////////////////////////////
    async fn event_loop(&mut self, terminal: &mut DefaultTerminal, chrome: &Chrome) -> anyhow::Result<()> {
        let mut events = EventStream::new();
        let mut refresh = tokio::time::interval(REFRESH_EVERY);
        refresh.set_missed_tick_behavior(MissedTickBehavior::Delay);

        loop {
            self.update_entries(chrome).await?;

            terminal.draw(|frame| self.draw(frame))?;

            select! {
                Some(Ok(event)) = events.next() => {
                    if self.handle_event(event)?.is_break() {
                        return Ok(());
                    }
                }
                _ = refresh.tick() => {},
            }
        }
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn handle_event(&mut self, event: Event) -> anyhow::Result<ControlFlow<()>> {
        let Event::Key(key) = event else {
            return Ok(ControlFlow::Continue(()));
        };
        if key.kind != KeyEventKind::Press {
            return Ok(ControlFlow::Continue(()));
        }

        self.header_note = None;
        match key.code {
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Left => self.list_state.select_first(),
            KeyCode::Right => self.list_state.select_last(),
            KeyCode::Enter => self.toggle(),
            KeyCode::Char('c') => self.copy_claude_launch_command(),
            KeyCode::Esc | KeyCode::Char('q') => return Ok(ControlFlow::Break(())),
            _ => {}
        }
        Ok(ControlFlow::Continue(()))
    }

    ////////////////////////////////////////////////////////////////////////////////
    async fn update_entries(&mut self, chrome: &Chrome) -> anyhow::Result<()> {
        for entry in self.entries.values_mut() {
            entry.marked_for_removal = true;
        }

        for tab in chrome.tabs().await? {
            let entry_id = tab.id().clone();
            let entry = self.entries.entry(entry_id).or_insert_with(|| TabEntry {
                tab,
                label: Default::default(),
                selected: false,
                marked_for_removal: false,
            });

            entry.marked_for_removal = false;
            entry.label = entry.tab.title().await;
        }

        let mut should_publish = false;
        for (_, entry) in self.entries.extract_if(.., |_, e| e.marked_for_removal) {
            should_publish |= entry.selected;
        }

        if should_publish {
            self.publish();
        }
        Ok(())
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn toggle(&mut self) {
        let Some(index) = self.list_state.selected() else {
            return;
        };
        let Some((_, entry)) = self.entries.get_index_mut(index) else {
            return;
        };

        entry.selected = !entry.selected;
        self.publish();
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn publish(&self) {
        let tabs = self
            .entries
            .values()
            .filter(|e| e.selected)
            .map(|e| e.tab.clone())
            .collect();
        self.exposed.set(tabs);
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn copy_claude_launch_command(&mut self) {
        let json = serde_json::json!({
            "mcpServers": {
                "mdev_browser": {
                    "type":"http",
                    "url": self.mcp_url,
                }
            }
        });

        let command = format!("claude --mcp-config '{json}' --allowedTools mcp__mdev_browser --strict-mcp-config");
        let result = arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(command));

        let note = match result {
            Ok(()) => "Launch command copied to clipboard".to_owned(),
            Err(e) => format!("Failed to copy launch command: {e}"),
        };
        self.header_note = Some(note);
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn draw(&mut self, frame: &mut Frame) {
        let constraints = [Constraint::Fill(1), Constraint::Length(3)];
        let [tabs_area, hotkeys_area] = Layout::vertical(constraints).areas(frame.area());

        self.draw_tabs(frame, tabs_area);
        self.draw_hotkeys(frame, hotkeys_area);
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn draw_tabs(&mut self, frame: &mut Frame, area: Rect) {
        let items = self.entries.values().map(|e| {
            let mark = match e.selected {
                true => "[x]",
                false => "[ ]",
            };
            format!("{mark} {}", e.label)
        });

        let title = Line::default().spans([
            Span::raw(" Select pages available via MCP at "),
            Span::styled(&self.mcp_url, STYLE_HIGHLIGHT),
            Span::raw(" "),
        ]);

        let block = Block::bordered().title(title);
        let list = List::new(items).block(block).highlight_style(STYLE_SELECTION);

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }

    ////////////////////////////////////////////////////////////////////////////////
    fn draw_hotkeys(&self, frame: &mut Frame, area: Rect) {
        let mut spans = Vec::new();
        let mut separator = "";

        for (key, action) in HOTKEYS {
            spans.push(Span::styled(separator, STYLE_BG_TEXT));
            spans.push(Span::styled(format!(" {key} "), STYLE_HIGHLIGHT));
            spans.push(Span::styled(format!("{action} "), STYLE_BG_TEXT));
            separator = " | ";
        }

        // https://webquiz.net/movie/1002067-modern-disney-animation-/play
        let block = Block::bordered().title(" Hotkeys ");
        let hotkeys = Paragraph::new(Line::from(spans)).block(block);

        frame.render_widget(hotkeys, area);
    }
}

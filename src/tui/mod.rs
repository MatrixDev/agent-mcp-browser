use ratatui::prelude::*;

mod notice;
pub mod picker;

////////////////////////////////////////////////////////////////////////////////
const STYLE_BG_TEXT: Style = Style::new().add_modifier(Modifier::DIM);
const STYLE_HIGHLIGHT: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::BOLD);
const STYLE_SELECTION: Style = Style::new()
    .bg(Color::Blue)
    .fg(Color::White)
    .add_modifier(Modifier::BOLD);

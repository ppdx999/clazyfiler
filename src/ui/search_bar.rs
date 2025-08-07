use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::{handlers::Handler, model::AppModel};

/// Renders the search bar component at the bottom
pub fn render_search_bar(
    frame: &mut Frame,
    area: Rect,
    model: &AppModel,
    handler: &Handler,
) {
    let (title, border_color, text_color, search_text) = match handler {
        Handler::Search(_) => (
            "🔍 Search Mode (Active)",
            Color::Green,
            Color::White,
            if model.query_text.is_empty() {
                "Type to search..."
            } else {
                &model.query_text
            },
        ),
        Handler::FuzzyFind(_) => (
            "🔍 Fuzzy Find Mode (Active) - ESC to exit",
            Color::Cyan,
            Color::White,
            if model.query_text.is_empty() {
                "Type to fuzzy search files..."
            } else {
                &model.query_text
            },
        ),
        Handler::Explore(_) => (
            "Search (Press '/' to search, 'f' for fuzzy find)",
            Color::Yellow,
            Color::DarkGray,
            "Press '/' to search or 'f' for fuzzy find...",
        ),
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let paragraph = Paragraph::new(search_text)
        .block(block)
        .style(Style::default().fg(text_color));

    frame.render_widget(paragraph, area);
}
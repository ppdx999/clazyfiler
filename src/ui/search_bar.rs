use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::{handlers::Handler, state::SearchInputViewState};

/// Renders the search bar component at the bottom
pub fn render_search_bar(
    frame: &mut Frame,
    area: Rect,
    search_input_view: &SearchInputViewState,
    handler: &Handler,
) {
    let (title, border_color, text_color, search_text) = match handler {
        Handler::Search(_) => (
            "🔍 Search Mode (Active)",
            Color::Green,
            Color::White,
            if search_input_view.query.is_empty() {
                "Type to search..."
            } else {
                &search_input_view.query
            },
        ),
        Handler::FuzzyFind(_) => (
            "🔍 Fuzzy Find Mode (Active) - ESC to exit",
            Color::Cyan,
            Color::White,
            if search_input_view.query.is_empty() {
                "Type to fuzzy search files..."
            } else {
                &search_input_view.query
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
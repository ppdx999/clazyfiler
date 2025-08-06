use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::state::FileDetailViewState;

/// Renders the file description component on the right side
pub fn render_file_description(
    frame: &mut Frame,
    area: Rect,
    file_detail_view: &FileDetailViewState,
) {
    let block = Block::default()
        .title(file_detail_view.title.clone())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(file_detail_view.content.clone())
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
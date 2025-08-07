use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::model::AppModel;

/// Renders the file description component on the right side
pub fn render_file_description(
    frame: &mut Frame,
    area: Rect,
    model: &AppModel,
) {
    // Generate title and content directly from model
    let (title, content) = if let Some(selected_file) = model.get_selected_file() {
        let title = if selected_file.is_directory {
            format!("📁 {}", selected_file.name)
        } else {
            format!("📄 {}", selected_file.name)
        };
        let content = model.get_file_content(selected_file);
        (title, content)
    } else {
        (
            "No file selected".to_string(),
            "Select a file to see details...".to_string(),
        )
    };

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: true });

    frame.render_widget(paragraph, area);
}
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use crate::{handlers::Handler, state::FileListViewState};

/// Renders the file list component on the left side
pub fn render_file_list(
    frame: &mut Frame,
    area: Rect,
    file_list_view: &FileListViewState,
    current_dir: &std::path::Path,
    handler: &Handler,
) {
    let title = &file_list_view.title;

    let items: Vec<ListItem> = file_list_view
        .files
        .iter()
        .map(|file| {
            let icon = if file.is_directory { "📁" } else { "📄" };

            // Show relative path for fuzzy find, just name for others
            let display_name = match handler {
                Handler::FuzzyFind(_) => {
                    if let Ok(relative) = file.path.strip_prefix(current_dir) {
                        relative.to_string_lossy().to_string()
                    } else {
                        file.path.to_string_lossy().to_string()
                    }
                }
                _ => file.name.clone(),
            };

            ListItem::new(format!("{} {}", icon, display_name))
        })
        .collect();

    let block = Block::default()
        .title(title.clone())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let mut list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Color::DarkGray)
            .fg(Color::Yellow),
    );

    // Always show highlight symbol if we have files
    if file_list_view.has_files() {
        list = list.highlight_symbol("> ");
    }

    let selected_index = if file_list_view.has_files() {
        Some(file_list_view.selected_index)
    } else {
        None
    };

    frame.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(selected_index),
    );
}
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use crate::{handlers::Handler, model::{AppModel, AppMode}};

/// Renders the file list component on the left side  
pub fn render_file_list(
    frame: &mut Frame,
    area: Rect,
    model: &AppModel,
    handler: &Handler,
) {
    // Generate title directly from model
    let title = match model.mode {
        AppMode::Explore => {
            if model.query_text.is_empty() {
                format!("Files - {}", model.current_dir.display())
            } else {
                format!("Search - {}", model.current_dir.display())
            }
        }
        AppMode::Search => {
            format!("Search - {}", model.current_dir.display())
        }
        AppMode::FuzzyFind => {
            if model.is_indexing {
                format!("🔍 Fuzzy Find - Indexing... ({} files)", model.all_files_cache.len())
            } else {
                format!("🔍 Fuzzy Find - {} total files", model.all_files_cache.len())
            }
        }
    };

    let items: Vec<ListItem> = model
        .files
        .iter()
        .map(|file| {
            let icon = if file.is_directory { "📁" } else { "📄" };

            // Show relative path for fuzzy find, just name for others
            let display_name = match handler {
                Handler::FuzzyFind(_) => {
                    // For fuzzy find, show relative path from root
                    file.path.file_name()
                        .map(|name| name.to_string_lossy().to_string())
                        .unwrap_or_else(|| file.name.clone())
                }
                _ => file.name.clone(),
            };

            ListItem::new(format!("{} {}", icon, display_name))
        })
        .collect();

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let mut list = List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Color::DarkGray)
            .fg(Color::Yellow),
    );

    // Always show highlight symbol if we have files
    if !model.files.is_empty() {
        list = list.highlight_symbol("> ");
    }

    let selected_index = if !model.files.is_empty() {
        Some(model.selected_index)
    } else {
        None
    };

    frame.render_stateful_widget(
        list,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(selected_index),
    );
}
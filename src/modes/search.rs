use super::Mode;
use crate::{actions::Action, config::Config, store::Store, ui_helpers};
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
    Frame,
};

/// Search mode - file searching and filtering mode
pub struct SearchMode;

impl Mode for SearchMode {
    fn handle_key(&self, key: KeyCode, _config: &Config) -> Option<Action> {
        match key {
            KeyCode::Char(c) => {
                Some(Action::UpdateSearchQuery(c.to_string()))
            }
            KeyCode::Backspace => {
                Some(Action::UpdateSearchQuery(String::new())) // Empty string signals backspace
            }
            KeyCode::Enter => {
                Some(Action::SearchSelectFirst)
            }
            KeyCode::Esc => {
                Some(Action::ExitSearchMode)
            }
            KeyCode::Up => {
                Some(Action::MoveSelection(-1))
            }
            KeyCode::Down => {
                Some(Action::MoveSelection(1))
            }
            _ => None,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, store: &Store) {
        // Create layout with search bar at bottom
        let (chunks, search_area) = ui_helpers::create_layout_with_search_bar(area, store.config.ui.panel_width_ratio as u16);
        
        // Render file list in left panel with search highlighting
        let file_items = self.create_file_list(store);
        let title = format!("Search Results ({} matches)", store.state.filtered_files.len());
        ui_helpers::render_file_list(frame, chunks[0], &title, file_items, store.state.selected_index);
        
        // Render file details in right panel
        ui_helpers::render_file_details(frame, chunks[1], store);
        
        // Render search bar at bottom
        ui_helpers::render_search_bar(frame, search_area, &store.state.search_query);
    }

    fn on_enter(&self, store: &mut Store) {
        store.enter_search_mode();
    }

    fn on_exit(&self, store: &mut Store) {
        store.exit_search_mode();
    }

}

// Internal helper methods for SearchMode
impl SearchMode {
    fn create_file_list(&self, store: &Store) -> Vec<ListItem> {
        store.state
            .filtered_files
            .iter()
            .enumerate()
            .filter_map(|(display_index, &file_index)| {
                // Bounds check to prevent index out of bounds
                if file_index < store.state.files.len() {
                    let file = &store.state.files[file_index];
                    let is_selected = display_index == store.state.selected_index;
                    
                    let prefix = if file.is_dir { "📁 " } else { "📄 " };
                    let line = create_highlighted_line(&file.name, &store.state.search_query, prefix, is_selected);
                    
                    Some(ListItem::new(line))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn create_highlighted_line(filename: &str, query: &str, prefix: &str, is_selected: bool) -> Line<'static> {
    if query.is_empty() {
        let style = if is_selected {
            Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        return Line::from(Span::styled(format!("{}{}", prefix, filename), style));
    }

    let mut spans = vec![];
    let query_lower = query.to_lowercase();
    let filename_lower = filename.to_lowercase();
    
    // Add prefix first
    let base_style = if is_selected {
        Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    
    spans.push(Span::styled(prefix.to_string(), base_style));
    
    let mut start = 0;
    while let Some(match_start) = filename_lower[start..].find(&query_lower) {
        let absolute_start = start + match_start;
        let absolute_end = absolute_start + query.len();
        
        // Add text before the match
        if absolute_start > start {
            spans.push(Span::styled(
                filename[start..absolute_start].to_string(),
                base_style
            ));
        }
        
        // Add highlighted match
        let highlight_style = if is_selected {
            Style::default().bg(Color::Blue).fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD)
        };
        
        spans.push(Span::styled(
            filename[absolute_start..absolute_end].to_string(),
            highlight_style
        ));
        
        start = absolute_end;
    }
    
    // Add remaining text after last match
    if start < filename.len() {
        spans.push(Span::styled(
            filename[start..].to_string(),
            base_style
        ));
    }
    
    Line::from(spans)
}
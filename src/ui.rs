use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::{handlers::Handler, state::AppState};

pub struct UI;

impl UI {
    /// Render the file list component on the left side
    pub fn render_file_list(frame: &mut Frame, area: Rect, state: &AppState, handler: &Handler) {
        let (title, items, selected_index, files_len) = match handler {
            Handler::FuzzyFind(_) => {
                // Fuzzy find mode: show files from fuzzy find state
                let title = if state.fuzzy_find.is_indexing {
                    format!("🔍 Fuzzy Find - Indexing... ({} files)", state.fuzzy_find.total_count())
                } else {
                    format!("🔍 Fuzzy Find - {} of {} files", state.fuzzy_find.filtered_count(), state.fuzzy_find.total_count())
                };
                
                let items: Vec<ListItem> = (0..state.fuzzy_find.filtered_count())
                    .filter_map(|i| state.fuzzy_find.get_filtered_file(i))
                    .map(|file| {
                        let icon = if file.is_directory { "📁" } else { "📄" };
                        // Show relative path for fuzzy find
                        let display_path = if let Ok(relative) = file.path.strip_prefix(state.current_dir()) {
                            relative.to_string_lossy().to_string()
                        } else {
                            file.path.to_string_lossy().to_string()
                        };
                        ListItem::new(format!("{} {}", icon, display_path))
                    })
                    .collect();
                
                let selected_index = if state.fuzzy_find.filtered_count() == 0 {
                    None
                } else {
                    Some(state.fuzzy_find.selected_index)
                };
                
                (title, items, selected_index, state.fuzzy_find.filtered_count())
            },
            _ => {
                // Normal mode: show files from navigation state
                let title = format!("Files - {}", state.current_dir().display());
                let items: Vec<ListItem> = (0..state.filtered_files_len())
                    .filter_map(|i| state.get_filtered_file(i))
                    .map(|file| {
                        let icon = if file.is_directory { "📁" } else { "📄" };
                        ListItem::new(format!("{} {}", icon, file.name))
                    })
                    .collect();

                let selected_index = if state.filtered_files_len() == 0 {
                    None
                } else {
                    Some(state.selected_index())
                };
                
                (title, items, selected_index, state.filtered_files_len())
            }
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        let mut list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::Yellow)
            );

        // Always show highlight symbol if we have files
        if files_len > 0 {
            list = list.highlight_symbol("> ");
        }
        
        frame.render_stateful_widget(list, area, &mut ratatui::widgets::ListState::default().with_selected(selected_index));
    }

    /// Render the file description component on the right side
    pub fn render_file_description(frame: &mut Frame, area: Rect, state: &AppState, handler: &Handler) {
        let selected_file = match handler {
            Handler::FuzzyFind(_) => state.fuzzy_find.get_selected_file(),
            _ => state.get_selected_file(),
        };
        
        let (title, content) = if let Some(selected_file) = selected_file {
            let title = if selected_file.is_directory {
                format!("📁 {}", selected_file.name)
            } else {
                format!("📄 {}", selected_file.name)
            };
            let content = state.read_file_content(selected_file);
            (title, content)
        } else {
            ("No Selection".to_string(), "No file or directory selected".to_string())
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


    /// Render the search bar component at the bottom
    pub fn render_search_bar(frame: &mut Frame, area: Rect, state: &AppState, handler: &Handler) {
        let (title, border_color, text_color, search_text) = match handler {
            Handler::Search(_) => (
                "🔍 Search Mode (Active)",
                Color::Green,
                Color::White,
                if state.search_query().is_empty() {
                    "Type to search..."
                } else {
                    state.search_query()
                }
            ),
            Handler::FuzzyFind(_) => (
                "🔍 Fuzzy Find Mode (Active) - ESC to exit",
                Color::Cyan,
                Color::White,
                if state.fuzzy_find.query.is_empty() {
                    "Type to fuzzy search files..."
                } else {
                    &state.fuzzy_find.query
                }
            ),
            Handler::Explore(_) => (
                "Search (Press '/' to search, 'f' for fuzzy find)",
                Color::Yellow,
                Color::DarkGray,
                "Press '/' to search or 'f' for fuzzy find..."
            )
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

    /// Create the main layout with three areas: file list, description, and search bar
    pub fn create_main_layout(area: Rect) -> (Rect, Rect, Rect) {
        // Create vertical layout: main area + search bar
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),      // Main content area
                Constraint::Length(3),   // Search bar (fixed height)
            ])
            .split(area);

        // Create horizontal layout for main area: file list + description
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // File list (left half)
                Constraint::Percentage(50), // File description (right half)
            ])
            .split(main_chunks[0]);

        (content_chunks[0], content_chunks[1], main_chunks[1])
    }

    /// Complete UI render function that combines all components
    pub fn render_complete_ui(frame: &mut Frame, state: &AppState, handler: &Handler) {
        let area = frame.area();
        let (file_list_area, description_area, search_area) = Self::create_main_layout(area);

        // Render all components using state data
        Self::render_file_list(frame, file_list_area, state, handler);
        Self::render_file_description(frame, description_area, state, handler);
        Self::render_search_bar(frame, search_area, state, handler);
    }
}

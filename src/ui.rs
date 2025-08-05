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
    pub fn render_file_list(frame: &mut Frame, area: Rect, state: &AppState) {
        let title = format!("Files - {}", state.current_dir().display());
        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        let items: Vec<ListItem> = (0..state.filtered_files_len())
            .filter_map(|i| state.get_filtered_file(i))
            .map(|file| {
                let icon = if file.is_directory { "📁" } else { "📄" };
                ListItem::new(format!("{} {}", icon, file.name))
            })
            .collect();

        let mut list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::Yellow)
            );

        // Always show highlight symbol if we have files
        if state.filtered_files_len() > 0 {
            list = list.highlight_symbol("> ");
        }

        // Use selection index directly - it's already bounded to filtered_files
        let selected_index = if state.filtered_files_len() == 0 {
            None
        } else {
            Some(state.selected_index())
        };
        
        frame.render_stateful_widget(list, area, &mut ratatui::widgets::ListState::default().with_selected(selected_index));
    }

    /// Render the file description component on the right side
    pub fn render_file_description(frame: &mut Frame, area: Rect, state: &AppState) {
        let (title, content) = if let Some(selected_file) = state.get_selected_file() {
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
        let (title, border_color, text_color) = match handler {
            Handler::Search(_) => (
                "🔍 Search Mode (Active)",
                Color::Green,
                Color::White
            ),
            Handler::Explore(_) => (
                "Search (Press '/' to activate)",
                Color::Yellow,
                Color::DarkGray
            )
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let search_text = if state.search_query().is_empty() {
            match handler {
                Handler::Search(_) => "Type to search...",
                Handler::Explore(_) => "Press '/' to search..."
            }
        } else {
state.search_query()
        };

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
        Self::render_file_list(frame, file_list_area, state);
        Self::render_file_description(frame, description_area, state);
        Self::render_search_bar(frame, search_area, state, handler);
    }
}

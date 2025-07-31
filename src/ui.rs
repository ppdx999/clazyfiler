use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use crate::state::AppState;

pub struct UIComponents;

impl UIComponents {
    /// Render the file list component on the left side
    pub fn render_file_list(frame: &mut Frame, area: Rect, _state: &AppState, _selected_index: Option<usize>) {
        // Create a bordered block for the file list
        let block = Block::default()
            .title("Files")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        // Mock file list for now - will be replaced with actual file data
        let items: Vec<ListItem> = vec![
            ListItem::new("Documents"),
            ListItem::new("Downloads"),
            ListItem::new("Pictures"),
            ListItem::new("Music"),
            ListItem::new("README.md"),
            ListItem::new("config.toml"),
        ];

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::Yellow)
            );

        frame.render_widget(list, area);
    }

    /// Render the file description component on the right side
    pub fn render_file_description(frame: &mut Frame, area: Rect, _state: &AppState, selected_file: Option<&str>) {
        let block = Block::default()
            .title("File Info")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White));

        let content = match selected_file {
            Some(filename) => {
                format!(
                    "File: {}\n\nSize: 1.2 KB\nModified: 2024-01-15 10:30:00\nPermissions: rw-r--r--\nType: Text file\n\nDescription:\nThis is a sample file description that shows detailed information about the selected file.", filename
                )
            },
            None => "No file selected".to_string(),
        };

        let paragraph = Paragraph::new(content)
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }

    /// Render the search bar component at the bottom
    pub fn render_search_bar(frame: &mut Frame, area: Rect, _state: &AppState, search_query: &str, is_active: bool) {
        let style = if is_active {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };

        let block = Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .border_style(style);

        let search_text = if search_query.is_empty() && !is_active {
            "Press '/' to search..."
        } else {
            search_query
        };

        let paragraph = Paragraph::new(search_text)
            .block(block)
            .style(style);

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
    pub fn render_complete_ui(
        frame: &mut Frame, 
        state: &AppState, 
        selected_index: Option<usize>,
        selected_file: Option<&str>,
        search_query: &str,
        search_active: bool
    ) {
        let area = frame.area();
        let (file_list_area, description_area, search_area) = Self::create_main_layout(area);

        // Render all components
        Self::render_file_list(frame, file_list_area, state, selected_index);
        Self::render_file_description(frame, description_area, state, selected_file);
        Self::render_search_bar(frame, search_area, state, search_query, search_active);
    }
}
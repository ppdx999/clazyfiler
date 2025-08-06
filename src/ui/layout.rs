use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
};

/// Creates the main layout with three areas: file list, description, and search bar
pub fn create_main_layout(area: Rect) -> (Rect, Rect, Rect) {
    // Create vertical layout: main area + search bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),    // Main content area
            Constraint::Length(3), // Search bar (fixed height)
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
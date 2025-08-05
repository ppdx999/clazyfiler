use crossterm::event::{KeyCode, KeyEvent};

use crate::{messages::AppMessage, state::AppState};

#[derive(Debug)]
pub struct ExploreHandler {
}

impl ExploreHandler {
    pub fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> Option<AppMessage> {
        match key.code {
            // Navigation keys - handle directly
            KeyCode::Char('j') | KeyCode::Down => {
                state.move_selection_down();
                None
            },
            KeyCode::Char('k') | KeyCode::Up => {
                state.move_selection_up();
                None
            },
            
            // Directory navigation
            KeyCode::Char('h') | KeyCode::Left | KeyCode::Esc => {
                match state.go_to_parent() {
                    Ok(_) => None,
                    Err(e) => Some(AppMessage::Error(format!("Navigation error: {}", e))),
                }
            },
            
            // Smart selection: directory navigation or file opening
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                if let Some(selected) = state.get_selected_file() {
                    if selected.is_directory {
                        // Navigate into directory
                        match state.enter_directory() {
                            Ok(_) => None,
                            Err(e) => Some(AppMessage::Error(format!("Navigation error: {}", e))),
                        }
                    } else {
                        // Open file - send message to App
                        Some(AppMessage::OpenFile)
                    }
                } else {
                    None
                }
            },
            
            // Refresh
            KeyCode::Char('r') | KeyCode::F(5) => {
                state.refresh_files();
                None
            },
            
            // Global actions - send messages to App
            KeyCode::Char('/') => Some(AppMessage::SwitchToSearchHandler),
            KeyCode::Char('q') => Some(AppMessage::Quit),
            
            _ => None,
        }
    }
}


impl ExploreHandler {
    pub fn new() -> Self {
        Self {
        }
    }
}

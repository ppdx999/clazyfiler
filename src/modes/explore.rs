use crossterm::event::{KeyCode, KeyEvent};

use crate::{actions::ModeSwitchAction, modes::interface::{KeyHandler, ModeResult}, state::AppState};

#[derive(Debug)]
pub struct ExploreHandler {
}

impl KeyHandler for ExploreHandler {
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> ModeResult {
        match key.code {
            // Navigation keys - handle directly
            KeyCode::Char('j') | KeyCode::Down => {
                state.move_selection_down();
                ModeResult::none()
            },
            KeyCode::Char('k') | KeyCode::Up => {
                state.move_selection_up();
                ModeResult::none()
            },
            
            // Directory navigation
            KeyCode::Char('h') | KeyCode::Left | KeyCode::Esc => {
                match state.go_to_parent() {
                    Ok(_) => ModeResult::none(),
                    Err(e) => ModeResult::error(format!("Navigation error: {}", e)),
                }
            },
            
            // Smart selection: directory navigation or file opening
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
                if let Some(selected) = state.get_selected_file() {
                    if selected.is_directory {
                        // Navigate into directory
                        match state.enter_directory() {
                            Ok(_) => ModeResult::none(),
                            Err(e) => ModeResult::error(format!("Navigation error: {}", e)),
                        }
                    } else {
                        // Open file - this needs App-level handling
                        ModeResult::open_file()
                    }
                } else {
                    ModeResult::none()
                }
            },
            
            // Refresh
            KeyCode::Char('r') | KeyCode::F(5) => {
                state.refresh_files();
                ModeResult::none()
            },
            
            // Global actions
            KeyCode::Char('/') => ModeResult::switch_mode(ModeSwitchAction::EnterSearchMode),
            KeyCode::Char('q') => ModeResult::quit(),
            
            _ => ModeResult::none(),
        }
    }
}


impl ExploreHandler {
    pub fn new() -> Self {
        Self {
        }
    }
}

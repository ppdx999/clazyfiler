use crossterm::event::{KeyCode, KeyEvent};

use crate::{actions::{Action, ModeSwitchAction}, modes::interface::ModeBehavior, state::AppState, ui::UIComponents};

#[derive(Debug)]
pub struct ExploreMode {
}

impl ModeBehavior for ExploreMode {
    fn handle_key(&self, key: KeyEvent, _state: &AppState) -> Option<Action> {
        match key.code {
            // Vim-style navigation
            KeyCode::Char('j') => Some(Action::MoveDown),
            KeyCode::Char('k') => Some(Action::MoveUp),
            KeyCode::Char('h') => Some(Action::Back),
            KeyCode::Char('l') => Some(Action::Select),
            
            // Arrow key navigation
            KeyCode::Down => Some(Action::MoveDown),
            KeyCode::Up => Some(Action::MoveUp),
            KeyCode::Left => Some(Action::Back),
            KeyCode::Right => Some(Action::Select),
            
            // Other common actions
            KeyCode::Enter => Some(Action::Select),
            KeyCode::Esc => Some(Action::Back),
            KeyCode::Char('r') => Some(Action::Refresh),
            KeyCode::F(5) => Some(Action::Refresh),
            KeyCode::Char('/') => Some(Action::SwitchMode(ModeSwitchAction::EnterSearchMode)),
            
            _ => None,
        }
    }
    fn dispatch(&mut self, action: Action, state: &mut AppState) -> Result<(), String> {
        match action {
            Action::MoveUp => {
                state.move_selection_up();
                Ok(())
            },
            Action::MoveDown => {
                state.move_selection_down();
                Ok(())
            },
            Action::Select => {
                // Try to enter directory or open file
                if let Some(selected) = state.get_selected_file() {
                    if selected.is_directory {
                        state.enter_directory()
                    } else {
                        // For now, just indicate file selection - could open file in future
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            },
            Action::Back => {
                state.go_to_parent()
            },
            Action::Refresh => {
                state.refresh_files();
                Ok(())
            },
            Action::SwitchMode(ModeSwitchAction::EnterSearchMode) => {
                state.toggle_search();
                Ok(())
            },
            _ => Ok(()),
        }
    }
    fn render(&self, frame: &mut ratatui::Frame, state: &AppState) {
        // In explore mode, render UI with current state
        UIComponents::render_complete_ui(frame, state);
    }
    
    fn on_enter(&mut self, state: &mut AppState) -> Result<(), String> {
        // When entering explore mode, ensure search is disabled
        if state.search_active {
            state.search_active = false;
            state.search_query.clear();
        }
        Ok(())
    }
    
    fn on_exit(&mut self, _state: &mut AppState) -> Result<(), String> {
        // No specific cleanup needed when exiting explore mode
        Ok(())
    }
}


impl ExploreMode {
    pub fn new() -> Self {
        Self {
        }
    }
}

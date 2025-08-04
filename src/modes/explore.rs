use crossterm::event::{KeyCode, KeyEvent};

use crate::{actions::{Action, ModeSwitchAction}, modes::interface::ModeBehavior, state::AppState};

#[derive(Debug)]
pub struct ExploreMode {
}

impl ModeBehavior for ExploreMode {
    fn handle_key(&self, key: KeyEvent, state: &AppState) -> Vec<Action> {
        match key.code {
            // Vim-style navigation
            KeyCode::Char('j') => vec![Action::MoveDown],
            KeyCode::Char('k') => vec![Action::MoveUp],
            KeyCode::Char('h') => vec![Action::Back],
            KeyCode::Char('l') => {
                // Smart selection: directory navigation or file opening
                if let Some(selected) = state.get_selected_file() {
                    if selected.is_directory {
                        vec![Action::Select]
                    } else {
                        vec![Action::OpenFile]
                    }
                } else {
                    vec![Action::Select]
                }
            },
            
            // Arrow key navigation
            KeyCode::Down => vec![Action::MoveDown],
            KeyCode::Up => vec![Action::MoveUp],
            KeyCode::Left => vec![Action::Back],
            KeyCode::Right => {
                // Smart selection: directory navigation or file opening
                if let Some(selected) = state.get_selected_file() {
                    if selected.is_directory {
                        vec![Action::Select]
                    } else {
                        vec![Action::OpenFile]
                    }
                } else {
                    vec![Action::Select]
                }
            },
            
            // Other common actions
            KeyCode::Enter => {
                // Smart selection: directory navigation or file opening
                if let Some(selected) = state.get_selected_file() {
                    if selected.is_directory {
                        vec![Action::Select]
                    } else {
                        vec![Action::OpenFile]
                    }
                } else {
                    vec![Action::Select]
                }
            },
            KeyCode::Esc => vec![Action::Back],
            KeyCode::Char('r') => vec![Action::Refresh],
            KeyCode::F(5) => vec![Action::Refresh],
            KeyCode::Char('/') => vec![Action::SwitchMode(ModeSwitchAction::EnterSearchMode)],
            KeyCode::Char('q') => vec![Action::Quit],
            
            _ => vec![],
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
                // Select action is now only for directories
                if let Some(selected) = state.get_selected_file() {
                    if selected.is_directory {
                        state.enter_directory()
                    } else {
                        // This shouldn't happen with new smart selection logic
                        Err("Select action called on file (this is a bug)".to_string())
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
            _ => Ok(()),
        }
    }
    fn render(&self, _frame: &mut ratatui::Frame, _state: &AppState) {
        // In explore mode, render UI with current state
        // Note: This is now handled by Mode::render_with_mode_context
        // Individual modes no longer call UI directly
    }
    
    fn on_enter(&mut self, _state: &mut AppState) -> Result<(), String> {
        // When entering explore mode, ensure search is disabled
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

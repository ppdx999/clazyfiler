
use crossterm::event::{KeyCode, KeyEvent};

use crate::{actions::{Action, ModeSwitchAction}, modes::interface::ModeBehavior, state::AppState, ui::UI};

#[derive(Debug)]
pub struct SearchMode {
}

impl ModeBehavior for SearchMode {
    fn handle_key(&self, key: KeyEvent, _state: &AppState) -> Vec<Action> {
        match key.code {
            // Exit search mode
            KeyCode::Enter => vec![Action::SwitchMode(ModeSwitchAction::EnterExploreMode)],
            KeyCode::Esc => vec![Action::SwitchMode(ModeSwitchAction::EnterExploreMode)],
            
            // Search input
            KeyCode::Backspace => vec![Action::SearchPop],
            KeyCode::Char(c) => vec![Action::SearchInput(c)],
            
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
            Action::SearchInput(c) => {
                state.append_search_query(c);
                Ok(())
            },
            Action::SearchPop => {
                state.pop_search_query();
                Ok(())
            }
            _ => Ok(())
        }
    }
    fn render(&self, frame: &mut ratatui::Frame, state: &AppState) {
        // In search mode, render UI with current state (search will be active in state)
        UI::render_complete_ui(frame, state);
    }
    
    fn on_enter(&mut self, _state: &mut AppState) -> Result<(), String> {
        // When entering search mode, activate search if not already active
        Ok(())
    }
    
    fn on_exit(&mut self, _state: &mut AppState) -> Result<(), String> {
        // When exiting search mode, clear search state
        Ok(())
    }
}

impl SearchMode {
    pub fn new() -> Self {
        Self {
        }
    }
}


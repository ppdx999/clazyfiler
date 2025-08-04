
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{actions::{Action, ModeSwitchAction}, modes::interface::ModeBehavior, state::AppState};

#[derive(Debug)]
pub struct SearchMode {
}

impl ModeBehavior for SearchMode {
    fn handle_key(&self, key: KeyEvent, _state: &AppState) -> Vec<Action> {
        match (key.code, key.modifiers) {
            // Exit actions
            (KeyCode::Enter, KeyModifiers::NONE) => vec![Action::SwitchMode(ModeSwitchAction::EnterExploreMode)],
            (KeyCode::Esc, KeyModifiers::NONE) => vec![Action::SearchClear, Action::SwitchMode(ModeSwitchAction::EnterExploreMode)],
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => vec![Action::SearchClear, Action::SwitchMode(ModeSwitchAction::EnterExploreMode)],
            
            // Character manipulation
            (KeyCode::Backspace, KeyModifiers::NONE) => vec![Action::SearchPop],
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => vec![Action::SearchPop], // Unix backspace
            (KeyCode::Char(c), KeyModifiers::NONE) => vec![Action::SearchInput(c)],
            
            // Unix terminal shortcuts
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => vec![Action::SearchClear],           // Clear entire line
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => vec![Action::SearchDeleteToEnd],    // Delete to end
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => vec![Action::SearchDeleteWord],     // Delete word backward
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => vec![Action::SearchHome],           // Move to beginning
            (KeyCode::Char('e'), KeyModifiers::CONTROL) => vec![Action::SearchEnd],            // Move to end
            
            // Additional shortcuts
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => vec![Action::SearchClear],          // Clear screen (clear search)
            (KeyCode::Delete, KeyModifiers::NONE) => vec![Action::SearchPop],                  // Alternative delete
            
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
            },
            Action::SearchClear => {
                state.clear_search_query();
                Ok(())
            },
            Action::SearchDeleteWord => {
                state.delete_word_backward();
                Ok(())
            },
            Action::SearchDeleteToEnd => {
                state.delete_to_end();
                Ok(())
            },
            Action::SearchHome => {
                // For search mode, home/end don't make sense since we're not editing cursor position
                // But we could interpret this as "go to first search result"
                // For now, just return Ok - could extend later
                Ok(())
            },
            Action::SearchEnd => {
                // Similar to SearchHome - could be "go to last search result"
                // For now, just return Ok - could extend later
                Ok(())
            },
            _ => Ok(())
        }
    }
}

impl SearchMode {
    pub fn new() -> Self {
        Self {
        }
    }
}


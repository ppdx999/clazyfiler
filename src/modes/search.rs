
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{actions::ModeSwitchAction, modes::interface::{KeyHandler, ModeResult}, state::AppState};

#[derive(Debug)]
pub struct SearchHandler {
}

impl KeyHandler for SearchHandler {
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> ModeResult {
        match (key.code, key.modifiers) {
            // Exit actions
            (KeyCode::Enter, KeyModifiers::NONE) => {
                ModeResult::switch_mode(ModeSwitchAction::EnterExploreMode)
            },
            (KeyCode::Esc, KeyModifiers::NONE) => {
                state.clear_search_query();
                ModeResult::switch_mode(ModeSwitchAction::EnterExploreMode)
            },
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                state.clear_search_query();
                ModeResult::switch_mode(ModeSwitchAction::EnterExploreMode)
            },
            
            // Character manipulation
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                state.pop_search_query();
                ModeResult::none()
            },
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                state.pop_search_query(); // Unix backspace
                ModeResult::none()
            },
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                state.append_search_query(c);
                ModeResult::none()
            },
            
            // Unix terminal shortcuts
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                state.clear_search_query(); // Clear entire line
                ModeResult::none()
            },
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                state.delete_to_end(); // Delete to end
                ModeResult::none()
            },
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                state.delete_word_backward(); // Delete word backward
                ModeResult::none()
            },
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                // For search mode, home/end don't make sense since we're not editing cursor position
                // For now, just return none - could extend later for "go to first search result"
                ModeResult::none()
            },
            (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                // Similar to Ctrl+A - could be "go to last search result"  
                // For now, just return none - could extend later
                ModeResult::none()
            },
            
            // Additional shortcuts
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                state.clear_search_query(); // Clear screen (clear search)
                ModeResult::none()
            },
            (KeyCode::Delete, KeyModifiers::NONE) => {
                state.pop_search_query(); // Alternative delete
                ModeResult::none()
            },
            
            _ => ModeResult::none(),
        }
    }
}

impl SearchHandler {
    pub fn new() -> Self {
        Self {
        }
    }
}


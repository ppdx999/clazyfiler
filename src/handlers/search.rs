
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{handlers::interface::KeyHandler, messages::{AppMessage, SwitchAction}, state::AppState};

#[derive(Debug)]
pub struct SearchHandler {
}

impl KeyHandler for SearchHandler {
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> Option<AppMessage> {
        match (key.code, key.modifiers) {
            // Exit actions - send messages to App
            (KeyCode::Enter, KeyModifiers::NONE) => {
                Some(AppMessage::SwitchMode(SwitchAction::EnterExploreMode))
            },
            (KeyCode::Esc, KeyModifiers::NONE) => {
                state.clear_search_query();
                Some(AppMessage::SwitchMode(SwitchAction::EnterExploreMode))
            },
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                state.clear_search_query();
                Some(AppMessage::SwitchMode(SwitchAction::EnterExploreMode))
            },
            
            // Character manipulation - handle locally
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                state.pop_search_query();
                None
            },
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                state.pop_search_query(); // Unix backspace
                None
            },
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                state.append_search_query(c);
                None
            },
            
            // Unix terminal shortcuts - handle locally
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                state.clear_search_query(); // Clear entire line
                None
            },
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                state.delete_to_end(); // Delete to end
                None
            },
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                state.delete_word_backward(); // Delete word backward
                None
            },
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => {
                // For search mode, home/end don't make sense since we're not editing cursor position
                // For now, just return none - could extend later for "go to first search result"
                None
            },
            (KeyCode::Char('e'), KeyModifiers::CONTROL) => {
                // Similar to Ctrl+A - could be "go to last search result"  
                // For now, just return none - could extend later
                None
            },
            
            // Additional shortcuts - handle locally
            (KeyCode::Char('l'), KeyModifiers::CONTROL) => {
                state.clear_search_query(); // Clear screen (clear search)
                None
            },
            (KeyCode::Delete, KeyModifiers::NONE) => {
                state.pop_search_query(); // Alternative delete
                None
            },
            
            _ => None,
        }
    }
}

impl SearchHandler {
    pub fn new() -> Self {
        Self {
        }
    }
}



use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{messages::AppMessage, model::AppModel};

#[derive(Debug)]
pub struct SearchHandler {
}

impl SearchHandler {
    pub fn handle_key(&mut self, key: KeyEvent, model: &mut AppModel) -> Option<AppMessage> {
        match (key.code, key.modifiers) {
            // Exit actions - send messages to App
            (KeyCode::Enter, KeyModifiers::NONE) => {
                Some(AppMessage::SwitchToExploreHandlerKeepQuery)  // Keep search results
            },
            (KeyCode::Esc, KeyModifiers::NONE) => {
                model.clear_query();
                Some(AppMessage::SwitchToExploreHandler)
            },
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                model.clear_query();
                Some(AppMessage::SwitchToExploreHandler)
            },
            
            // Navigation keys within search results
            (KeyCode::Down, KeyModifiers::NONE) => {
                model.move_selection_down();
                None
            },
            (KeyCode::Up, KeyModifiers::NONE) => {
                model.move_selection_up();
                None
            },
            
            // Unix-style navigation with Ctrl+N/Ctrl+P
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                model.move_selection_down();
                None
            },
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                model.move_selection_up();
                None
            },
            
            // Character manipulation - handle locally
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                model.pop_from_query();
                None
            },
            (KeyCode::Char('h'), KeyModifiers::CONTROL) => {
                model.pop_from_query();
                None
            },
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                model.append_to_query(c);
                None
            },
            
            // Unix terminal shortcuts - handle locally
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                model.clear_query();
                None
            },
            (KeyCode::Char('k'), KeyModifiers::CONTROL) => {
                model.delete_to_end();
                None
            },
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                model.delete_word_backward();
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
                model.clear_query();
                None
            },
            (KeyCode::Delete, KeyModifiers::NONE) => {
                model.pop_from_query();
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


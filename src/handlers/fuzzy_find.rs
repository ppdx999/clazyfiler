use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::{messages::AppMessage, state::AppState};

#[derive(Debug)]
pub struct FuzzyFindHandler {
}

impl FuzzyFindHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> Option<AppMessage> {
        match key.code {
            // Open selected file
            KeyCode::Enter => {
                if state.fuzzy_find.get_selected_file().is_some() {
                    Some(AppMessage::OpenFile)
                } else {
                    None
                }
            },
            
            // Edit search query
            KeyCode::Char(c) if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                state.fuzzy_find.append_to_query(c);
                None
            },
            
            // Delete characters
            KeyCode::Backspace => {
                state.fuzzy_find.pop_from_query();
                None
            },
            
            // Ctrl+W: Delete word backward
            KeyCode::Char('w') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                state.fuzzy_find.delete_word_backward();
                None
            },
            
            // Ctrl+U: Delete to end (changed from Ctrl+K to avoid conflict)
            KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                state.fuzzy_find.delete_to_end();
                None
            },

            // Escape: Return to explore mode
            KeyCode::Esc => Some(AppMessage::SwitchToExploreHandler),
            
            // Global actions
            KeyCode::Char('q') => Some(AppMessage::Quit),
            
            _ => None,
        }
    }
}

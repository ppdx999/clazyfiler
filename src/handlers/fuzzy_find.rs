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
        match (key.code, key.modifiers) {
            // Open selected file or navigate to directory
            (KeyCode::Enter, KeyModifiers::NONE) => {
                if let Some(selected_file) = state.fuzzy_find.get_selected_file() {
                    if selected_file.is_directory {
                        // Navigate to directory and switch back to explore mode
                        Some(AppMessage::NavigateToDirectory(selected_file.path.clone()))
                    } else {
                        // Open file with editor
                        Some(AppMessage::OpenFile)
                    }
                } else {
                    None
                }
            },
            
            // Navigation keys within fuzzy find results
            (KeyCode::Down, KeyModifiers::NONE) => {
                state.fuzzy_find.move_selection_down();
                None
            },
            (KeyCode::Up, KeyModifiers::NONE) => {
                state.fuzzy_find.move_selection_up();
                None
            },
            
            // Unix-style navigation with Ctrl+N/Ctrl+P
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                state.fuzzy_find.move_selection_down();
                None
            },
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                state.fuzzy_find.move_selection_up();
                None
            },
            
            // Exit actions
            (KeyCode::Esc, KeyModifiers::NONE) => Some(AppMessage::SwitchToExploreHandler),
            (KeyCode::Char('q'), KeyModifiers::NONE) => Some(AppMessage::Quit),
            
            // Character manipulation
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                state.fuzzy_find.pop_from_query();
                None
            },
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                state.fuzzy_find.delete_word_backward();
                None
            },
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                state.fuzzy_find.delete_to_end();
                None
            },
            
            // Edit search query - regular characters without control modifiers
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                state.fuzzy_find.append_to_query(c);
                None
            },
            
            _ => None,
        }
    }
}

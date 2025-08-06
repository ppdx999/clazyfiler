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
                if let Some(selected_file) = state.get_selected_file() {
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
                state.move_selection_down();
                None
            },
            (KeyCode::Up, KeyModifiers::NONE) => {
                state.move_selection_up();
                None
            },
            
            // Unix-style navigation with Ctrl+N/Ctrl+P
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => {
                state.move_selection_down();
                None
            },
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => {
                state.move_selection_up();
                None
            },
            
            // Exit actions
            (KeyCode::Esc, KeyModifiers::NONE) => Some(AppMessage::SwitchToExploreHandler),
            (KeyCode::Char('q'), KeyModifiers::NONE) => Some(AppMessage::Quit),
            
            // Character manipulation
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                state.pop_search_query();
                state.update_fuzzy_find_view();
                None
            },
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                state.delete_word_backward();
                state.update_fuzzy_find_view();
                None
            },
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                state.delete_to_end();
                state.update_fuzzy_find_view();
                None
            },
            
            // Edit search query - regular characters without control modifiers
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                state.append_search_query(c);
                state.update_fuzzy_find_view();
                None
            },
            
            _ => None,
        }
    }
}

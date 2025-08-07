use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::{messages::AppMessage, model::AppModel};

#[derive(Debug)]
pub struct FuzzyFindHandler {
}

impl FuzzyFindHandler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn handle_key(&mut self, key: KeyEvent, model: &mut AppModel) -> Option<AppMessage> {
        match (key.code, key.modifiers) {
            // Open selected file or navigate to directory
            (KeyCode::Enter, KeyModifiers::NONE) => {
                if let Some(selected_file) = model.get_selected_file() {
                    if selected_file.is_directory {
                        // Navigate to directory directly, then switch back to explore mode
                        let path = selected_file.path.clone();
                        if let Err(e) = model.change_directory(path) {
                            Some(AppMessage::Error(format!("Failed to navigate to directory: {}", e)))
                        } else {
                            Some(AppMessage::SwitchToExploreHandler)
                        }
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
            
            // Exit actions
            (KeyCode::Esc, KeyModifiers::NONE) => Some(AppMessage::SwitchToExploreHandler),
            (KeyCode::Char('q'), KeyModifiers::NONE) => Some(AppMessage::Quit),
            
            // Character manipulation
            (KeyCode::Backspace, KeyModifiers::NONE) => {
                model.pop_from_query();
                None
            },
            (KeyCode::Char('w'), KeyModifiers::CONTROL) => {
                model.delete_word_backward();
                None
            },
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                model.delete_to_end();
                None
            },
            
            // Edit search query - regular characters without control modifiers
            (KeyCode::Char(c), KeyModifiers::NONE) => {
                model.append_to_query(c);
                None
            },
            
            _ => None,
        }
    }
}

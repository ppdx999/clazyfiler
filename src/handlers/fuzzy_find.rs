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
            // Open selected file or navigate to directory
            KeyCode::Enter => {
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
            KeyCode::Char('j') | KeyCode::Down => {
                state.fuzzy_find.move_selection_down();
                None
            },
            KeyCode::Char('k') | KeyCode::Up => {
                state.fuzzy_find.move_selection_up();
                None
            },
            
            // Unix-style navigation with Ctrl+N/Ctrl+P
            KeyCode::Char('n') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                state.fuzzy_find.move_selection_down();
                None
            },
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                state.fuzzy_find.move_selection_up();
                None
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

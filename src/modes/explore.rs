use crossterm::event::KeyEvent;

use crate::{actions::Action, modes::interface::ModeBehavior, state::AppState, ui::UIComponents};

#[derive(Debug)]
pub struct ExploreMode {
}

impl ModeBehavior for ExploreMode {
    fn handle_key(&self, key: KeyEvent, _state: &AppState) -> Option<Action> {
        return None
    }
    fn dispatch(&mut self, _action: Action, _state: &mut AppState) -> Result<(), String> {
        Ok(())
    }
    fn render(&self, frame: &mut ratatui::Frame, state: &AppState) {
        // In explore mode, search is not active
        UIComponents::render_complete_ui(
            frame,
            state,
            Some(0), // Mock selected index - will be replaced with actual selection
            Some("README.md"), // Mock selected file - will be replaced with actual file
            "", // No search query in explore mode
            false // Search is not active
        );
    }
}


impl ExploreMode {
    pub fn new() -> Self {
        Self {
        }
    }
}

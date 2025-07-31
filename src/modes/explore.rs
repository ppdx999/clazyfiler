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
        // In explore mode, render UI with current state
        UIComponents::render_complete_ui(frame, state);
    }
}


impl ExploreMode {
    pub fn new() -> Self {
        Self {
        }
    }
}

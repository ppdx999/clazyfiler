
use crossterm::event::KeyEvent;

use crate::{actions::Action, modes::interface::ModeBehavior, state::AppState};

#[derive(Debug)]
pub struct SearchMode {
}

impl ModeBehavior for SearchMode {
    fn handle_key(&self, _key: KeyEvent, _state: &AppState) -> Option<Action> {
        return None
    }
    fn dispatch(&mut self, _action: Action, _state: &mut AppState) -> Result<(), String> {
        Ok(())
    }
    fn render(&self, _frame: &mut ratatui::Frame, _state: &AppState) {
    }
}


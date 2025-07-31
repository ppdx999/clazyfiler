
use crossterm::event::KeyCode;

use crate::{actions::Action, modes::interface::ModeBehavior};

#[derive(Debug)]
pub struct SearchMode {
}

impl ModeBehavior for SearchMode {
    fn handle_key(&self, key: KeyCode) -> Option<Action> {
        return None
    }
    fn dispatch(&mut self, _action: Action) -> Result<(), String> {
        Ok(())
    }
    fn render(&self, _frame: &mut ratatui::Frame) {
    }
}


use crossterm::event::KeyCode;

use crate::{actions::Action, modes::interface::ModeBehavior};

#[derive(Debug)]
pub struct ExploreMode {
}

impl ModeBehavior for ExploreMode {
    fn handle_key(&self, key: KeyCode) -> Option<Action> {
        return None
    }
    fn dispatch(&mut self, _action: Action) -> Result<(), String> {
        Ok(())
    }
    fn render(&self, _frame: &mut ratatui::Frame) {
    }
}


impl ExploreMode {
    pub fn new() -> Self {
        Self {
        }
    }
}

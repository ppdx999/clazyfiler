use crate::{actions::Action};
use crossterm::event::KeyCode;
use ratatui::Frame;

pub trait ModeBehavior {
    fn handle_key(&self, _key: KeyCode) -> Option<Action>;
    fn dispatch(&mut self, _action: Action) -> Result<(), String>;
    fn render(&self, _frame: &mut Frame);
}


use crate::{actions::Action, state::AppState};
use crossterm::event::KeyEvent;

pub trait ModeBehavior {
    fn handle_key(&self, key: KeyEvent, state: &AppState) -> Vec<Action>;
    fn dispatch(&mut self, action: Action, state: &mut AppState) -> Result<(), String>;
}


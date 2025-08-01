use crate::{actions::Action, state::AppState};
use crossterm::event::KeyEvent;
use ratatui::Frame;

pub trait ModeBehavior {
    fn handle_key(&self, key: KeyEvent, state: &AppState) -> Option<Action>;
    fn dispatch(&mut self, action: Action, state: &mut AppState) -> Result<(), String>;
    fn render(&self, frame: &mut Frame, state: &AppState);
    
    /// Called when entering this mode
    fn on_enter(&mut self, _state: &mut AppState) -> Result<(), String> {
        Ok(())
    }
    
    /// Called when exiting this mode
    fn on_exit(&mut self, _state: &mut AppState) -> Result<(), String> {
        Ok(())
    }
}


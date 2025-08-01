
use crossterm::event::KeyEvent;

use crate::{actions::Action, modes::interface::ModeBehavior, state::AppState, ui::UIComponents};

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
    fn render(&self, frame: &mut ratatui::Frame, state: &AppState) {
        // In search mode, render UI with current state (search will be active in state)
        UIComponents::render_complete_ui(frame, state);
    }
    
    fn on_enter(&mut self, state: &mut AppState) -> Result<(), String> {
        // When entering search mode, activate search if not already active
        if !state.search_active {
            state.search_active = true;
        }
        Ok(())
    }
    
    fn on_exit(&mut self, state: &mut AppState) -> Result<(), String> {
        // When exiting search mode, clear search state
        state.search_active = false;
        state.search_query.clear();
        Ok(())
    }
}

impl SearchMode {
    pub fn new() -> Self {
        Self {
        }
    }
}


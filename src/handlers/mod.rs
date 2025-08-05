pub mod interface;
mod explore;
mod search;

use crate::{handlers::{explore::ExploreHandler, interface::KeyHandler, search::SearchHandler}, messages::{AppMessage, SwitchAction}, state::AppState};
use crossterm::event::{KeyEvent};
use ratatui::Frame;

#[derive(Debug)]
pub enum Handler {
    Explore(ExploreHandler),
    Search(SearchHandler),
}

impl KeyHandler for Handler {
    /// Handle keyboard input - delegates to current handler
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> Option<AppMessage> {
        match self {
            Handler::Explore(explore_handler) => explore_handler.handle_key(key, state),
            Handler::Search(search_handler) => search_handler.handle_key(key, state)
        }
    }
}


impl Handler {
    pub fn new_explore_handler() -> Self {
        Handler::Explore(ExploreHandler::new())
    }
    
    pub fn new_search_handler() -> Self {
        Handler::Search(SearchHandler::new())
    }
    
    /// Render with handler awareness - provides handler context to UI
    pub fn render_with_handler_context(&self, frame: &mut Frame, state: &AppState) {
        use crate::ui::UI;
        UI::render_complete_ui(frame, state, self);
    }
    
    /// Switch from current handler to a new handler
    pub fn switch_to(&mut self, switch_action: SwitchAction, _state: &mut AppState) -> Result<(), String> {
        // Replace current handler with new handler
        *self = match switch_action {
            SwitchAction::EnterExploreMode => Self::new_explore_handler(),
            SwitchAction::EnterSearchMode => Self::new_search_handler(),
        };
        
        Ok(())
    }
}

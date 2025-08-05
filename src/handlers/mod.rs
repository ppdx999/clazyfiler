mod explore;
mod search;
mod fuzzy_find;

use crate::{handlers::{explore::ExploreHandler, search::SearchHandler, fuzzy_find::FuzzyFindHandler}, messages::AppMessage, state::AppState};
use crossterm::event::{KeyEvent};
use ratatui::Frame;

#[derive(Debug)]
pub enum Handler {
    Explore(ExploreHandler),
    Search(SearchHandler),
    FuzzyFind(FuzzyFindHandler),
}

impl Handler {
    pub fn new_explore_handler() -> Self {
        Handler::Explore(ExploreHandler::new())
    }
    
    pub fn new_search_handler() -> Self {
        Handler::Search(SearchHandler::new())
    }
    
    pub fn new_fuzzy_find_handler() -> Self {
        Handler::FuzzyFind(FuzzyFindHandler::new())
    }
    
    /// Handle keyboard input - delegates to current handler
    pub fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> Option<AppMessage> {
        match self {
            Handler::Explore(explore_handler) => explore_handler.handle_key(key, state),
            Handler::Search(search_handler) => search_handler.handle_key(key, state),
            Handler::FuzzyFind(fuzzy_find_handler) => fuzzy_find_handler.handle_key(key, state)
        }
    }
    
    /// Render with handler awareness - provides handler context to UI
    pub fn render_with_handler_context(&self, frame: &mut Frame, state: &AppState) {
        use crate::ui::UI;
        UI::render_complete_ui(frame, state, self);
    }
    
    /// Switch from current handler to a new handler
    pub fn switch_to(&mut self, message: &AppMessage, _state: &mut AppState) -> Result<(), String> {
        // Replace current handler with new handler
        *self = match message {
            AppMessage::SwitchToExploreHandler => Self::new_explore_handler(),
            AppMessage::SwitchToSearchHandler => Self::new_search_handler(),
            AppMessage::SwitchToFuzzyFindHandler => Self::new_fuzzy_find_handler(),
            _ => return Err("Invalid switch message".to_string()),
        };
        
        Ok(())
    }
}

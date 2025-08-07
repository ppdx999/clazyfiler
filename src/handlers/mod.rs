mod explore;
mod search;
mod fuzzy_find;

use crate::{handlers::{explore::ExploreHandler, search::SearchHandler, fuzzy_find::FuzzyFindHandler}, messages::AppMessage, model::AppModel};
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
    pub fn handle_key(&mut self, key: KeyEvent, model: &mut AppModel) -> Option<AppMessage> {
        match self {
            Handler::Explore(explore_handler) => explore_handler.handle_key(key, model),
            Handler::Search(search_handler) => search_handler.handle_key(key, model),
            Handler::FuzzyFind(fuzzy_find_handler) => fuzzy_find_handler.handle_key(key, model)
        }
    }
    
    /// Render with handler awareness - provides handler context to UI
    pub fn render_with_handler_context(&self, frame: &mut Frame, model: &AppModel) {
        use crate::ui::UI;
        UI::render_complete_ui(frame, model, self);
    }
    
    /// Switch from current handler to a new handler
    pub fn switch_to(&mut self, message: &AppMessage, _model: &mut AppModel) -> Result<(), String> {
        // Replace current handler with new handler
        *self = match message {
            AppMessage::SwitchToExploreHandler | AppMessage::SwitchToExploreHandlerKeepQuery => Self::new_explore_handler(),
            AppMessage::SwitchToSearchHandler => Self::new_search_handler(),
            AppMessage::SwitchToFuzzyFindHandler => Self::new_fuzzy_find_handler(),
            _ => return Err("Invalid switch message".to_string()),
        };
        
        Ok(())
    }
}

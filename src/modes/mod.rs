pub mod interface;
mod explore;
mod search;

use crate::{actions::Action, modes::{explore::ExploreMode, interface::ModeBehavior, search::SearchMode}};
use crossterm::event::KeyCode;
use ratatui::Frame;

#[derive(Debug)]
pub enum Mode {
    Explore(ExploreMode),
    Search(SearchMode),
}

impl ModeBehavior for Mode {
    /// Handle keyboard input - delegates to current mode without pattern matching hell
    fn handle_key(&self, key: KeyCode) -> Option<Action> {
        match self {
            Mode::Explore(explor_mode) => explor_mode.handle_key(key),
            Mode::Search(search_mode) => search_mode.handle_key(key)
        }
    }

    fn dispatch(&mut self, action: Action) -> Result<(), String> {
        match self {
            Mode::Explore(explore_mode) => explore_mode.dispatch(action),
            Mode::Search(search_mode) => search_mode.dispatch(action)
        }
    }

    /// Render current mode - delegates without ModeManager overhead
    fn render(&self, frame: &mut Frame) {
        match self {
            Mode::Explore(explore_mode) => explore_mode.render(frame),
            Mode::Search(search_mode) => search_mode.render(frame)
        }
    }
}


impl Mode {
    pub fn new_explore_mode() -> Self {
        Mode::Explore(ExploreMode::new())
    }
}

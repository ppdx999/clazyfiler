pub mod interface;
mod explore;
mod search;

use crate::{actions::Action, modes::{explore::ExploreMode, interface::ModeBehavior, search::SearchMode}, state::AppState};
use crossterm::event::{KeyEvent};
use ratatui::Frame;

#[derive(Debug)]
pub enum Mode {
    Explore(ExploreMode),
    Search(SearchMode),
}

impl ModeBehavior for Mode {
    /// Handle keyboard input - delegates to current mode without pattern matching hell
    fn handle_key(&self, key: KeyEvent, state: &AppState) -> Option<Action> {
        match self {
            Mode::Explore(explor_mode) => explor_mode.handle_key(key, state),
            Mode::Search(search_mode) => search_mode.handle_key(key, state)
        }
    }

    fn dispatch(&mut self, action: Action, state: &mut AppState) -> Result<(), String> {
        match self {
            Mode::Explore(explore_mode) => explore_mode.dispatch(action, state),
            Mode::Search(search_mode) => search_mode.dispatch(action, state)
        }
    }

    /// Render current mode - delegates without ModeManager overhead
    fn render(&self, frame: &mut Frame, state: &AppState) {
        match self {
            Mode::Explore(explore_mode) => explore_mode.render(frame, state),
            Mode::Search(search_mode) => search_mode.render(frame, state)
        }
    }
}


impl Mode {
    pub fn new_explore_mode() -> Self {
        Mode::Explore(ExploreMode::new())
    }
}

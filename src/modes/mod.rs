pub mod interface;
mod explore;
mod search;

use crate::{actions::{Action, ModeSwitchAction}, modes::{explore::ExploreMode, interface::ModeBehavior, search::SearchMode}, state::AppState};
use crossterm::event::{KeyEvent};
use ratatui::Frame;

#[derive(Debug)]
pub enum Mode {
    Explore(ExploreMode),
    Search(SearchMode),
}

impl ModeBehavior for Mode {
    /// Handle keyboard input - delegates to current mode without pattern matching hell
    fn handle_key(&self, key: KeyEvent, state: &AppState) -> Vec<Action> {
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
    
    fn on_enter(&mut self, state: &mut AppState) -> Result<(), String> {
        match self {
            Mode::Explore(explore_mode) => explore_mode.on_enter(state),
            Mode::Search(search_mode) => search_mode.on_enter(state)
        }
    }
    
    fn on_exit(&mut self, state: &mut AppState) -> Result<(), String> {
        match self {
            Mode::Explore(explore_mode) => explore_mode.on_exit(state),
            Mode::Search(search_mode) => search_mode.on_exit(state)
        }
    }
}


impl Mode {
    pub fn new_explore_mode() -> Self {
        Mode::Explore(ExploreMode::new())
    }
    
    pub fn new_search_mode() -> Self {
        Mode::Search(SearchMode::new())
    }
    
    /// Switch from current mode to a new mode, calling on_exit and on_enter appropriately
    pub fn switch_to(&mut self, switch_action: ModeSwitchAction, state: &mut AppState) -> Result<(), String> {
        // Call on_exit for current mode
        self.on_exit(state)?;
        
        // Replace current mode with new mode
        *self = match switch_action {
            ModeSwitchAction::EnterExploreMode => Self::new_explore_mode(),
            ModeSwitchAction::EnterSearchMode => Self::new_search_mode(),
        };
        
        // Call on_enter for new mode
        self.on_enter(state)?;
        
        Ok(())
    }
}

pub mod interface;
mod explore;
mod search;

use crate::{actions::ModeSwitchAction, modes::{explore::ExploreMode, interface::{ModeBehavior, ModeResult}, search::SearchMode}, state::AppState};
use crossterm::event::{KeyEvent};
use ratatui::Frame;

#[derive(Debug)]
pub enum Mode {
    Explore(ExploreMode),
    Search(SearchMode),
}

impl ModeBehavior for Mode {
    /// Handle keyboard input - delegates to current mode
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> ModeResult {
        match self {
            Mode::Explore(explore_mode) => explore_mode.handle_key(key, state),
            Mode::Search(search_mode) => search_mode.handle_key(key, state)
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
    
    /// Render with mode awareness - provides mode context to UI
    pub fn render_with_mode_context(&self, frame: &mut Frame, state: &AppState) {
        use crate::ui::UI;
        UI::render_complete_ui(frame, state, self);
    }
    
    /// Switch from current mode to a new mode
    pub fn switch_to(&mut self, switch_action: ModeSwitchAction, _state: &mut AppState) -> Result<(), String> {
        // Replace current mode with new mode
        *self = match switch_action {
            ModeSwitchAction::EnterExploreMode => Self::new_explore_mode(),
            ModeSwitchAction::EnterSearchMode => Self::new_search_mode(),
        };
        
        Ok(())
    }
}

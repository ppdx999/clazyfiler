pub mod explore;
pub mod search;

use crate::{actions::Action, config::Config, store::Store};
use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

/// Mode trait that encapsulates mode-specific behavior
/// This trait provides only abstract methods that all modes must implement
/// Each mode completely owns its rendering and behavior logic
pub trait Mode {
    /// Handle keyboard input for this mode
    fn handle_key(&self, key: KeyCode, config: &Config) -> Option<Action>;
    
    /// Handle mode entry (setup state)
    fn on_enter(&self, store: &mut Store);
    
    /// Handle mode exit (cleanup state) 
    fn on_exit(&self, store: &mut Store);
    
    /// Render the complete UI for this mode
    /// Each mode has full control over its layout and presentation
    fn render(&self, frame: &mut Frame, area: Rect, store: &Store);
}

/// Mode types for type-safe mode management
#[derive(Debug, Clone, PartialEq)]
pub enum ModeType {
    Explore,
    Search,
}

/// Mode manager that handles current mode and transitions
pub struct ModeManager {
    current_mode_type: ModeType,
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            current_mode_type: ModeType::Explore,
        }
    }
    
    pub fn current_mode_type(&self) -> &ModeType {
        &self.current_mode_type
    }
    
    pub fn get_current_mode(&self) -> Box<dyn Mode> {
        match self.current_mode_type {
            ModeType::Explore => Box::new(explore::ExploreMode),
            ModeType::Search => Box::new(search::SearchMode),
        }
    }
    
    pub fn transition_to(&mut self, new_mode: ModeType, store: &mut Store) {
        if self.current_mode_type == new_mode {
            return; // No transition needed
        }
        
        // Exit current mode
        self.get_current_mode().on_exit(store);
        
        // Transition to new mode
        self.current_mode_type = new_mode;
        
        // Enter new mode
        self.get_current_mode().on_enter(store);
    }
    
    pub fn handle_key(&self, key: KeyCode, config: &Config) -> Option<Action> {
        self.get_current_mode().handle_key(key, config)
    }
    
    pub fn render(&self, frame: &mut Frame, area: Rect, store: &Store) {
        match self.current_mode_type {
            ModeType::Explore => explore::ExploreMode.render(frame, area, store),
            ModeType::Search => search::SearchMode.render(frame, area, store),
        }
    }
}
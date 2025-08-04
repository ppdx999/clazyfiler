use crate::{actions::ModeSwitchAction, state::AppState};
use crossterm::event::KeyEvent;

/// Simplified key handler - handle keys directly with state, return only global actions
pub trait KeyHandler {
    /// Handle key input directly, returning global actions that need App-level processing
    /// Returns: (should_quit, should_open_file, mode_switch_request)
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> ModeResult;
}

/// Result of key handling - only global actions that need App-level processing
#[derive(Debug)]
pub struct ModeResult {
    pub quit: bool,
    pub open_file: bool,
    pub switch_mode: Option<ModeSwitchAction>,
    pub error: Option<String>,
}

impl Default for ModeResult {
    fn default() -> Self {
        Self {
            quit: false,
            open_file: false,
            switch_mode: None,
            error: None,
        }
    }
}

impl ModeResult {
    pub fn none() -> Self {
        Self::default()
    }
    
    pub fn quit() -> Self {
        Self { quit: true, ..Default::default() }
    }
    
    pub fn open_file() -> Self {
        Self { open_file: true, ..Default::default() }
    }
    
    pub fn switch_mode(action: ModeSwitchAction) -> Self {
        Self { switch_mode: Some(action), ..Default::default() }
    }
    
    pub fn error(message: String) -> Self {
        Self { error: Some(message), ..Default::default() }
    }
}


use crate::{messages::AppMessage, state::AppState};
use crossterm::event::KeyEvent;

/// Simplified key handler - handle keys directly with state, send messages to App
pub trait KeyHandler {
    /// Handle key input directly, returning messages for App-level processing
    fn handle_key(&mut self, key: KeyEvent, state: &mut AppState) -> Option<AppMessage>;
}


use crate::{store::Store, modes::ModeManager};
use ratatui::Frame;

/// View function - delegates complete rendering to the current mode
/// This follows the Strategy pattern - the UI layer doesn't know about mode-specific details
pub fn view(f: &mut Frame, store: &Store, mode_manager: &ModeManager) {
    // Complete delegation to mode-specific rendering
    mode_manager.render(f, f.area(), store);
}
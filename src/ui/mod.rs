mod file_detail;
mod file_list;
mod layout;
mod search_bar;

use ratatui::Frame;

use crate::{handlers::Handler, state::AppState};

// Re-export individual render functions for direct access if needed
pub use file_detail::render_file_description;
pub use file_list::render_file_list;
pub use layout::create_main_layout;
pub use search_bar::render_search_bar;

pub struct UI;

impl UI {
    /// Complete UI render function that orchestrates all components
    pub fn render_complete_ui(frame: &mut Frame, state: &AppState, handler: &Handler) {
        let area = frame.area();
        let (file_list_area, description_area, search_area) = create_main_layout(area);

        // Render all components using specific view states
        render_file_list(frame, file_list_area, &state.file_list_view, state.current_dir(), handler);
        render_file_description(frame, description_area, &state.file_detail_view);
        render_search_bar(frame, search_area, &state.search_input_view, handler);
    }
}
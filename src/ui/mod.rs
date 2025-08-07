mod file_detail;
mod file_list;
mod layout;
mod search_bar;

use ratatui::Frame;

use crate::{handlers::Handler, model::AppModel};

// Re-export individual render functions for direct access if needed
pub use file_detail::render_file_description;
pub use file_list::render_file_list;
pub use layout::create_main_layout;
pub use search_bar::render_search_bar;

pub struct UI;

impl UI {
    /// Complete UI render function that orchestrates all components
    pub fn render_complete_ui(frame: &mut Frame, model: &AppModel, handler: &Handler) {
        let area = frame.area();
        let (file_list_area, description_area, search_area) = create_main_layout(area);

        // Render all components directly with model - much simpler!
        render_file_list(frame, file_list_area, model, handler);
        render_file_description(frame, description_area, model);
        render_search_bar(frame, search_area, model, handler);
    }
}
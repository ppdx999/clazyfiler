use super::Mode;
use crate::{actions::Action, config::Config, store::Store, ui_helpers};
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::ListItem,
    Frame,
};

/// Explore mode - normal file browsing mode
pub struct ExploreMode;

impl Mode for ExploreMode {
    fn handle_key(&self, key: KeyCode, config: &Config) -> Option<Action> {
        let key_str = match key {
            KeyCode::Char(c) => c.to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Esc => "Escape".to_string(),
            KeyCode::F(5) => "F5".to_string(),
            _ => return None,
        };

        config.keymaps.get(&key_str).and_then(|action_str| {
            match action_str.as_str() {
                "quit" => Some(Action::Quit),
                "up" => Some(Action::MoveSelection(-1)),
                "down" => Some(Action::MoveSelection(1)),
                "select" => Some(Action::EnterDirectory),
                "back" => Some(Action::Back),
                "refresh" => Some(Action::Refresh),
                "search" => Some(Action::EnterSearchMode),
                _ => None,
            }
        })
    }

    fn render(&self, frame: &mut Frame, area: Rect, store: &Store) {
        // Create dual panel layout
        let chunks = ui_helpers::create_dual_panel_layout(area, store.config.ui.panel_width_ratio as u16);
        
        // Render file list in left panel
        let file_items = self.create_file_list(store);
        let title = format!("Files in {}", store.state.current_dir.display());
        ui_helpers::render_file_list(frame, chunks[0], &title, file_items, store.state.selected_index);
        
        // Render file details in right panel
        ui_helpers::render_file_details(frame, chunks[1], store);
    }

    fn on_enter(&self, _store: &mut Store) {
        // Nothing special needed when entering explore mode
    }

    fn on_exit(&self, _store: &mut Store) {
        // Nothing special needed when exiting explore mode
    }
}

// Internal helper methods for ExploreMode
impl ExploreMode {
    fn create_file_list(&self, store: &Store) -> Vec<ListItem> {
        store.state
            .files
            .iter()
            .enumerate()
            .map(|(i, file)| {
                let style = if i == store.state.selected_index {
                    Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                
                let prefix = if file.is_dir { "📁 " } else { "📄 " };
                ListItem::new(format!("{}{}", prefix, file.name)).style(style)
            })
            .collect()
    }
}
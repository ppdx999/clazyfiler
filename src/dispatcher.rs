use crate::{actions::Action, config::Config, store::Store};
use crossterm::event::KeyCode;
use std::env;

// Dispatcher (handles actions and updates store) - Flux Pattern
pub struct Dispatcher {
    store: Store,
}

impl Dispatcher {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let store = Store::new()?;
        Ok(Dispatcher { store })
    }

    // Dispatcher - single point for all state changes (Flux Pattern)
    pub fn dispatch(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::MoveSelection(direction) => {
                self.store.move_selection(direction);
                Ok(())
            }
            Action::EnterDirectory => {
                self.store.enter_directory()
            }
            Action::Back => {
                self.store.go_back()
            }
            Action::LoadDirectory(path) => {
                match env::set_current_dir(&path) {
                    Ok(()) => {
                        self.store.state.current_dir = path;
                        match self.store.load_files() {
                            Ok(()) => Ok(()),
                            Err(e) => Err(format!("Failed to load directory: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Failed to change directory: {}", e)),
                }
            }
            Action::Refresh => {
                match self.store.load_files() {
                    Ok(()) => Ok(()),
                    Err(e) => Err(format!("Failed to refresh: {}", e)),
                }
            }
            Action::EnterSearchMode => {
                self.store.enter_search_mode();
                Ok(())
            }
            Action::ExitSearchMode => {
                self.store.exit_search_mode();
                Ok(())
            }
            Action::UpdateSearchQuery(query) => {
                self.store.handle_search_input(query);
                Ok(())
            }
            Action::SearchSelectFirst => {
                self.store.select_first_match();
                Ok(())
            }
            Action::Quit => Ok(()),
        }
    }

    // Delegate methods to access store state
    pub fn get_store(&self) -> &Store {
        &self.store
    }
}

// Action Creator - converts keyboard input to actions
pub fn key_to_action(key: KeyCode, config: &Config) -> Option<Action> {
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
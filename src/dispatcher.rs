use crate::{actions::Action, config::Config, store::Store, modes::{ModeManager, ModeType}};
use crossterm::event::KeyCode;
use std::env;

// Dispatcher (handles actions and updates store) - Flux Pattern
pub struct Dispatcher {
    store: Store,
    mode_manager: ModeManager,
}

impl Dispatcher {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let store = Store::new()?;
        let mode_manager = ModeManager::new();
        Ok(Dispatcher { store, mode_manager })
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
                self.mode_manager.transition_to(ModeType::Search, &mut self.store);
                Ok(())
            }
            Action::ExitSearchMode => {
                self.mode_manager.transition_to(ModeType::Explore, &mut self.store);
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
    
    pub fn get_mode_manager(&self) -> &ModeManager {
        &self.mode_manager
    }
}

// Action Creator - converts keyboard input to actions using mode-specific handling
pub fn key_to_action(key: KeyCode, config: &Config, mode_manager: &ModeManager) -> Option<Action> {
    // Delegate to current mode for key handling
    mode_manager.handle_key(key, config)
}
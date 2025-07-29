use crate::config::Config;
use std::{env, fs, io, path::PathBuf, time::SystemTime};

// Store (holds application state)
#[derive(Debug)]
pub struct Store {
    pub state: AppState,
    pub config: Config,
}

#[derive(Debug)]
pub struct AppState {
    pub current_dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub selected_index: usize,
    pub search_mode: bool,
    pub search_query: String,
    pub filtered_files: Vec<usize>,
}

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub is_dir: bool,
    pub modified: Option<SystemTime>,
}

impl Store {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let current_dir = env::current_dir()?;
        let config = Config::load()?;
        let mut store = Store {
            state: AppState {
                current_dir: current_dir.clone(),
                files: Vec::new(),
                selected_index: 0,
                search_mode: false,
                search_query: String::new(),
                filtered_files: Vec::new(),
            },
            config,
        };
        store.load_files()?;
        Ok(store)
    }

    // Private methods for state manipulation
    pub fn move_selection(&mut self, direction: isize) {
        let max_items = if self.state.search_mode {
            if self.state.filtered_files.is_empty() {
                self.state.selected_index = 0;
                return;
            }
            self.state.filtered_files.len()
        } else {
            if self.state.files.is_empty() {
                self.state.selected_index = 0;
                return;
            }
            self.state.files.len()
        };

        let new_index = (self.state.selected_index as isize + direction)
            .max(0)
            .min(max_items.saturating_sub(1) as isize) as usize;
        
        self.state.selected_index = new_index;
    }

    pub fn enter_directory(&mut self) -> Result<(), String> {
        if let Some(selected_file) = self.get_selected_file() {
            if selected_file.is_dir {
                match env::set_current_dir(&selected_file.path) {
                    Ok(()) => {
                        self.state.current_dir = selected_file.path.clone();
                        match self.load_files() {
                            Ok(()) => Ok(()),
                            Err(e) => Err(format!("Failed to load directory contents: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Failed to enter directory: {}", e)),
                }
            } else {
                Err("Selected item is not a directory".to_string())
            }
        } else {
            Err("No file selected".to_string())
        }
    }

    pub fn go_back(&mut self) -> Result<(), String> {
        let current_path = self.state.current_dir.clone();
        if let Some(parent) = current_path.parent() {
            match env::set_current_dir(parent) {
                Ok(()) => {
                    self.state.current_dir = parent.to_path_buf();
                    match self.load_files() {
                        Ok(()) => Ok(()),
                        Err(e) => Err(format!("Failed to load parent directory: {}", e)),
                    }
                }
                Err(e) => Err(format!("Failed to go back: {}", e)),
            }
        } else {
            Err("Already at the root directory".to_string())
        }
    }

    pub fn load_files(&mut self) -> io::Result<()> {
        self.state.files.clear();
        self.state.selected_index = 0;
        // Reset search state when loading new directory  
        if self.state.search_mode {
            self.state.search_mode = false;
            self.state.search_query.clear();
            self.state.filtered_files.clear();
        }
        let entries = fs::read_dir(&self.state.current_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            let file_entry = FileEntry {
                name: file_name,
                path: path.clone(),
                size: metadata.len(),
                is_dir: metadata.is_dir(),
                modified: metadata.modified().ok(),
            };
            
            self.state.files.push(file_entry);
        }
        
        self.state.files.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(())
    }

    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        if self.state.search_mode && !self.state.filtered_files.is_empty() {
            let filtered_index = self.state.selected_index;
            if filtered_index < self.state.filtered_files.len() {
                let actual_index = self.state.filtered_files[filtered_index];
                self.state.files.get(actual_index)
            } else {
                None
            }
        } else if self.state.search_mode {
            None
        } else {
            self.state.files.get(self.state.selected_index)
        }
    }

    pub fn enter_search_mode(&mut self) {
        self.state.search_mode = true;
        self.state.search_query.clear();
        self.state.selected_index = 0;
        self.update_search_filter();
    }

    pub fn exit_search_mode(&mut self) {
        self.state.search_mode = false;
        self.state.search_query.clear();
        self.state.filtered_files.clear();
        // Ensure selected_index is valid for the full file list
        if !self.state.files.is_empty() {
            self.state.selected_index = self.state.selected_index.min(self.state.files.len() - 1);
        } else {
            self.state.selected_index = 0;
        }
    }

    pub fn handle_search_input(&mut self, input: String) {
        if input.is_empty() {
            // Backspace - remove last character
            if !self.state.search_query.is_empty() {
                self.state.search_query.pop();
            }
        } else {
            // Add character
            self.state.search_query.push_str(&input);
        }
        self.update_search_filter();
        // Always reset to first item when search changes
        self.state.selected_index = 0;
    }

    fn update_search_filter(&mut self) {
        self.state.filtered_files.clear();
        if self.state.search_query.is_empty() {
            return;
        }

        let query_lower = self.state.search_query.to_lowercase();
        for (index, file) in self.state.files.iter().enumerate() {
            if file.name.to_lowercase().contains(&query_lower) {
                self.state.filtered_files.push(index);
            }
        }
    }

    pub fn select_first_match(&mut self) {
        if self.state.search_mode && !self.state.filtered_files.is_empty() {
            let filtered_index = self.state.selected_index;
            if filtered_index < self.state.filtered_files.len() {
                let actual_file_index = self.state.filtered_files[filtered_index];
                if actual_file_index < self.state.files.len() {
                    // Exit search mode and set selection to the matched file
                    self.state.search_mode = false;
                    self.state.search_query.clear();
                    self.state.filtered_files.clear();
                    self.state.selected_index = actual_file_index;
                }
            }
        }
    }
}
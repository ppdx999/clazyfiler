use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
}

#[derive(Debug)]
pub struct AppState {
    pub current_dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub selected_index: usize,
    pub search_query: String,
    pub search_active: bool,
}

impl AppState {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut state = Self {
            current_dir: current_dir.clone(),
            files: Vec::new(),
            selected_index: 0,
            search_query: String::new(),
            search_active: false,
        };
        
        state.refresh_files();
        state
    }

    pub fn refresh_files(&mut self) {
        self.files.clear();
        
        if let Ok(entries) = fs::read_dir(&self.current_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    let file_entry = FileEntry {
                        name: entry.file_name().to_string_lossy().to_string(),
                        path: entry.path(),
                        is_directory: metadata.is_dir(),
                        size: if metadata.is_file() { Some(metadata.len()) } else { None },
                        modified: metadata.modified().ok(),
                    };
                    self.files.push(file_entry);
                }
            }
        }
        
        // Sort: directories first, then files, both alphabetically
        self.files.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });
        
        // Reset selection if out of bounds
        if self.selected_index >= self.files.len() && !self.files.is_empty() {
            self.selected_index = self.files.len() - 1;
        }
    }

    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        self.files.get(self.selected_index)
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.files.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn enter_directory(&mut self) -> Result<(), String> {
        if let Some(selected) = self.get_selected_file() {
            if selected.is_directory {
                self.current_dir = selected.path.clone();
                self.selected_index = 0;
                self.refresh_files();
                Ok(())
            } else {
                Err("Selected item is not a directory".to_string())
            }
        } else {
            Err("No file selected".to_string())
        }
    }

    pub fn go_to_parent(&mut self) -> Result<(), String> {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.selected_index = 0;
            self.refresh_files();
            Ok(())
        } else {
            Err("Already at root directory".to_string())
        }
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
    }

    pub fn append_search_query(&mut self, c: char) {
        self.search_query.push(c);
    }

    pub fn backspace_search_query(&mut self) {
        self.search_query.pop();
    }

    pub fn toggle_search(&mut self) {
        self.search_active = !self.search_active;
        if !self.search_active {
            self.search_query.clear();
        }
    }

    pub fn get_filtered_files(&self) -> Vec<&FileEntry> {
        if self.search_query.is_empty() {
            self.files.iter().collect()
        } else {
            self.files
                .iter()
                .filter(|file| {
                    file.name.to_lowercase().contains(&self.search_query.to_lowercase())
                })
                .collect()
        }
    }
}

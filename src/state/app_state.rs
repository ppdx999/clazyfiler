use std::path::PathBuf;
use std::time::SystemTime;
use crate::core::Result;
use crate::services::EditorService;
use crate::state::{NavigationState, SearchState};

/// File entry information
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified: Option<SystemTime>,
}

/// Main application state - now focused and modular
#[derive(Debug)]
pub struct AppState {
    pub navigation: NavigationState,
    pub search: SearchState,
    editor_service: EditorService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            navigation: NavigationState::new(),
            search: SearchState::new(),
            editor_service: EditorService::new(),
        }
    }

    /// Refresh files and update search filtering
    pub fn refresh_files(&mut self) {
        if let Err(e) = self.navigation.refresh_files() {
            eprintln!("Failed to refresh files: {}", e);
            return;
        }

        // Update search filtering after refresh
        self.update_search_filtering();
        
        // Clear search after directory navigation
        self.clear_search_query();
    }

    /// Update search filtering based on current files
    fn update_search_filtering(&mut self) {
        let total_files = self.navigation.files_len();
        self.search.update_with_file_names(total_files, |i| {
            self.navigation.get_file(i).map(|f| f.name.clone())
        });
    }

    // Navigation delegation methods
    pub fn move_selection_up(&mut self) {
        if self.search.is_active() {
            self.search.move_selection_up();
        } else {
            self.navigation.move_selection_up();
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.search.is_active() {
            self.search.move_selection_down();
        } else {
            self.navigation.move_selection_down();
        }
    }

    pub fn enter_directory(&mut self) -> Result<()> {
        let result = self.navigation.enter_directory();
        if result.is_ok() {
            self.update_search_filtering();
            self.clear_search_query();
        }
        result
    }

    pub fn go_to_parent(&mut self) -> Result<()> {
        let result = self.navigation.go_to_parent();
        if result.is_ok() {
            self.update_search_filtering();
            self.clear_search_query();
        }
        result
    }

    /// Get currently selected file (considering search filtering)
    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        if self.search.is_active() {
            if let Some(file_index) = self.search.get_selected_file_index() {
                self.navigation.get_file(file_index)
            } else {
                None
            }
        } else {
            self.navigation.get_selected_file()
        }
    }

    /// Get file by filtered index
    pub fn get_filtered_file(&self, filtered_index: usize) -> Option<&FileEntry> {
        if self.search.is_active() {
            if let Some(file_index) = self.search.get_file_index(filtered_index) {
                self.navigation.get_file(file_index)
            } else {
                None
            }
        } else {
            self.navigation.get_file(filtered_index)
        }
    }

    /// Get number of filtered files
    pub fn filtered_files_len(&self) -> usize {
        if self.search.is_active() {
            self.search.filtered_count()
        } else {
            self.navigation.files_len()
        }
    }

    /// Get selected index (in filtered results)
    pub fn selected_index(&self) -> usize {
        if self.search.is_active() {
            self.search.selected_index
        } else {
            self.navigation.selected_index
        }
    }

    /// Get current directory
    pub fn current_dir(&self) -> &PathBuf {
        &self.navigation.current_dir
    }

    // Search delegation methods
    pub fn search_query(&self) -> &str {
        &self.search.query
    }

    pub fn set_search_query(&mut self, query: String) {
        self.search.set_query(query, self.navigation.files_len());
    }

    pub fn append_search_query(&mut self, c: char) {
        self.search.append_to_query(c, self.navigation.files_len());
        self.update_search_filtering();
    }

    pub fn pop_search_query(&mut self) {
        self.search.pop_from_query(self.navigation.files_len());
        self.update_search_filtering();
    }

    pub fn clear_search_query(&mut self) {
        self.search.clear_query(self.navigation.files_len());
        self.update_search_filtering();
    }

    pub fn delete_word_backward(&mut self) {
        self.search.delete_word_backward(self.navigation.files_len());
        self.update_search_filtering();
    }

    pub fn delete_to_end(&mut self) {
        self.search.delete_to_end(self.navigation.files_len());
        self.update_search_filtering();
    }

    // Editor service delegation
    pub fn open_file_with_vim(&mut self, file: &FileEntry) -> Result<()> {
        self.editor_service.open_file(file)
    }

    /// Get file content for display
    pub fn read_file_content(&self, file: &FileEntry) -> String {
        match self.navigation.get_file_content(file) {
            Ok(content) => content,
            Err(e) => format!("❌ Error reading file: {}", e),
        }
    }
}
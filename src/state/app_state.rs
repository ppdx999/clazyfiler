use std::path::PathBuf;
use crate::core::Result;
use crate::services::{EditorService, FileService};
use crate::state::{NavigationState, SearchState, FuzzyFindState};

/// File entry information
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: Option<u64>,
}

/// Main application state - now focused and modular
#[derive(Debug)]
pub struct AppState {
    pub navigation: NavigationState,
    pub search: SearchState,
    pub fuzzy_find: FuzzyFindState,
    editor_service: EditorService,
    file_service: FileService,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            navigation: NavigationState::new(),
            search: SearchState::new(),
            fuzzy_find: FuzzyFindState::new(),
            editor_service: EditorService::new(),
            file_service: FileService::new(),
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
    pub fn open_selected_file_with_editor(&mut self) -> Result<()> {
        // Get the selected file from appropriate state
        let selected_file = if self.fuzzy_find.is_active() {
            match self.fuzzy_find.get_selected_file() {
                Some(file) => file.clone(),
                None => return Err(crate::core::ClazyfilerError::editor("selection", "No file selected in fuzzy find")),
            }
        } else {
            match self.get_selected_file() {
                Some(file) => file.clone(),
                None => return Err(crate::core::ClazyfilerError::editor("selection", "No file selected")),
            }
        };
        
        // Check if it's a directory
        if selected_file.is_directory {
            return Err(crate::core::ClazyfilerError::editor("editor", "Cannot open directory with editor"));
        }
        
        // Open the file with editor
        let result = self.editor_service.open_file(&selected_file);
        
        // Refresh files after editor operation regardless of result
        self.refresh_files();
        
        result
    }

    /// Navigate to a specific directory (used from fuzzy find)
    pub fn navigate_to_directory(&mut self, path: std::path::PathBuf) -> Result<()> {
        // Clear fuzzy find state when navigating to a new directory
        self.fuzzy_find.clear();
        self.navigation.change_directory(path)
    }

    /// Clear fuzzy find state (used when exiting fuzzy find mode)
    pub fn clear_fuzzy_find_state(&mut self) {
        self.fuzzy_find.clear();
    }

    /// Start fuzzy finding from current directory
    pub fn start_fuzzy_find(&mut self) -> Result<()> {
        let root_dir = self.navigation.current_dir.clone();
        self.fuzzy_find.start_indexing(&root_dir);
        
        // Start recursive file indexing in background
        match self.file_service.scan_directory_tree(&root_dir) {
            Ok(files) => {
                self.fuzzy_find.add_files(files);
                self.fuzzy_find.finish_indexing();
                Ok(())
            }
            Err(e) => {
                self.fuzzy_find.finish_indexing();
                Err(e)
            }
        }
    }

    /// Get file content for display
    pub fn read_file_content(&self, file: &FileEntry) -> String {
        match self.navigation.get_file_content(file) {
            Ok(content) => content,
            Err(e) => format!("❌ Error reading file: {}", e),
        }
    }
}
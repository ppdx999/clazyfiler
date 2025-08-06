use std::path::PathBuf;
use crate::core::Result;
use crate::services::{EditorService, FileService};
use crate::state::{
    SearchInputViewState, FileListViewState, FileDetailViewState,
    NavigationData, FuzzyFindData
};

/// File entry information
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: Option<u64>,
}

/// Main application state - view-centric architecture
#[derive(Debug)]
pub struct AppState {
    // View states (UI components)
    pub search_input_view: SearchInputViewState,
    pub file_list_view: FileListViewState,
    pub file_detail_view: FileDetailViewState,
    
    // Data sources
    pub navigation_data: NavigationData,
    pub fuzzy_find_data: FuzzyFindData,
    
    // Services
    editor_service: EditorService,
    file_service: FileService,
}

impl AppState {
    pub fn new() -> Self {
        let mut state = Self {
            // Initialize view states
            search_input_view: SearchInputViewState::new(),
            file_list_view: FileListViewState::new(),
            file_detail_view: FileDetailViewState::new(),
            
            // Initialize data sources
            navigation_data: NavigationData::new(),
            fuzzy_find_data: FuzzyFindData::new(),
            
            // Initialize services
            editor_service: EditorService::new(),
            file_service: FileService::new(),
        };
        
        // Initial directory load
        state.load_current_directory();
        
        state
    }

    /// Load current directory and update views
    fn load_current_directory(&mut self) {
        let current_dir = self.navigation_data.current_dir.clone();
        if let Ok(files) = self.file_service.read_directory(&current_dir) {
            self.navigation_data.update(current_dir.clone(), files);
            self.update_explore_view();
        } else {
            eprintln!("Warning: Failed to load initial directory");
        }
    }

    /// Update file list view for explore mode (shows current directory files)
    pub fn update_explore_view(&mut self) {
        let title = format!("Files - {}", self.navigation_data.current_dir.display());
        let files = if self.search_input_view.is_active() {
            self.navigation_data.filter_files(&self.search_input_view.query)
        } else {
            self.navigation_data.files.clone()
        };
        
        self.file_list_view.set_files(files, title);
        self.update_file_detail_view();
    }

    /// Update file list view for search mode (filtered current directory files)
    pub fn update_search_view(&mut self) {
        let title = format!("Search - {}", self.navigation_data.current_dir.display());
        let files = self.navigation_data.filter_files(&self.search_input_view.query);
        
        self.file_list_view.set_files(files, title);
        self.update_file_detail_view();
    }

    /// Update file list view for fuzzy find mode (fuzzy matched files)
    pub fn update_fuzzy_find_view(&mut self) {
        let title = if self.fuzzy_find_data.is_indexing {
            format!("🔍 Fuzzy Find - Indexing... ({} files)", self.fuzzy_find_data.total_count())
        } else {
            format!("🔍 Fuzzy Find - {} total files", self.fuzzy_find_data.total_count())
        };
        
        let files = self.fuzzy_find_data.fuzzy_filter_files(&self.search_input_view.query);
        self.file_list_view.set_files(files, title);
        self.update_file_detail_view();
    }

    /// Update file detail view based on currently selected file
    fn update_file_detail_view(&mut self) {
        if let Some(selected_file) = self.file_list_view.get_selected_file() {
            let title = if selected_file.is_directory {
                format!("📁 {}", selected_file.name)
            } else {
                format!("📄 {}", selected_file.name)
            };
            let content = match self.file_service.read_file_content(selected_file) {
                Ok(content) => content,
                Err(e) => format!("❌ Error reading file: {}", e),
            };
            self.file_detail_view.set_file_details(title, content);
        } else {
            self.file_detail_view.clear();
        }
    }

    /// Navigation actions
    pub fn enter_directory(&mut self) -> Result<()> {
        if let Some(selected_file) = self.file_list_view.get_selected_file() {
            if selected_file.is_directory {
                let new_dir = selected_file.path.clone();
                self.change_directory(new_dir)?;
            }
        }
        Ok(())
    }

    pub fn go_to_parent(&mut self) -> Result<()> {
        if let Some(parent) = self.navigation_data.current_dir.parent() {
            self.change_directory(parent.to_path_buf())?;
        }
        Ok(())
    }

    pub fn change_directory(&mut self, new_dir: PathBuf) -> Result<()> {
        let files = self.file_service.read_directory(&new_dir)?;
        
        self.navigation_data.update(new_dir, files);
        self.search_input_view.clear_query(); // Clear search when changing directories
        self.fuzzy_find_data.clear(); // Clear fuzzy find data
        self.update_explore_view();
        Ok(())
    }

    pub fn refresh_files(&mut self) {
        let current_dir = self.navigation_data.current_dir.clone();
        if let Ok(files) = self.file_service.read_directory(&current_dir) {
            self.navigation_data.update(current_dir, files);
            self.update_explore_view();
        }
    }

    /// File list navigation
    pub fn move_selection_up(&mut self) {
        self.file_list_view.move_selection_up();
        self.update_file_detail_view();
    }

    pub fn move_selection_down(&mut self) {
        self.file_list_view.move_selection_down();
        self.update_file_detail_view();
    }

    /// Get currently selected file
    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        self.file_list_view.get_selected_file()
    }

    /// Get file by filtered index
    pub fn get_filtered_file(&self, index: usize) -> Option<&FileEntry> {
        self.file_list_view.get_file(index)
    }

    /// Get number of filtered files
    pub fn filtered_files_len(&self) -> usize {
        self.file_list_view.files_len()
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.file_list_view.selected_index
    }

    /// Get current directory
    pub fn current_dir(&self) -> &PathBuf {
        &self.navigation_data.current_dir
    }

    /// Search input methods (unified for search and fuzzy find modes)
    pub fn search_query(&self) -> &str {
        &self.search_input_view.query
    }

    pub fn append_search_query(&mut self, c: char) {
        self.search_input_view.append_to_query(c);
    }

    pub fn pop_search_query(&mut self) {
        self.search_input_view.pop_from_query();
    }

    pub fn clear_search_query(&mut self) {
        self.search_input_view.clear_query();
    }

    pub fn delete_word_backward(&mut self) {
        self.search_input_view.delete_word_backward();
    }

    pub fn delete_to_end(&mut self) {
        self.search_input_view.delete_to_end();
    }

    /// Editor service delegation  
    pub fn open_selected_file_with_editor(&mut self) -> Result<()> {
        let selected_file = match self.get_selected_file() {
            Some(file) => file.clone(),
            None => return Err(crate::core::ClazyfilerError::editor("selection", "No file selected")),
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


    /// Start fuzzy finding from current directory
    pub fn start_fuzzy_find(&mut self) -> Result<()> {
        let root_dir = self.navigation_data.current_dir.clone();
        self.fuzzy_find_data.start_indexing();
        
        // Start recursive file indexing in background
        match self.file_service.scan_directory_tree(&root_dir) {
            Ok(files) => {
                self.fuzzy_find_data.add_files(files);
                self.fuzzy_find_data.finish_indexing();
                Ok(())
            }
            Err(e) => {
                self.fuzzy_find_data.finish_indexing();
                Err(e)
            }
        }
    }
}
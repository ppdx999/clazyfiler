use std::path::PathBuf;
use crate::core::Result;
use crate::services::{EditorService, FileService};

/// File entry information
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub size: Option<u64>,
}

/// Application mode determines how files are sourced and displayed
#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Explore,    // Browse current directory
    Search,     // Search within current directory  
    FuzzyFind,  // Fuzzy search across directory tree
}

/// Source of files currently being displayed
#[derive(Debug, Clone, PartialEq)]
pub enum FilesSource {
    CurrentDir,      // Files from current directory
    SearchResults,   // Filtered files from current directory
    FuzzyResults,    // Fuzzy-matched files from recursive scan
}

/// Core application model - single source of truth
/// Contains only essential state, everything else derives from these 4 fields
#[derive(Debug)]
pub struct AppModel {
    // The 4 essential fields
    pub current_dir: PathBuf,
    pub query_text: String,
    pub files: Vec<FileEntry>,           // Currently displayed files (filtered)
    pub selected_index: usize,
    
    // Context metadata
    pub mode: AppMode,
    pub files_source: FilesSource,
    
    // Source data for filtering
    pub directory_files: Vec<FileEntry>, // Original unfiltered directory files
    
    // Background state for fuzzy find
    pub all_files_cache: Vec<FileEntry>,  // All files from recursive scan
    pub is_indexing: bool,                // Whether fuzzy find is still scanning
    
    // Services
    file_service: FileService,
    editor_service: EditorService,
}

impl AppModel {
    pub fn new() -> Result<Self> {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let file_service = FileService::new();
        let editor_service = EditorService::new();
        
        // Load initial directory
        let directory_files = file_service.read_directory(&current_dir)?;
        
        Ok(Self {
            current_dir,
            query_text: String::new(),
            files: directory_files.clone(),      // Initially same as directory files
            selected_index: 0,
            mode: AppMode::Explore,
            files_source: FilesSource::CurrentDir,
            directory_files,                     // Store original files for filtering
            all_files_cache: Vec::new(),
            is_indexing: false,
            file_service,
            editor_service,
        })
    }
    
    /// Get currently selected file
    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        self.files.get(self.selected_index)
    }
    
    /// Update query text and refresh files based on current mode
    pub fn update_query(&mut self, new_query: String) {
        self.query_text = new_query;
        self.refresh_files_for_current_mode();
    }
    
    /// Append character to query
    pub fn append_to_query(&mut self, c: char) {
        self.query_text.push(c);
        self.refresh_files_for_current_mode();
    }
    
    /// Remove last character from query
    pub fn pop_from_query(&mut self) {
        self.query_text.pop();
        self.refresh_files_for_current_mode();
    }
    
    /// Clear query text
    pub fn clear_query(&mut self) {
        self.query_text.clear();
        self.refresh_files_for_current_mode();
    }
    
    /// Delete word backward (Ctrl+W)
    pub fn delete_word_backward(&mut self) {
        if let Some(pos) = self.query_text.rfind(' ') {
            self.query_text.truncate(pos);
        } else {
            self.query_text.clear();
        }
        self.refresh_files_for_current_mode();
    }
    
    /// Delete to end (Ctrl+K)
    pub fn delete_to_end(&mut self) {
        self.query_text.clear();
        self.refresh_files_for_current_mode();
    }
    
    /// Move selection up
    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }
    
    /// Move selection down
    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.files.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }
    
    /// Change directory and update files
    pub fn change_directory(&mut self, new_dir: PathBuf) -> Result<()> {
        let directory_files = self.file_service.read_directory(&new_dir)?;
        
        self.current_dir = new_dir;
        self.directory_files = directory_files.clone();
        self.files = directory_files;  // Initially show all files
        self.selected_index = 0;
        self.query_text.clear(); // Clear query when changing directories
        self.mode = AppMode::Explore;
        self.files_source = FilesSource::CurrentDir;
        self.all_files_cache.clear(); // Clear fuzzy find cache
        self.is_indexing = false;
        
        Ok(())
    }
    
    /// Navigate to parent directory
    pub fn go_to_parent(&mut self) -> Result<()> {
        if let Some(parent) = self.current_dir.parent() {
            self.change_directory(parent.to_path_buf())
        } else {
            Ok(())
        }
    }
    
    /// Enter selected directory
    pub fn enter_selected_directory(&mut self) -> Result<()> {
        if let Some(selected_file) = self.get_selected_file() {
            if selected_file.is_directory {
                let path = selected_file.path.clone();
                self.change_directory(path)?;
            }
        }
        Ok(())
    }
    
    /// Switch to explore mode
    pub fn switch_to_explore_mode(&mut self) {
        self.mode = AppMode::Explore;
        self.query_text.clear();
        self.refresh_files_for_current_mode();
    }
    
    /// Switch to explore mode but keep current query (for maintaining search results)
    pub fn switch_to_explore_mode_keep_query(&mut self) {
        self.mode = AppMode::Explore;
        // Don't clear query_text - keep the current search results
        self.refresh_files_for_current_mode();
    }
    
    /// Switch to search mode
    pub fn switch_to_search_mode(&mut self) {
        self.mode = AppMode::Search;
        self.refresh_files_for_current_mode();
    }
    
    /// Switch to fuzzy find mode and start indexing
    pub fn switch_to_fuzzy_find_mode(&mut self) -> Result<()> {
        self.mode = AppMode::FuzzyFind;
        self.start_fuzzy_indexing()?;
        Ok(())
    }
    
    /// Start fuzzy find indexing
    fn start_fuzzy_indexing(&mut self) -> Result<()> {
        self.is_indexing = true;
        self.all_files_cache.clear();
        
        // Perform recursive scan
        match self.file_service.scan_directory_tree(&self.current_dir) {
            Ok(all_files) => {
                self.all_files_cache = all_files;
                self.is_indexing = false;
                self.refresh_files_for_current_mode();
                Ok(())
            }
            Err(e) => {
                self.is_indexing = false;
                Err(e)
            }
        }
    }
    
    /// Refresh files based on current mode and query
    fn refresh_files_for_current_mode(&mut self) {
        match self.mode {
            AppMode::Explore => {
                // Use stored directory files, optionally filtered by query
                if self.query_text.is_empty() {
                    self.files = self.directory_files.clone();
                    self.files_source = FilesSource::CurrentDir;
                } else {
                    self.files = self.filter_files(&self.directory_files, &self.query_text);
                    self.files_source = FilesSource::SearchResults;
                }
            }
            AppMode::Search => {
                // Filter current directory files by query
                self.files = self.filter_files(&self.directory_files, &self.query_text);
                self.files_source = FilesSource::SearchResults;
            }
            AppMode::FuzzyFind => {
                // Fuzzy filter cached files
                self.files = self.fuzzy_filter_files(&self.all_files_cache, &self.query_text);
                self.files_source = FilesSource::FuzzyResults;
            }
        }
        
        // Reset selection if out of bounds
        if self.selected_index >= self.files.len() && !self.files.is_empty() {
            self.selected_index = self.files.len() - 1;
        } else if self.files.is_empty() {
            self.selected_index = 0;
        }
    }
    
    /// Simple text filtering for search mode
    fn filter_files(&self, files: &[FileEntry], query: &str) -> Vec<FileEntry> {
        if query.is_empty() {
            return files.to_vec();
        }
        
        files
            .iter()
            .filter(|file| file.name.to_lowercase().contains(&query.to_lowercase()))
            .cloned()
            .collect()
    }
    
    /// Fuzzy filtering with scoring for fuzzy find mode
    fn fuzzy_filter_files(&self, files: &[FileEntry], query: &str) -> Vec<FileEntry> {
        if query.is_empty() {
            return files.to_vec();
        }
        
        let mut matches: Vec<(FileEntry, i32)> = files
            .iter()
            .filter_map(|file| {
                let score = self.fuzzy_match(&file.path.to_string_lossy(), query);
                if score > 0 {
                    Some((file.clone(), score))
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by score (higher is better)
        matches.sort_by(|a, b| b.1.cmp(&a.1));
        matches.into_iter().map(|(file, _)| file).collect()
    }
    
    /// Fuzzy matching algorithm
    fn fuzzy_match(&self, text: &str, pattern: &str) -> i32 {
        let text = text.to_lowercase();
        let pattern = pattern.to_lowercase();
        
        if pattern.is_empty() {
            return 100;
        }

        let mut score: i32 = 0;
        let text_chars = text.chars().collect::<Vec<_>>();
        let pattern_chars = pattern.chars().collect::<Vec<_>>();
        
        let mut text_idx = 0;
        let mut pattern_idx = 0;
        let mut consecutive_matches = 0;

        while text_idx < text_chars.len() && pattern_idx < pattern_chars.len() {
            if text_chars[text_idx] == pattern_chars[pattern_idx] {
                score += 10 + consecutive_matches;
                consecutive_matches += 1;
                pattern_idx += 1;
            } else {
                consecutive_matches = 0;
            }
            text_idx += 1;
        }

        if pattern_idx == pattern_chars.len() {
            let path_depth_penalty = text.matches('/').count() as i32;
            score = score.saturating_sub(path_depth_penalty);
            
            if text.contains(&pattern) {
                score += 50;
            }
            
            score.max(1)
        } else {
            0
        }
    }
    
    /// Open selected file with editor
    pub fn open_selected_file_with_editor(&mut self) -> Result<()> {
        let selected_file = match self.get_selected_file() {
            Some(file) => file.clone(),
            None => return Err(crate::core::ClazyfilerError::editor("selection", "No file selected")),
        };
        
        if selected_file.is_directory {
            return Err(crate::core::ClazyfilerError::editor("editor", "Cannot open directory with editor"));
        }
        
        let result = self.editor_service.open_file(&selected_file);
        
        // Refresh files after editor operation
        self.refresh_current_directory();
        
        result
    }
    
    /// Refresh current directory files
    pub fn refresh_current_directory(&mut self) {
        // Re-read directory files from disk
        if let Ok(directory_files) = self.file_service.read_directory(&self.current_dir) {
            self.directory_files = directory_files;
        }
        self.refresh_files_for_current_mode();
    }
    
    /// Get file content for display
    pub fn get_file_content(&self, file: &FileEntry) -> String {
        match self.file_service.read_file_content(file) {
            Ok(content) => content,
            Err(e) => format!("❌ Error reading file: {}", e),
        }
    }
}

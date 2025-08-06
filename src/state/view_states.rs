use std::path::PathBuf;
use crate::state::FileEntry;

/// View state for the search input component (shared by search and fuzzy find modes)
#[derive(Debug)]
pub struct SearchInputViewState {
    pub query: String,
}

impl SearchInputViewState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
        }
    }

    /// Update search query
    pub fn set_query(&mut self, query: String) {
        self.query = query;
    }

    /// Append character to search query
    pub fn append_to_query(&mut self, c: char) {
        self.query.push(c);
    }

    /// Remove last character from search query
    pub fn pop_from_query(&mut self) {
        self.query.pop();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    /// Delete word backward (Ctrl+W)
    pub fn delete_word_backward(&mut self) {
        if let Some(pos) = self.query.rfind(' ') {
            self.query.truncate(pos);
        } else {
            self.query.clear();
        }
    }

    /// Delete to end of query (Ctrl+U)
    pub fn delete_to_end(&mut self) {
        self.query.clear();
    }

    /// Check if search is active
    pub fn is_active(&self) -> bool {
        !self.query.is_empty()
    }
}

/// View state for the file list component (displays filtered files)
#[derive(Debug)]
pub struct FileListViewState {
    pub files: Vec<FileEntry>,
    pub selected_index: usize,
    pub title: String,
}

impl FileListViewState {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            selected_index: 0,
            title: "Files".to_string(),
        }
    }

    /// Update the displayed files
    pub fn set_files(&mut self, files: Vec<FileEntry>, title: String) {
        self.files = files;
        self.title = title;
        
        // Reset selection if out of bounds
        if self.selected_index >= self.files.len() && !self.files.is_empty() {
            self.selected_index = self.files.len() - 1;
        } else if self.files.is_empty() {
            self.selected_index = 0;
        }
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

    /// Get currently selected file
    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        self.files.get(self.selected_index)
    }

    /// Get file by index
    pub fn get_file(&self, index: usize) -> Option<&FileEntry> {
        self.files.get(index)
    }

    /// Get total number of files
    pub fn files_len(&self) -> usize {
        self.files.len()
    }

    /// Check if any files are available
    pub fn has_files(&self) -> bool {
        !self.files.is_empty()
    }
}

/// View state for the file detail/preview component
#[derive(Debug)]
pub struct FileDetailViewState {
    pub title: String,
    pub content: String,
}

impl FileDetailViewState {
    pub fn new() -> Self {
        Self {
            title: "No Selection".to_string(),
            content: "No file or directory selected".to_string(),
        }
    }

    /// Update the displayed file details
    pub fn set_file_details(&mut self, title: String, content: String) {
        self.title = title;
        self.content = content;
    }

    /// Clear file details (show no selection)
    pub fn clear(&mut self) {
        self.title = "No Selection".to_string();
        self.content = "No file or directory selected".to_string();
    }
}

/// Data source for directory navigation
#[derive(Debug)]
pub struct NavigationData {
    pub current_dir: PathBuf,
    pub files: Vec<FileEntry>,
}

impl NavigationData {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self {
            current_dir,
            files: Vec::new(),
        }
    }

    /// Update with new directory and files
    pub fn update(&mut self, current_dir: PathBuf, files: Vec<FileEntry>) {
        self.current_dir = current_dir;
        self.files = files;
    }

    /// Filter files based on search query
    pub fn filter_files(&self, query: &str) -> Vec<FileEntry> {
        if query.is_empty() {
            self.files.clone()
        } else {
            self.files
                .iter()
                .filter(|file| {
                    file.name.to_lowercase().contains(&query.to_lowercase())
                })
                .cloned()
                .collect()
        }
    }
}

/// Data source for fuzzy finding
#[derive(Debug)]
pub struct FuzzyFindData {
    pub all_files: Vec<FileEntry>,
    pub is_indexing: bool,
}

impl FuzzyFindData {
    pub fn new() -> Self {
        Self {
            all_files: Vec::new(),
            is_indexing: false,
        }
    }

    /// Start indexing
    pub fn start_indexing(&mut self) {
        self.is_indexing = true;
        self.all_files.clear();
    }

    /// Add files to index
    pub fn add_files(&mut self, files: Vec<FileEntry>) {
        self.all_files.extend(files);
    }

    /// Finish indexing
    pub fn finish_indexing(&mut self) {
        self.is_indexing = false;
    }

    /// Filter files with fuzzy matching
    pub fn fuzzy_filter_files(&self, query: &str) -> Vec<FileEntry> {
        if query.is_empty() {
            self.all_files.clone()
        } else {
            let mut matches: Vec<(FileEntry, i32)> = self.all_files
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
    }

    /// Simple fuzzy matching algorithm
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

    /// Get total file count
    pub fn total_count(&self) -> usize {
        self.all_files.len()
    }

    /// Clear all data
    pub fn clear(&mut self) {
        self.all_files.clear();
        self.is_indexing = false;
    }
}
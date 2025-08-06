use std::path::PathBuf;
use crate::state::FileEntry;

/// State component for fuzzy finding files across directory tree
#[derive(Debug)]
pub struct FuzzyFindState {
    pub query: String,
    pub all_files: Vec<FileEntry>,        // All files found recursively
    pub filtered_indices: Vec<usize>,     // Indices into all_files that match query
    pub selected_index: usize,            // Index into filtered_indices
    pub is_indexing: bool,                // Whether we're still scanning directories
}

impl FuzzyFindState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            all_files: Vec::new(),
            filtered_indices: Vec::new(),
            selected_index: 0,
            is_indexing: false,
        }
    }

    /// Start indexing files from the given root directory
    pub fn start_indexing(&mut self, _root_dir: &PathBuf) {
        self.is_indexing = true;
        self.all_files.clear();
        self.filtered_indices.clear();
        self.selected_index = 0;
        // Actual indexing will be handled by FileService
    }

    /// Add files to the index (called by FileService during recursive scan)
    pub fn add_files(&mut self, files: Vec<FileEntry>) {
        self.all_files.extend(files);
        self.update_filtered_results();
    }

    /// Mark indexing as complete
    pub fn finish_indexing(&mut self) {
        self.is_indexing = false;
        self.update_filtered_results();
    }

    /// Update search query and filter results
    pub fn set_query(&mut self, query: String) {
        self.query = query;
        self.update_filtered_results();
    }

    /// Append character to search query
    pub fn append_to_query(&mut self, c: char) {
        self.query.push(c);
        self.update_filtered_results();
    }

    /// Remove last character from search query
    pub fn pop_from_query(&mut self) {
        self.query.pop();
        self.update_filtered_results();
    }

    /// Clear search query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.update_filtered_results();
    }

    /// Delete word backward (Ctrl+W)
    pub fn delete_word_backward(&mut self) {
        if let Some(pos) = self.query.rfind(' ') {
            self.query.truncate(pos);
        } else {
            self.query.clear();
        }
        self.update_filtered_results();
    }

    /// Delete to end of query (Ctrl+K)
    pub fn delete_to_end(&mut self) {
        self.query.clear();
        self.update_filtered_results();
    }

    /// Update filtered results based on current query using fuzzy matching
    fn update_filtered_results(&mut self) {
        if self.query.is_empty() {
            // Show all files when no query
            self.filtered_indices = (0..self.all_files.len()).collect();
        } else {
            // Apply fuzzy matching
            let mut matches: Vec<(usize, i32)> = self.all_files
                .iter()
                .enumerate()
                .filter_map(|(i, file)| {
                    let score = self.fuzzy_match(&file.path.to_string_lossy(), &self.query);
                    if score > 0 {
                        Some((i, score))
                    } else {
                        None
                    }
                })
                .collect();

            // Sort by score (higher is better)
            matches.sort_by(|a, b| b.1.cmp(&a.1));
            self.filtered_indices = matches.into_iter().map(|(i, _)| i).collect();
        }

        // Reset selection if out of bounds
        if self.selected_index >= self.filtered_indices.len() && !self.filtered_indices.is_empty() {
            self.selected_index = self.filtered_indices.len() - 1;
        } else if self.filtered_indices.is_empty() {
            self.selected_index = 0;
        }
    }

    /// Simple fuzzy matching algorithm
    /// Returns score > 0 if match, 0 if no match. Higher scores are better matches.
    fn fuzzy_match(&self, text: &str, pattern: &str) -> i32 {
        let text = text.to_lowercase();
        let pattern = pattern.to_lowercase();
        
        if pattern.is_empty() {
            return 100; // Empty pattern matches everything with high score
        }

        let mut score: i32 = 0;
        let text_chars = text.chars().collect::<Vec<_>>();
        let pattern_chars = pattern.chars().collect::<Vec<_>>();
        
        let mut text_idx = 0;
        let mut pattern_idx = 0;
        let mut consecutive_matches = 0;

        while text_idx < text_chars.len() && pattern_idx < pattern_chars.len() {
            if text_chars[text_idx] == pattern_chars[pattern_idx] {
                // Found a match
                score += 10 + consecutive_matches; // Bonus for consecutive matches
                consecutive_matches += 1;
                pattern_idx += 1;
            } else {
                consecutive_matches = 0;
            }
            text_idx += 1;
        }

        // Must match all pattern characters
        if pattern_idx == pattern_chars.len() {
            // Bonus for shorter paths (prefer files closer to root)
            let path_depth_penalty = text.matches('/').count() as i32;
            score = score.saturating_sub(path_depth_penalty);
            
            // Bonus for matching at word boundaries or path separators
            if text.contains(&pattern) {
                score += 50; // Exact substring match bonus
            }
            
            score.max(1) // Ensure positive score for matches
        } else {
            0 // Didn't match all characters
        }
    }

    /// Move selection up in filtered results
    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down in filtered results
    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.filtered_indices.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Get the currently selected file
    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        self.get_filtered_file(self.selected_index)
    }

    /// Get file by filtered index
    pub fn get_filtered_file(&self, filtered_index: usize) -> Option<&FileEntry> {
        self.filtered_indices
            .get(filtered_index)
            .and_then(|&file_index| self.all_files.get(file_index))
    }

    /// Get number of filtered files
    pub fn filtered_count(&self) -> usize {
        self.filtered_indices.len()
    }

    /// Get total number of indexed files
    pub fn total_count(&self) -> usize {
        self.all_files.len()
    }

    /// Check if search is active
    pub fn is_active(&self) -> bool {
        !self.query.is_empty() || !self.all_files.is_empty()
    }

    /// Clear all fuzzy find state (used when changing directories)
    pub fn clear(&mut self) {
        self.query.clear();
        self.all_files.clear();
        self.filtered_indices.clear();
        self.selected_index = 0;
        self.is_indexing = false;
    }
}
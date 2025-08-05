/// State component responsible for search functionality and filtering
#[derive(Debug)]
pub struct SearchState {
    pub query: String,
    pub filtered_indices: Vec<usize>, // Indices into the files Vec
    pub selected_index: usize,        // Index into filtered_indices
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            filtered_indices: Vec::new(),
            selected_index: 0,
        }
    }


    /// Append character to search query
    pub fn append_to_query(&mut self, c: char, total_files: usize) {
        self.query.push(c);
        self.update_filtered_indices(total_files);
    }

    /// Remove last character from search query
    pub fn pop_from_query(&mut self, total_files: usize) {
        self.query.pop();
        self.update_filtered_indices(total_files);
    }

    /// Clear search query
    pub fn clear_query(&mut self, total_files: usize) {
        self.query.clear();
        self.update_filtered_indices(total_files);
    }

    /// Delete word backward (Ctrl+W)
    pub fn delete_word_backward(&mut self, total_files: usize) {
        if let Some(pos) = self.query.rfind(' ') {
            self.query.truncate(pos);
        } else {
            self.query.clear();
        }
        self.update_filtered_indices(total_files);
    }

    /// Delete to end of query (Ctrl+K)
    pub fn delete_to_end(&mut self, total_files: usize) {
        self.query.clear();
        self.update_filtered_indices(total_files);
    }

    /// Update filtered indices based on current query
    fn update_filtered_indices(&mut self, total_files: usize) {
        if self.query.is_empty() {
            // No search: all files are visible
            self.filtered_indices = (0..total_files).collect();
        } else {
            // This is a placeholder - the actual filtering logic should use file names
            // In the new architecture, this will be handled by the parent component
            self.filtered_indices = (0..total_files).collect();
        }

        // Reset selection if out of bounds
        if self.selected_index >= self.filtered_indices.len() && !self.filtered_indices.is_empty() {
            self.selected_index = self.filtered_indices.len() - 1;
        } else if self.filtered_indices.is_empty() {
            self.selected_index = 0;
        }
    }

    /// Update filtered indices with actual file filtering logic
    pub fn update_with_file_names<F>(&mut self, total_files: usize, get_file_name: F)
    where
        F: Fn(usize) -> Option<String>,
    {
        if self.query.is_empty() {
            self.filtered_indices = (0..total_files).collect();
        } else {
            self.filtered_indices = (0..total_files)
                .filter(|&i| {
                    if let Some(name) = get_file_name(i) {
                        name.to_lowercase().contains(&self.query.to_lowercase())
                    } else {
                        false
                    }
                })
                .collect();
        }

        // Reset selection if out of bounds
        if self.selected_index >= self.filtered_indices.len() && !self.filtered_indices.is_empty() {
            self.selected_index = self.filtered_indices.len() - 1;
        } else if self.filtered_indices.is_empty() {
            self.selected_index = 0;
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

    /// Get the currently selected file index (in the original files array)
    pub fn get_selected_file_index(&self) -> Option<usize> {
        self.filtered_indices.get(self.selected_index).copied()
    }

    /// Get file index by filtered index
    pub fn get_file_index(&self, filtered_index: usize) -> Option<usize> {
        self.filtered_indices.get(filtered_index).copied()
    }

    /// Get number of filtered files
    pub fn filtered_count(&self) -> usize {
        self.filtered_indices.len()
    }

    /// Check if search is active
    pub fn is_active(&self) -> bool {
        !self.query.is_empty()
    }
}
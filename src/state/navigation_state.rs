use std::path::PathBuf;
use crate::core::{ClazyfilerError, Result};
use crate::services::FileService;
use crate::state::app_state::FileEntry;

/// State component responsible for directory navigation and file selection
#[derive(Debug)]
pub struct NavigationState {
    pub current_dir: PathBuf,
    pub files: Vec<FileEntry>,
    pub selected_index: usize,
    file_service: FileService,
}

impl NavigationState {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let file_service = FileService::new();
        
        let mut state = Self {
            current_dir: current_dir.clone(),
            files: Vec::new(),
            selected_index: 0,
            file_service,
        };
        
        // Initial directory load
        if let Err(e) = state.refresh_files() {
            eprintln!("Warning: Failed to load initial directory: {}", e);
        }
        
        state
    }

    /// Refresh the current directory contents
    pub fn refresh_files(&mut self) -> Result<()> {
        self.files = self.file_service.read_directory(&self.current_dir)?;
        
        // Reset selection if out of bounds
        if self.selected_index >= self.files.len() && !self.files.is_empty() {
            self.selected_index = self.files.len() - 1;
        } else if self.files.is_empty() {
            self.selected_index = 0;
        }
        
        Ok(())
    }

    /// Navigate into the selected directory
    pub fn enter_directory(&mut self) -> Result<()> {
        let selected = self.get_selected_file()
            .ok_or_else(|| ClazyfilerError::navigation(
                self.current_dir.to_string_lossy().as_ref(), 
                "No file selected"
            ))?;

        if !selected.is_directory {
            return Err(ClazyfilerError::navigation(
                selected.path.to_string_lossy().as_ref(),
                "Selected item is not a directory"
            ));
        }

        self.current_dir = selected.path.clone();
        self.selected_index = 0;
        self.refresh_files()?;
        Ok(())
    }

    /// Navigate to parent directory
    pub fn go_to_parent(&mut self) -> Result<()> {
        let parent = self.file_service.get_parent_dir(&self.current_dir)
            .ok_or_else(|| ClazyfilerError::navigation(
                self.current_dir.to_string_lossy().as_ref(),
                "Already at root directory"
            ))?;

        self.current_dir = parent;
        self.selected_index = 0;
        self.refresh_files()?;
        Ok(())
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

    /// Get file content for display
    pub fn get_file_content(&self, file: &FileEntry) -> Result<String> {
        self.file_service.read_file_content(file)
    }

    /// Get total number of files
    pub fn files_len(&self) -> usize {
        self.files.len()
    }

    /// Get file by index
    pub fn get_file(&self, index: usize) -> Option<&FileEntry> {
        self.files.get(index)
    }
}
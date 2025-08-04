use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use std::io::Read;

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
    pub filtered_files: Vec<usize>, // Indices into files Vec
    pub selected_index: usize,      // Index into filtered_files
    pub search_query: String,
}

impl AppState {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut state = Self {
            current_dir: current_dir.clone(),
            files: Vec::new(),
            filtered_files: Vec::new(),
            selected_index: 0,
            search_query: String::new(),
        };
        
        state.refresh_files();
        state
    }

    pub fn refresh_files(&mut self) {
        self.files.clear();
        self.clear_search_query();
        
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
        
        // Update filtered files after refresh
        self.update_filtered_files();
    }

    fn update_filtered_files(&mut self) {
        if self.search_query.is_empty() {
            // No search: all files are visible
            self.filtered_files = (0..self.files.len()).collect();
        } else {
            // Search active: filter files by name
            self.filtered_files = self.files
                .iter()
                .enumerate()
                .filter(|(_, file)| {
                    file.name.to_lowercase().contains(&self.search_query.to_lowercase())
                })
                .map(|(index, _)| index)
                .collect();
        }
        
        // Reset selection if out of bounds
        if self.selected_index >= self.filtered_files.len() && !self.filtered_files.is_empty() {
            self.selected_index = self.filtered_files.len() - 1;
        } else if self.filtered_files.is_empty() {
            self.selected_index = 0;
        }
    }

    pub fn get_selected_file(&self) -> Option<&FileEntry> {
        if let Some(&file_index) = self.filtered_files.get(self.selected_index) {
            self.files.get(file_index)
        } else {
            None
        }
    }

    pub fn get_filtered_file(&self, index: usize) -> Option<&FileEntry> {
        if let Some(&file_index) = self.filtered_files.get(index) {
            self.files.get(file_index)
        } else {
            None
        }
    }

    pub fn filtered_files_len(&self) -> usize {
        self.filtered_files.len()
    }

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.filtered_files.len().saturating_sub(1) {
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
        self.update_filtered_files();
    }

    pub fn append_search_query(&mut self, c: char) {
        self.search_query.push(c);
        self.update_filtered_files();
    }

    pub fn pop_search_query(&mut self) {
        self.search_query.pop();
        self.update_filtered_files();
    }

    pub fn clear_search_query(&mut self) {
        self.search_query.clear();
        self.update_filtered_files();
    }

    pub fn delete_word_backward(&mut self) {
        // Delete word backward (Ctrl+W) - find last space and delete from there
        if let Some(pos) = self.search_query.rfind(' ') {
            self.search_query.truncate(pos);
        } else {
            self.search_query.clear();
        }
        self.update_filtered_files();
    }

    pub fn delete_to_end(&mut self) {
        // Delete to end of line (Ctrl+K) - in search, this is same as clear
        self.search_query.clear();
        self.update_filtered_files();
    }

    /// Read file content for display in the right panel
    pub fn read_file_content(&self, file: &FileEntry) -> String {
        if file.is_directory {
            self.list_directory_children(file)
        } else {
            self.read_text_file_content(file)
        }
    }

    /// Read text content from a file with size and binary detection
    fn read_text_file_content(&self, file: &FileEntry) -> String {
        const MAX_FILE_SIZE: u64 = 1024 * 1024; // 1MB limit
        const MAX_PREVIEW_LINES: usize = 100;

        // Check file size
        if let Some(size) = file.size {
            if size > MAX_FILE_SIZE {
                return format!(
                    "📄 File too large to preview\n\nSize: {}\nPath: {}\n\nUse external editor to view this file.",
                    self.format_file_size(size),
                    file.path.display()
                );
            }
        }

        match fs::File::open(&file.path) {
            Ok(mut file_handle) => {
                let mut buffer = Vec::new();
                
                // Read the file
                match file_handle.read_to_end(&mut buffer) {
                    Ok(_) => {
                        // Check if file contains binary data
                        if buffer.iter().any(|&b| b == 0 || (b < 32 && b != b'\n' && b != b'\r' && b != b'\t')) {
                            format!(
                                "🔧 Binary file detected\n\nSize: {} bytes\nPath: {}\n\nThis appears to be a binary file and cannot be displayed as text.",
                                buffer.len(),
                                file.path.display()
                            )
                        } else {
                            // Convert to string and limit lines
                            match String::from_utf8(buffer) {
                                Ok(content) => {
                                    let lines: Vec<&str> = content.lines().collect();
                                    if lines.len() > MAX_PREVIEW_LINES {
                                        format!(
                                            "📝 Text File Preview (first {} lines)\n\n{}\n\n... ({} more lines)",
                                            MAX_PREVIEW_LINES,
                                            lines[..MAX_PREVIEW_LINES].join("\n"),
                                            lines.len() - MAX_PREVIEW_LINES
                                        )
                                    } else {
                                        format!("📝 Text File Content\n\n{}", content)
                                    }
                                },
                                Err(_) => format!(
                                    "⚠️ Invalid UTF-8 encoding\n\nPath: {}\n\nFile contains non-UTF-8 data and cannot be displayed.",
                                    file.path.display()
                                )
                            }
                        }
                    },
                    Err(e) => format!(
                        "❌ Failed to read file\n\nPath: {}\nError: {}\n\nCheck file permissions and try again.",
                        file.path.display(),
                        e
                    )
                }
            },
            Err(e) => format!(
                "❌ Failed to open file\n\nPath: {}\nError: {}\n\nCheck if file exists and you have read permissions.",
                file.path.display(),
                e
            )
        }
    }

    /// List directory children for display in the right panel
    fn list_directory_children(&self, dir: &FileEntry) -> String {
        match fs::read_dir(&dir.path) {
            Ok(entries) => {
                let mut children: Vec<FileEntry> = Vec::new();
                
                for entry in entries.flatten() {
                    if let Ok(metadata) = entry.metadata() {
                        children.push(FileEntry {
                            name: entry.file_name().to_string_lossy().to_string(),
                            path: entry.path(),
                            is_directory: metadata.is_dir(),
                            size: if metadata.is_file() { Some(metadata.len()) } else { None },
                            modified: metadata.modified().ok(),
                        });
                    }
                }

                // Sort: directories first, then files, both alphabetically
                children.sort_by(|a, b| {
                    match (a.is_directory, b.is_directory) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    }
                });

                if children.is_empty() {
                    format!("📁 Directory Contents\n\n(Empty directory)")
                } else {
                    let mut content = format!("📁 Directory Contents ({} items)\n\n", children.len());
                    
                    for child in children.iter().take(50) { // Limit to 50 items for display
                        let icon = if child.is_directory { "📁" } else { "📄" };
                        let size_str = if let Some(size) = child.size {
                            format!(" ({})", self.format_file_size(size))
                        } else {
                            String::new()
                        };
                        content.push_str(&format!("{} {}{}\n", icon, child.name, size_str));
                    }
                    
                    if children.len() > 50 {
                        content.push_str(&format!("\n... and {} more items", children.len() - 50));
                    }
                    
                    content
                }
            },
            Err(e) => format!(
                "❌ Failed to read directory\n\nPath: {}\nError: {}\n\nCheck directory permissions.",
                dir.path.display(),
                e
            )
        }
    }

    /// Format file size helper (moved from UI)
    fn format_file_size(&self, size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size_f = size as f64;
        let mut unit_index = 0;

        while size_f >= 1024.0 && unit_index < UNITS.len() - 1 {
            size_f /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size_f, UNITS[unit_index])
        }
    }

}

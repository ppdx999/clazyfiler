use std::fs;
use std::path::{Path, PathBuf};
use std::io::Read;
use crate::core::{ClazyfilerError, Result};
use crate::state::FileEntry;

/// Service responsible for all file system operations
/// Extracted from AppState to separate concerns and improve testability
#[derive(Debug)]
pub struct FileService;

impl FileService {
    pub fn new() -> Self {
        Self
    }

    /// Read directory contents and return sorted file entries
    pub fn read_directory(&self, dir_path: &Path) -> Result<Vec<FileEntry>> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| ClazyfilerError::file_system("read_dir", dir_path.to_string_lossy().as_ref(), e))?;

        let mut files = Vec::new();
        
        for entry in entries {
            match entry {
                Ok(entry) => {
                    match entry.metadata() {
                        Ok(metadata) => {
                            let file_entry = FileEntry {
                                name: entry.file_name().to_string_lossy().to_string(),
                                path: entry.path(),
                                is_directory: metadata.is_dir(),
                                size: if metadata.is_file() { Some(metadata.len()) } else { None },
                            };
                            files.push(file_entry);
                        }
                        Err(e) => {
                            // Log warning but continue processing other files
                            eprintln!("Warning: Failed to read metadata for {}: {}", entry.path().display(), e);
                        }
                    }
                }
                Err(e) => {
                    // Log warning but continue processing other files
                    eprintln!("Warning: Failed to read directory entry: {}", e);
                }
            }
        }

        // Sort: directories first, then files, both alphabetically
        files.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });

        Ok(files)
    }

    /// Read file content for display, with size and binary detection
    pub fn read_file_content(&self, file: &FileEntry) -> Result<String> {
        if file.is_directory {
            return self.list_directory_children(file);
        }

        const MAX_FILE_SIZE: u64 = 1024 * 1024; // 1MB limit
        const MAX_PREVIEW_LINES: usize = 100;

        // Check file size
        if let Some(size) = file.size {
            if size > MAX_FILE_SIZE {
                return Ok(format!(
                    "📄 File too large to preview\n\nSize: {}\nPath: {}\n\nUse external editor to view this file.",
                    Self::format_file_size(size),
                    file.path.display()
                ));
            }
        }

        let mut file_handle = fs::File::open(&file.path)
            .map_err(|e| ClazyfilerError::file_system("open", file.path.to_string_lossy().as_ref(), e))?;

        let mut buffer = Vec::new();
        file_handle.read_to_end(&mut buffer)
            .map_err(|e| ClazyfilerError::file_system("read", file.path.to_string_lossy().as_ref(), e))?;

        // Check if file contains binary data
        if buffer.iter().any(|&b| b == 0 || (b < 32 && b != b'\n' && b != b'\r' && b != b'\t')) {
            return Ok(format!(
                "🔧 Binary file detected\n\nSize: {} bytes\nPath: {}\n\nThis appears to be a binary file and cannot be displayed as text.",
                buffer.len(),
                file.path.display()
            ));
        }

        // Convert to string and limit lines
        match String::from_utf8(buffer) {
            Ok(content) => {
                let lines: Vec<&str> = content.lines().collect();
                if lines.len() > MAX_PREVIEW_LINES {
                    Ok(format!(
                        "📝 Text File Preview (first {} lines)\n\n{}\n\n... ({} more lines)",
                        MAX_PREVIEW_LINES,
                        lines[..MAX_PREVIEW_LINES].join("\n"),
                        lines.len() - MAX_PREVIEW_LINES
                    ))
                } else {
                    Ok(format!("📝 Text File Content\n\n{}", content))
                }
            },
            Err(_) => Ok(format!(
                "⚠️ Invalid UTF-8 encoding\n\nPath: {}\n\nFile contains non-UTF-8 data and cannot be displayed.",
                file.path.display()
            ))
        }
    }

    /// List directory children for display
    fn list_directory_children(&self, dir: &FileEntry) -> Result<String> {
        let children = self.read_directory(&dir.path)?;

        if children.is_empty() {
            Ok("📁 Directory Contents\n\n(Empty directory)".to_string())
        } else {
            let mut content = format!("📁 Directory Contents ({} items)\n\n", children.len());
            
            for child in children.iter().take(50) { // Limit to 50 items for display
                let icon = if child.is_directory { "📁" } else { "📄" };
                let size_str = if let Some(size) = child.size {
                    format!(" ({})", Self::format_file_size(size))
                } else {
                    String::new()
                };
                content.push_str(&format!("{} {}{}\n", icon, child.name, size_str));
            }
            
            if children.len() > 50 {
                content.push_str(&format!("\n... and {} more items", children.len() - 50));
            }
            
            Ok(content)
        }
    }

    /// Format file size helper
    fn format_file_size(size: u64) -> String {
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


    /// Get parent directory of a given path
    pub fn get_parent_dir(&self, path: &Path) -> Option<PathBuf> {
        path.parent().map(|p| p.to_path_buf())
    }

    /// Recursively scan directory tree and return all files
    /// This is used for fuzzy finding across the entire directory structure
    pub fn scan_directory_tree(&self, root_path: &Path) -> Result<Vec<FileEntry>> {
        let mut all_files = Vec::new();
        self.scan_directory_recursive(root_path, &mut all_files)?;
        Ok(all_files)
    }

    /// Recursive helper for directory tree scanning
    fn scan_directory_recursive(&self, dir_path: &Path, all_files: &mut Vec<FileEntry>) -> Result<()> {
        let entries = fs::read_dir(dir_path)
            .map_err(|e| ClazyfilerError::file_system("read_dir", dir_path.to_string_lossy().as_ref(), e))?;

        for entry in entries {
            match entry {
                Ok(entry) => {
                    match entry.metadata() {
                        Ok(metadata) => {
                            let file_entry = FileEntry {
                                name: entry.file_name().to_string_lossy().to_string(),
                                path: entry.path(),
                                is_directory: metadata.is_dir(),
                                size: if metadata.is_file() { Some(metadata.len()) } else { None },
                            };

                            // Add this entry to our results
                            all_files.push(file_entry.clone());

                            // If it's a directory, recursively scan it
                            if metadata.is_dir() {
                                // Skip hidden directories and common build/cache directories to avoid slowdown
                                let file_name = entry.file_name();
                                let dir_name = file_name.to_string_lossy();
                                if !dir_name.starts_with('.') && 
                                   !matches!(dir_name.as_ref(), "node_modules" | "target" | ".git" | "build" | "dist") {
                                    if let Err(e) = self.scan_directory_recursive(&entry.path(), all_files) {
                                        // Log error but continue scanning other directories
                                        eprintln!("Warning: Failed to scan directory {}: {}", entry.path().display(), e);
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            // Log warning but continue processing other files
                            eprintln!("Warning: Failed to read metadata for {}: {}", entry.path().display(), e);
                        }
                    }
                }
                Err(e) => {
                    // Log warning but continue processing other files
                    eprintln!("Warning: Failed to read directory entry: {}", e);
                }
            }
        }
        Ok(())
    }
}
use crate::store::{Store, FileEntry};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use std::{fs, path::PathBuf, time::SystemTime};

// View function - renders the UI based on store state
pub fn view(f: &mut Frame, store: &Store) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(store.config.ui.panel_width_ratio as u16),
            Constraint::Percentage(100 - store.config.ui.panel_width_ratio as u16),
        ])
        .split(f.area());

    // Left panel - File list
    let items: Vec<ListItem> = store.state
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let style = if i == store.state.selected_index {
                Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            let prefix = if file.is_dir { "📁 " } else { "📄 " };
            ListItem::new(format!("{}{}", prefix, file.name)).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    if !items.is_empty() && store.state.selected_index < items.len() {
        list_state.select(Some(store.state.selected_index));
    }

    let files_list = List::new(items)
        .block(
            Block::default()
                .title(format!("Files in {}", store.state.current_dir.display()))
                .borders(Borders::ALL)
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(files_list, main_chunks[0], &mut list_state);

    // Right panel - File details
    let details_text = if let Some(selected_file) = store.get_selected_file() {
        format_file_details(selected_file)
    } else {
        "No file selected".to_string()
    };

    let details_paragraph = Paragraph::new(details_text)
        .block(
            Block::default()
                .title("Details")
                .borders(Borders::ALL)
        )
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));

    f.render_widget(details_paragraph, main_chunks[1]);
}

pub fn format_file_details(file: &FileEntry) -> String {
    let mut details = Vec::new();
    
    details.push(format!("Name: {}", file.name));
    details.push(format!("Path: {}", file.path.display()));
    details.push(format!("Type: {}", if file.is_dir { "Directory" } else { "File" }));
    
    if !file.is_dir {
        details.push(format!("Size: {} bytes", file.size));
        if file.size > 1024 {
            let kb = file.size as f64 / 1024.0;
            if kb > 1024.0 {
                let mb = kb / 1024.0;
                details.push(format!("Size (MB): {:.2}", mb));
            } else {
                details.push(format!("Size (KB): {:.2}", kb));
            }
        }
    }
    
    if let Some(modified) = file.modified {
        if let Ok(duration) = modified.duration_since(SystemTime::UNIX_EPOCH) {
            let secs = duration.as_secs();
            let days = secs / 86400;
            let hours = (secs % 86400) / 3600;
            let mins = (secs % 3600) / 60;
            
            if days > 0 {
                details.push(format!("Modified: {} days ago", days));
            } else if hours > 0 {
                details.push(format!("Modified: {} hours ago", hours));
            } else if mins > 0 {
                details.push(format!("Modified: {} minutes ago", mins));
            } else {
                details.push("Modified: Less than a minute ago".to_string());
            }
        }
    }
    
    details.push("".to_string());
    details.push("─".repeat(40));
    details.push("".to_string());
    
    let content = get_file_content(file);
    details.push(content);
    
    details.join("\n")
}

fn get_file_content(file: &FileEntry) -> String {
    if file.is_dir {
        get_directory_listing(&file.path)
    } else {
        get_file_preview(&file.path)
    }
}

fn get_directory_listing(dir_path: &PathBuf) -> String {
    match fs::read_dir(dir_path) {
        Ok(entries) => {
            let mut items = Vec::new();
            items.push("Contents:".to_string());
            items.push("".to_string());

            let mut dirs = Vec::new();
            let mut files = Vec::new();

            for entry in entries {
                if let Ok(entry) = entry {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if let Ok(metadata) = entry.metadata() {
                        if metadata.is_dir() {
                            dirs.push(format!("📁 {}/", name));
                        } else {
                            let size = if metadata.len() > 1024 * 1024 {
                                format!(" ({:.1} MB)", metadata.len() as f64 / (1024.0 * 1024.0))
                            } else if metadata.len() > 1024 {
                                format!(" ({:.1} KB)", metadata.len() as f64 / 1024.0)
                            } else {
                                format!(" ({} B)", metadata.len())
                            };
                            files.push(format!("📄 {}{}", name, size));
                        }
                    }
                }
            }

            dirs.sort();
            files.sort();

            let has_dirs = !dirs.is_empty();
            let has_files = !files.is_empty();
            
            items.extend(dirs);
            if has_dirs && has_files {
                items.push("".to_string());
            }
            items.extend(files);

            if items.len() <= 2 {
                items.push("(Empty directory)".to_string());
            }

            items.join("\n")
        }
        Err(e) => format!("Error reading directory: {}", e),
    }
}

fn get_file_preview(file_path: &PathBuf) -> String {
    match fs::metadata(file_path) {
        Ok(metadata) => {
            let size = metadata.len();
            
            if size > 10 * 1024 * 1024 {
                return format!("File too large to preview\nSize: {:.2} MB", size as f64 / (1024.0 * 1024.0));
            }

            match fs::read(file_path) {
                Ok(bytes) => {
                    if is_binary_content(&bytes) {
                        format!("Binary file\nSize: {} bytes\nType: {}", 
                            size, 
                            guess_file_type(file_path)
                        )
                    } else {
                        match String::from_utf8(bytes) {
                            Ok(content) => {
                                let lines: Vec<&str> = content.lines().collect();
                                if lines.len() > 100 {
                                    format!("Text file preview (first 100 lines):\n\n{}\n\n... ({} more lines)", 
                                        lines[..100].join("\n"), 
                                        lines.len() - 100
                                    )
                                } else {
                                    format!("Text file preview:\n\n{}", content)
                                }
                            }
                            Err(_) => {
                                format!("File contains non-UTF8 content\nSize: {} bytes", size)
                            }
                        }
                    }
                }
                Err(e) => format!("Error reading file: {}", e),
            }
        }
        Err(e) => format!("Error getting file info: {}", e),
    }
}

fn is_binary_content(bytes: &[u8]) -> bool {
    let null_count = bytes.iter().filter(|&&b| b == 0).count();
    if null_count > 0 {
        return true;
    }

    let sample_size = std::cmp::min(1024, bytes.len());
    let non_printable = bytes[..sample_size]
        .iter()
        .filter(|&&b| b < 32 && b != 9 && b != 10 && b != 13)
        .count();

    non_printable > sample_size / 4
}

fn guess_file_type(file_path: &PathBuf) -> &'static str {
    if let Some(extension) = file_path.extension() {
        match extension.to_string_lossy().to_lowercase().as_str() {
            "rs" => "Rust source",
            "py" => "Python script",
            "js" => "JavaScript",
            "ts" => "TypeScript", 
            "html" => "HTML document",
            "css" => "CSS stylesheet",
            "json" => "JSON data",
            "toml" => "TOML config",
            "yaml" | "yml" => "YAML config",
            "md" => "Markdown document",
            "txt" => "Text file",
            "log" => "Log file",
            "png" | "jpg" | "jpeg" | "gif" | "bmp" => "Image file",
            "mp3" | "wav" | "flac" => "Audio file",
            "mp4" | "avi" | "mkv" => "Video file",
            "pdf" => "PDF document",
            "zip" | "tar" | "gz" | "7z" => "Archive",
            "exe" | "dll" | "so" => "Executable",
            _ => "Unknown type",
        }
    } else {
        "No extension"
    }
}

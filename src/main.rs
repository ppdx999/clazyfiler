mod config;

use config::Config;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{
    env,
    fs,
    io,
    path::PathBuf,
    time::SystemTime,
};

#[derive(Debug)]
struct FileEntry {
    name: String,
    path: PathBuf,
    size: u64,
    is_dir: bool,
    modified: Option<SystemTime>,
}

struct App {
    current_dir: PathBuf,
    files: Vec<FileEntry>,
    selected_index: usize,
    config: Config,
}

impl App {
    fn get_file_content(&self, file: &FileEntry) -> String {
        if file.is_dir {
            self.get_directory_listing(&file.path)
        } else {
            self.get_file_preview(&file.path)
        }
    }

    fn get_directory_listing(&self, dir_path: &PathBuf) -> String {
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

    fn get_file_preview(&self, file_path: &PathBuf) -> String {
        // Check file size first
        match fs::metadata(file_path) {
            Ok(metadata) => {
                let size = metadata.len();
                
                // Don't preview very large files
                if size > 10 * 1024 * 1024 { // 10MB limit
                    return format!("File too large to preview\nSize: {:.2} MB", size as f64 / (1024.0 * 1024.0));
                }

                // Try to read the file
                match fs::read(file_path) {
                    Ok(bytes) => {
                        // Check if file appears to be binary
                        if self.is_binary_content(&bytes) {
                            format!("Binary file\nSize: {} bytes\nType: {}", 
                                size, 
                                self.guess_file_type(file_path)
                            )
                        } else {
                            // Try to convert to UTF-8 string
                            match String::from_utf8(bytes) {
                                Ok(content) => {
                                    let lines: Vec<&str> = content.lines().collect();
                                    if lines.len() > 100 {
                                        // Show first 100 lines for very long files
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

    fn is_binary_content(&self, bytes: &[u8]) -> bool {
        // Simple heuristic: if file contains null bytes or too many non-printable chars, consider it binary
        let null_count = bytes.iter().filter(|&&b| b == 0).count();
        if null_count > 0 {
            return true;
        }

        // Check first 1024 bytes for non-printable characters
        let sample_size = std::cmp::min(1024, bytes.len());
        let non_printable = bytes[..sample_size]
            .iter()
            .filter(|&&b| b < 32 && b != 9 && b != 10 && b != 13) // Allow tab, newline, carriage return
            .count();

        non_printable > sample_size / 4 // If more than 25% non-printable, consider binary
    }

    fn guess_file_type(&self, file_path: &PathBuf) -> &'static str {
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
    fn new() -> Result<App, Box<dyn std::error::Error>> {
        let current_dir = env::current_dir()?;
        let config = Config::load()?;
        let mut app = App {
            current_dir: current_dir.clone(),
            files: Vec::new(),
            selected_index: 0,
            config,
        };
        app.load_files()?;
        Ok(app)
    }

    fn load_files(&mut self) -> io::Result<()> {
        self.files.clear();
        self.selected_index = 0;
        let entries = fs::read_dir(&self.current_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            
            let file_entry = FileEntry {
                name: file_name,
                path: path.clone(),
                size: metadata.len(),
                is_dir: metadata.is_dir(),
                modified: metadata.modified().ok(),
            };
            
            self.files.push(file_entry);
        }
        
        self.files.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(())
    }

    fn move_selection(&mut self, direction: isize) {
        if self.files.is_empty() {
            return;
        }

        let new_index = (self.selected_index as isize + direction)
            .max(0)
            .min(self.files.len() as isize - 1) as usize;
        
        self.selected_index = new_index;
    }

    fn get_selected_file(&self) -> Option<&FileEntry> {
        self.files.get(self.selected_index)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let result = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    if app.config.key_matches(c, "quit") {
                        return Ok(());
                    } else if app.config.key_matches(c, "up") {
                        app.move_selection(-1);
                    } else if app.config.key_matches(c, "down") {
                        app.move_selection(1);
                    }
                }
                KeyCode::Up => app.move_selection(-1),
                KeyCode::Down => app.move_selection(1),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(app.config.ui.panel_width_ratio as u16),
            Constraint::Percentage(100 - app.config.ui.panel_width_ratio as u16),
        ])
        .split(f.area());

    // Left panel - File list
    let items: Vec<ListItem> = app
        .files
        .iter()
        .enumerate()
        .map(|(i, file)| {
            let style = if i == app.selected_index {
                Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            
            let prefix = if file.is_dir { "📁 " } else { "📄 " };
            ListItem::new(format!("{}{}", prefix, file.name)).style(style)
        })
        .collect();

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_index));

    let files_list = List::new(items)
        .block(
            Block::default()
                .title(format!("Files in {}", app.current_dir.display()))
                .borders(Borders::ALL)
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(files_list, main_chunks[0], &mut list_state);

    // Right panel - File details
    let details_text = if let Some(selected_file) = app.get_selected_file() {
        format_file_details(app, selected_file)
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

fn format_file_details(app: &App, file: &FileEntry) -> String {
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
    
    // Add content preview
    let content = app.get_file_content(file);
    details.push(content);
    
    details.join("\n")
}

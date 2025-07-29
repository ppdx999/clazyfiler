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

// FLUX ARCHITECTURE IMPLEMENTATION

// Actions (Flux Pattern)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Action {
    MoveSelection(isize),
    EnterDirectory,
    Back,
    LoadDirectory(PathBuf),
    Refresh,
    Quit,
    EnterSearchMode,
    ExitSearchMode,
    UpdateSearchQuery(String),
    SearchSelectFirst,
}

// Store (holds application state)
#[derive(Debug)]
pub struct Store {
    state: AppState,
    config: Config,
}

// Dispatcher (handles actions and updates store) - Flux Pattern
pub struct Dispatcher {
    store: Store,
}

#[derive(Debug)]
pub struct AppState {
    current_dir: PathBuf,
    files: Vec<FileEntry>,
    selected_index: usize,
    search_mode: bool,
    search_query: String,
    filtered_files: Vec<usize>,
}

#[derive(Debug)]
struct FileEntry {
    name: String,
    path: PathBuf,
    size: u64,
    is_dir: bool,
    modified: Option<SystemTime>,
}

impl Dispatcher {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let current_dir = env::current_dir()?;
        let config = Config::load()?;
        let mut store = Store {
            state: AppState {
                current_dir: current_dir.clone(),
                files: Vec::new(),
                selected_index: 0,
                search_mode: false,
                search_query: String::new(),
                filtered_files: Vec::new(),
            },
            config,
        };
        store.load_files()?;
        Ok(Dispatcher { store })
    }

    // Dispatcher - single point for all state changes (Flux Pattern)
    fn dispatch(&mut self, action: Action) -> Result<(), String> {
        match action {
            Action::MoveSelection(direction) => {
                self.store.move_selection(direction);
                Ok(())
            }
            Action::EnterDirectory => {
                self.store.enter_directory()
            }
            Action::Back => {
                self.store.go_back()
            }
            Action::LoadDirectory(path) => {
                match env::set_current_dir(&path) {
                    Ok(()) => {
                        self.store.state.current_dir = path;
                        match self.store.load_files() {
                            Ok(()) => Ok(()),
                            Err(e) => Err(format!("Failed to load directory: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Failed to change directory: {}", e)),
                }
            }
            Action::Refresh => {
                match self.store.load_files() {
                    Ok(()) => Ok(()),
                    Err(e) => Err(format!("Failed to refresh: {}", e)),
                }
            }
            Action::EnterSearchMode => {
                self.store.enter_search_mode();
                Ok(())
            }
            Action::ExitSearchMode => {
                self.store.exit_search_mode();
                Ok(())
            }
            Action::UpdateSearchQuery(query) => {
                self.store.handle_search_input(query);
                Ok(())
            }
            Action::SearchSelectFirst => {
                self.store.select_first_match();
                Ok(())
            }
            Action::Quit => Ok(()),
        }
    }

    // Delegate methods to access store state
    fn get_store(&self) -> &Store {
        &self.store
    }
}

impl Store {

    // Private methods for state manipulation
    fn move_selection(&mut self, direction: isize) {
        let max_items = if self.state.search_mode {
            if self.state.filtered_files.is_empty() {
                self.state.selected_index = 0;
                return;
            }
            self.state.filtered_files.len()
        } else {
            if self.state.files.is_empty() {
                self.state.selected_index = 0;
                return;
            }
            self.state.files.len()
        };

        let new_index = (self.state.selected_index as isize + direction)
            .max(0)
            .min(max_items.saturating_sub(1) as isize) as usize;
        
        self.state.selected_index = new_index;
    }

    fn enter_directory(&mut self) -> Result<(), String> {
        if let Some(selected_file) = self.get_selected_file() {
            if selected_file.is_dir {
                match env::set_current_dir(&selected_file.path) {
                    Ok(()) => {
                        self.state.current_dir = selected_file.path.clone();
                        match self.load_files() {
                            Ok(()) => Ok(()),
                            Err(e) => Err(format!("Failed to load directory contents: {}", e)),
                        }
                    }
                    Err(e) => Err(format!("Failed to enter directory: {}", e)),
                }
            } else {
                Err("Selected item is not a directory".to_string())
            }
        } else {
            Err("No file selected".to_string())
        }
    }

    fn go_back(&mut self) -> Result<(), String> {
        let current_path = self.state.current_dir.clone();
        if let Some(parent) = current_path.parent() {
            match env::set_current_dir(parent) {
                Ok(()) => {
                    self.state.current_dir = parent.to_path_buf();
                    match self.load_files() {
                        Ok(()) => Ok(()),
                        Err(e) => Err(format!("Failed to load parent directory: {}", e)),
                    }
                }
                Err(e) => Err(format!("Failed to go back: {}", e)),
            }
        } else {
            Err("Already at the root directory".to_string())
        }
    }

    fn load_files(&mut self) -> io::Result<()> {
        self.state.files.clear();
        self.state.selected_index = 0;
        // Reset search state when loading new directory  
        if self.state.search_mode {
            self.state.search_mode = false;
            self.state.search_query.clear();
            self.state.filtered_files.clear();
        }
        let entries = fs::read_dir(&self.state.current_dir)?;
        
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
            
            self.state.files.push(file_entry);
        }
        
        self.state.files.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(())
    }

    fn get_selected_file(&self) -> Option<&FileEntry> {
        if self.state.search_mode && !self.state.filtered_files.is_empty() {
            let filtered_index = self.state.selected_index;
            if filtered_index < self.state.filtered_files.len() {
                let actual_index = self.state.filtered_files[filtered_index];
                self.state.files.get(actual_index)
            } else {
                None
            }
        } else if self.state.search_mode {
            None
        } else {
            self.state.files.get(self.state.selected_index)
        }
    }

    fn enter_search_mode(&mut self) {
        self.state.search_mode = true;
        self.state.search_query.clear();
        self.state.selected_index = 0;
        self.update_search_filter();
    }

    fn exit_search_mode(&mut self) {
        self.state.search_mode = false;
        self.state.search_query.clear();
        self.state.filtered_files.clear();
        // Ensure selected_index is valid for the full file list
        if !self.state.files.is_empty() {
            self.state.selected_index = self.state.selected_index.min(self.state.files.len() - 1);
        } else {
            self.state.selected_index = 0;
        }
    }

    fn handle_search_input(&mut self, input: String) {
        if input.is_empty() {
            // Backspace - remove last character
            if !self.state.search_query.is_empty() {
                self.state.search_query.pop();
            }
        } else {
            // Add character
            self.state.search_query.push_str(&input);
        }
        self.update_search_filter();
        // Always reset to first item when search changes
        self.state.selected_index = 0;
    }

    fn update_search_filter(&mut self) {
        self.state.filtered_files.clear();
        if self.state.search_query.is_empty() {
            return;
        }

        let query_lower = self.state.search_query.to_lowercase();
        for (index, file) in self.state.files.iter().enumerate() {
            if file.name.to_lowercase().contains(&query_lower) {
                self.state.filtered_files.push(index);
            }
        }
    }

    fn select_first_match(&mut self) {
        if self.state.search_mode && !self.state.filtered_files.is_empty() {
            let filtered_index = self.state.selected_index;
            if filtered_index < self.state.filtered_files.len() {
                let actual_file_index = self.state.filtered_files[filtered_index];
                if actual_file_index < self.state.files.len() {
                    // Exit search mode and set selection to the matched file
                    self.state.search_mode = false;
                    self.state.search_query.clear();
                    self.state.filtered_files.clear();
                    self.state.selected_index = actual_file_index;
                }
            }
        }
    }

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
        match fs::metadata(file_path) {
            Ok(metadata) => {
                let size = metadata.len();
                
                if size > 10 * 1024 * 1024 {
                    return format!("File too large to preview\nSize: {:.2} MB", size as f64 / (1024.0 * 1024.0));
                }

                match fs::read(file_path) {
                    Ok(bytes) => {
                        if self.is_binary_content(&bytes) {
                            format!("Binary file\nSize: {} bytes\nType: {}", 
                                size, 
                                self.guess_file_type(file_path)
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

    fn is_binary_content(&self, bytes: &[u8]) -> bool {
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
}

// Action Creator - converts keyboard input to actions
fn key_to_action(key: KeyCode, config: &Config) -> Option<Action> {
    let key_str = match key {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Escape".to_string(),
        KeyCode::F(5) => "F5".to_string(),
        _ => return None,
    };

    config.keymaps.get(&key_str).and_then(|action_str| {
        match action_str.as_str() {
            "quit" => Some(Action::Quit),
            "up" => Some(Action::MoveSelection(-1)),
            "down" => Some(Action::MoveSelection(1)),
            "select" => Some(Action::EnterDirectory),
            "back" => Some(Action::Back),
            "refresh" => Some(Action::Refresh),
            _ => None,
        }
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut dispatcher = Dispatcher::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    let result = run_app(&mut terminal, &mut dispatcher);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, dispatcher: &mut Dispatcher) -> io::Result<()> {
    loop {
        terminal.draw(|f| view(f, dispatcher.get_store()))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc {
                return Ok(())
            }
            if let Some(action) = key_to_action(key.code, &dispatcher.get_store().config) {
                match &action {
                    Action::Quit => return Ok(()),
                    _ => {
                        if let Err(e) = dispatcher.dispatch(action) {
                            eprintln!("Action error: {}", e);
                        }
                    }
                }
            }
        }
    }
}

// View function - renders the UI based on store state
fn view(f: &mut Frame, store: &Store) {
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
        format_file_details(store, selected_file)
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

fn format_file_details(store: &Store, file: &FileEntry) -> String {
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
    
    let content = store.get_file_content(file);
    details.push(content);
    
    details.join("\n")
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame, Terminal,
};
use std::{
    env,
    fs,
    io,
    path::PathBuf,
};

struct App {
    current_dir: PathBuf,
    files: Vec<String>,
}

impl App {
    fn new() -> io::Result<App> {
        let current_dir = env::current_dir()?;
        let mut app = App {
            current_dir: current_dir.clone(),
            files: Vec::new(),
        };
        app.load_files()?;
        Ok(app)
    }

    fn load_files(&mut self) -> io::Result<()> {
        self.files.clear();
        let entries = fs::read_dir(&self.current_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();
            self.files.push(file_name);
        }
        
        self.files.sort();
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new()?;
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.area());

    let items: Vec<ListItem> = app
        .files
        .iter()
        .map(|file| ListItem::new(file.as_str()))
        .collect();

    let files_list = List::new(items)
        .block(
            Block::default()
                .title(format!("Files in {}", app.current_dir.display()))
                .borders(Borders::ALL)
        )
        .style(Style::default().fg(Color::White));

    f.render_widget(files_list, chunks[0]);
}

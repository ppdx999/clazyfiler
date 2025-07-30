mod actions;
mod config;
mod dispatcher;
mod modes;
mod store;
mod ui;
mod ui_helpers;

use actions::Action;
use dispatcher::{Dispatcher, key_to_action};
use ui::view;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io;

// FLUX ARCHITECTURE IMPLEMENTATION


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
        terminal.draw(|f| view(f, dispatcher.get_store(), dispatcher.get_mode_manager()))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc {
                return Ok(())
            }
            if let Some(action) = key_to_action(key.code, &dispatcher.get_store().config, dispatcher.get_mode_manager()) {
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


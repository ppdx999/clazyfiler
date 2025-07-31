use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::Backend, Frame, Terminal};
use crate::{
    actions::Action,
    modes::Mode,
    modes::interface::ModeBehavior,
};
    
pub struct App {
    pub mode: Mode
}

impl App {
    pub fn new() -> Self {
        Self {
            mode: Mode::new_explore_mode()
        }
    }

    pub fn handle_key(&self, key: KeyCode) -> Option<Action> {
        // Handle global key event
        if key == KeyCode::Char('q') {
            return Some(Action::Quit)
        }

        // Handle mode specific key event
        return self.mode.handle_key(key)
    }

    pub fn dispatch(&mut self, action: Action) -> Result<(), String> {
        // Handle global action dispatch

        // Hnadle mode specific action dispatch
        return self.mode.dispatch(action)
    }

    pub fn render(&self, frame: &mut Frame) {
        return self.mode.render(frame);
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| app.render(f))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        let Some(action) = app.handle_key(key.code) else {
            continue;
        };

        if let Action::Quit = &action {
            return Ok(());
        }

        if let Err(e) = app.dispatch(action) {
            eprintln!("Action error: {}", e);
        }
    }
}


use crossterm::event::{self, Event, KeyEvent};
use ratatui::{prelude::Backend, Frame, Terminal};
use crate::{
    actions::{Action}, key::is_ctrl_c, modes::{interface::ModeBehavior, Mode}, state::AppState
};
    
pub struct App {
    pub mode: Mode,
    pub state: AppState
}

impl App {
    pub fn new() -> Self {
        let mut app = Self {
            mode: Mode::new_explore_mode(),
            state: AppState::new()
        };
        
        // Call on_enter for the initial mode
        if let Err(e) = app.mode.on_enter(&mut app.state) {
            eprintln!("Error initializing mode: {}", e);
        }
        
        app
    }

    pub fn handle_key(&self, key: KeyEvent) -> Vec<Action> {
        // Handle global key event
        if is_ctrl_c(&key) {
            return vec![Action::Quit]
        }

        // Handle mode specific key event
        return self.mode.handle_key(key, &self.state)
    }

    pub fn dispatch(&mut self, action: Action) -> Result<(), String> {
        // Handle global action dispatch
        match action {
            // Handle Mode Switch
            Action::SwitchMode(switch_action) =>
                self.mode.switch_to(switch_action, &mut self.state),
            // Hnadle mode specific action dispatch
            _ => self.mode.dispatch(action, &mut self.state)
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Handle global rendering

        // Handle mode specific rendering
        return self.mode.render(frame, &self.state);
    }
}

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();

    loop {
        terminal.draw(|f| app.render(f))?;

        let Event::Key(key) = event::read()? else {
            continue;
        };

        let actions = app.handle_key(key);
        
        if actions.is_empty() {
            continue;
        }

        for action in actions {
            if let Action::Quit = &action {
                return Ok(());
            }

            if let Err(e) = app.dispatch(action) {
                eprintln!("Action error: {}", e);
            }
        }
    }
}


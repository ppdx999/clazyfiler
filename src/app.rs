use std::io;

use crossterm::event::{self, Event, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
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
            // Handle file opening (requires special terminal handling)
            Action::OpenFile => {
                let Some(selected) = self.state.get_selected_file() else {
                    return Err("No file selected".to_string())
                };
                if selected.is_directory {
                    return Err("Cannot open directory with vim".to_string())
                };

                let mut stdout = io::stdout();
                // Disable raw mode before launching vim
                execute!(stdout, LeaveAlternateScreen).map_err(|e| format!("Fail to LeaveAlternateScreen: {}", e))?;
                disable_raw_mode().map_err(|e| format!("Failed to disable raw mode: {}", e))?;

                let result = self.state.open_file_with_vim(selected);

                // Re-enable raw mode after vim exits
                execute!(stdout, EnterAlternateScreen).map_err(|e| format!("Fail to EnterAlternateScreen: {}", e))?;
                enable_raw_mode().map_err(|e| format!("Failed to re-enable raw mode: {}", e))?;

                match result {
                    Ok(_) => {
                        // Refresh files after returning from vim in case file was modified
                        self.state.refresh_files();
                        Ok(())
                    },
                    Err(e) => Err(e)
                }
            },
            // Handle mode specific action dispatch
            _ => self.mode.dispatch(action, &mut self.state)
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Handle global rendering

        // Handle mode specific rendering with mode context
        return self.mode.render_with_mode_context(frame, &self.state);
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

            // Handle OpenFile action specially to clear terminal after
            if let Action::OpenFile = &action {
                if let Err(e) = app.dispatch(action) {
                    eprintln!("Action error: {}", e);
                } else {
                    // Clear and redraw terminal after returning from vim
                    terminal.clear()?;
                }
            } else {
                if let Err(e) = app.dispatch(action) {
                    eprintln!("Action error: {}", e);
                }
            }
        }
    }
}


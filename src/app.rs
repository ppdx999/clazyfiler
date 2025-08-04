use std::io;

use crossterm::event::{self, Event, KeyEvent};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{prelude::Backend, Frame, Terminal};
use crate::{
    actions::{Action}, key::is_ctrl_c, modes::{interface::ModeBehavior, Mode}, state::AppState
};
    
pub struct App<B: Backend> {
    pub mode: Mode,
    pub state: AppState,
    terminal: Terminal<B>,
}

impl<B: Backend> App<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        let mut app = Self {
            mode: Mode::new_explore_mode(),
            state: AppState::new(),
            terminal,
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
            Action::OpenFile => self.open_file_with_vim(),
            // Handle mode specific action dispatch
            _ => self.mode.dispatch(action, &mut self.state)
        }
    }

    /// Open the selected file with vim, handling all terminal complexity internally
    fn open_file_with_vim(&mut self) -> Result<(), String> {
        let Some(selected) = self.state.get_selected_file() else {
            return Err("No file selected".to_string())
        };
        if selected.is_directory {
            return Err("Cannot open directory with vim".to_string())
        };

        // Save terminal state and switch to normal mode
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen).map_err(|e| format!("Failed to leave alternate screen: {}", e))?;
        disable_raw_mode().map_err(|e| format!("Failed to disable raw mode: {}", e))?;

        // Launch vim
        let result = self.state.open_file_with_vim(selected);

        // Restore terminal state and switch back to TUI mode
        execute!(stdout, EnterAlternateScreen).map_err(|e| format!("Failed to enter alternate screen: {}", e))?;
        enable_raw_mode().map_err(|e| format!("Failed to re-enable raw mode: {}", e))?;
        
        // Clear terminal and refresh after returning from vim
        self.terminal.clear().map_err(|e| format!("Failed to clear terminal: {}", e))?;

        match result {
            Ok(_) => {
                // Refresh files after returning from vim in case file was modified
                self.state.refresh_files();
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        // Handle global rendering

        // Handle mode specific rendering with mode context
        return self.mode.render_with_mode_context(frame, &self.state);
    }

    /// Draw the current state to the terminal
    pub fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            // Handle global rendering and mode specific rendering with mode context
            self.mode.render_with_mode_context(f, &self.state);
        })?;
        Ok(())
    }

    /// Main application loop - handles all events and terminal management
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            // Draw the current state
            self.draw()?;

            // Handle input events
            let Event::Key(key) = event::read()? else {
                continue;
            };

            let actions = self.handle_key(key);
            
            if actions.is_empty() {
                continue;
            }

            // Process all actions
            for action in actions {
                if let Action::Quit = &action {
                    return Ok(());
                }

                if let Err(e) = self.dispatch(action) {
                    eprintln!("Action error: {}", e);
                }
            }
        }
    }
}

pub fn run_app<B: Backend>(terminal: Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(terminal);
    app.run()
}


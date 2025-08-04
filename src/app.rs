use crossterm::event::{self, Event, KeyEvent};
use ratatui::{prelude::Backend, Terminal};
use crate::{
    actions::{Action}, key::is_ctrl_c, modes::{interface::ModeBehavior, Mode}, state::AppState, terminal::TerminalExt
};
    
pub struct App<B: Backend> {
    pub mode: Mode,
    pub state: AppState,
    terminal: Terminal<B>,
}

impl<B: Backend> App<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self {
            mode: Mode::new_explore_mode(),
            state: AppState::new(),
            terminal,
        }
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

    /// Open the selected file with vim, delegating terminal complexity to terminal layer
    fn open_file_with_vim(&mut self) -> Result<(), String> {
        // Get the selected file info before borrowing
        let selected_file = match self.state.get_selected_file() {
            Some(file) => file.clone(), // Clone the FileEntry to avoid borrowing issues
            None => return Err("No file selected".to_string()),
        };
        
        if selected_file.is_directory {
            return Err("Cannot open directory with vim".to_string())
        };

        // Use terminal's suspend/resume functionality to handle all terminal complexity
        let result = self.terminal.with_suspended_terminal(|| {
            self.state.open_file_with_vim(&selected_file).map_err(|e| e.into())
        });

        match result {
            Ok(_) => {
                // Refresh files after returning from vim in case file was modified
                self.state.refresh_files();
                Ok(())
            },
            Err(e) => Err(format!("Failed to open file with vim: {}", e))
        }
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


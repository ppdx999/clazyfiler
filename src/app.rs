use crossterm::event::{self, Event, KeyEvent};
use ratatui::{prelude::Backend, Terminal};
use crate::{
    handlers::{interface::KeyHandler, Handler}, key::is_ctrl_c, messages::{AppMessage, SwitchAction}, state::AppState, terminal::TerminalExt
};
    
pub struct App<B: Backend> {
    pub handler: Handler,
    pub state: AppState,
    terminal: Terminal<B>,
}

impl<B: Backend> App<B> {
    pub fn new(terminal: Terminal<B>) -> Self {
        Self {
            handler: Handler::new_explore_handler(),
            state: AppState::new(),
            terminal,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<AppMessage> {
        // Handle global key event
        if is_ctrl_c(&key) {
            return Some(AppMessage::Quit)
        }

        // Handle handler specific key event
        return self.handler.handle_key(key, &mut self.state)
    }

    /// Handle messages from handlers using clean message dispatching
    fn handle_message(&mut self, message: AppMessage) -> Result<bool, String> {
        
        match message {
            AppMessage::Quit => self.handle_quit(),
            AppMessage::OpenFile => self.handle_open_file(),
            AppMessage::SwitchMode(action) => self.handle_switch_handler(action),
            AppMessage::Error(error) => self.handle_error(error),
        }
    }
    
    /// Handle quit message
    fn handle_quit(&mut self) -> Result<bool, String> {
        Ok(true) // Signal to quit
    }
    
    /// Handle open file message
    fn handle_open_file(&mut self) -> Result<bool, String> {
        self.open_file_with_vim()?;
        Ok(false) // Continue running
    }
    
    /// Handle switch handler message
    fn handle_switch_handler(&mut self, action: SwitchAction) -> Result<bool, String> {
        self.handler.switch_to(action, &mut self.state)?;
        Ok(false) // Continue running
    }
    
    /// Handle error message
    fn handle_error(&mut self, error: String) -> Result<bool, String> {
        Err(error) // Propagate error to main loop
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
            // Handle global rendering and handler specific rendering with handler context
            self.handler.render_with_handler_context(f, &self.state);
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

            let message = self.handle_key(key);
            
            // Handle message if present
            if let Some(msg) = message {
                match self.handle_message(msg) {
                    Ok(should_quit) => {
                        if should_quit {
                            return Ok(());
                        }
                    },
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }
    }
}

pub fn run_app<B: Backend>(terminal: Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(terminal);
    app.run()
}


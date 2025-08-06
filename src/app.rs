use crossterm::event::{self, Event, KeyEvent};
use ratatui::{prelude::Backend, Terminal};
use crate::{
    handlers::Handler, key::is_ctrl_c, messages::AppMessage, state::AppState, terminal::TerminalExt
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

    /// Open the selected file with editor - delegates to state with terminal suspension
    fn open_file_with_editor(&mut self) -> Result<(), String> {
        self.terminal.with_suspended_terminal(|| {
            self.state.open_selected_file_with_editor().map_err(|e| e.into())
        }).map_err(|e| e.to_string())
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
                match msg {
                    AppMessage::Quit => return Ok(()),
                    AppMessage::OpenFile => self.open_file_with_editor()?,
                    AppMessage::SwitchToExploreHandler => {
                        self.state.update_explore_view();
                        self.handler.switch_to(&msg, &mut self.state)?;
                    },
                    AppMessage::SwitchToSearchHandler => {
                        self.state.update_search_view();
                        self.handler.switch_to(&msg, &mut self.state)?;
                    },
                    AppMessage::SwitchToFuzzyFindHandler => {
                        // Start fuzzy find indexing when switching to fuzzy find mode
                        if let Err(e) = self.state.start_fuzzy_find() {
                            return Err(format!("Failed to start fuzzy find: {}", e).into());
                        }
                        self.state.update_fuzzy_find_view();
                        self.handler.switch_to(&msg, &mut self.state)?;
                    },
                    AppMessage::Error(error) => Err(error)?,
                }
            }
        }
    }
}

pub fn run_app<B: Backend>(terminal: Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(terminal);
    app.run()
}


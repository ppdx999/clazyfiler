use crossterm::event::{self, Event, KeyEvent};
use ratatui::{prelude::Backend, Terminal};
use crate::{
    handlers::Handler, key::is_ctrl_c, messages::AppMessage, model::AppModel, 
    terminal::TerminalExt
};
    
pub struct App<B: Backend> {
    pub handler: Handler,
    pub model: AppModel,
    terminal: Terminal<B>,
}

impl<B: Backend> App<B> {
    pub fn new(terminal: Terminal<B>) -> Result<Self, Box<dyn std::error::Error>> {
        let model = AppModel::new()?;
        Ok(Self {
            handler: Handler::new_explore_handler(),
            model,
            terminal,
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> Option<AppMessage> {
        // Handle global key event
        if is_ctrl_c(&key) {
            return Some(AppMessage::Quit)
        }

        // Handle handler specific key event
        return self.handler.handle_key(key, &mut self.model)
    }

    /// Open the selected file with editor - delegates to model with terminal suspension
    fn open_file_with_editor(&mut self) -> Result<(), String> {
        self.terminal.with_suspended_terminal(|| {
            self.model.open_selected_file_with_editor().map_err(|e| e.into())
        }).map_err(|e| e.to_string())
    }


    /// Draw the current state to the terminal
    pub fn draw(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.terminal.draw(|f| {
            // Render directly with model - no ViewModels needed!
            self.handler.render_with_handler_context(f, &self.model);
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
                        self.model.switch_to_explore_mode();
                        self.handler.switch_to(&AppMessage::SwitchToExploreHandler, &mut self.model)?;
                    },
                    AppMessage::SwitchToExploreHandlerKeepQuery => {
                        self.model.switch_to_explore_mode_keep_query();
                        self.handler.switch_to(&AppMessage::SwitchToExploreHandler, &mut self.model)?;
                    },
                    AppMessage::SwitchToSearchHandler => {
                        self.model.switch_to_search_mode();
                        self.handler.switch_to(&msg, &mut self.model)?;
                    },
                    AppMessage::SwitchToFuzzyFindHandler => {
                        // Start fuzzy find indexing when switching to fuzzy find mode
                        if let Err(e) = self.model.switch_to_fuzzy_find_mode() {
                            return Err(format!("Failed to start fuzzy find: {}", e).into());
                        }
                        self.handler.switch_to(&msg, &mut self.model)?;
                    },
                    AppMessage::Error(error) => Err(error)?,
                }
            }
        }
    }
}

pub fn run_app<B: Backend>(terminal: Terminal<B>) -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new(terminal)?;
    app.run()
}


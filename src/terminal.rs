use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::{self, Stdout};

/// Terminal wrapper that handles setup and cleanup automatically
pub struct TerminalManager {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalManager {
    /// Initialize terminal with proper setup for TUI applications
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Enable raw mode for character-by-character input
        enable_raw_mode()?;
        
        // Setup stdout with alternate screen and mouse capture
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        
        // Create terminal backend and terminal
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(TerminalManager { terminal })
    }
    
    /// Get mutable reference to the underlying terminal
    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }
    
    /// Clean shutdown of terminal (called automatically on Drop)
    fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Disable raw mode
        disable_raw_mode()?;
        
        // Restore terminal state
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        
        // Show cursor
        self.terminal.show_cursor()?;
        
        Ok(())
    }
}

impl Drop for TerminalManager {
    /// Automatically cleanup terminal when TerminalManager is dropped
    fn drop(&mut self) {
        if let Err(e) = self.cleanup() {
            eprintln!("Error during terminal cleanup: {:?}", e);
        }
    }
}

/// RAII-style terminal management
/// 
/// This provides automatic setup and cleanup of terminal resources:
/// - Enables raw mode for character input
/// - Sets up alternate screen to preserve terminal state
/// - Enables mouse capture for potential future features  
/// - Automatically restores terminal state on drop
/// 
/// Example usage:
/// ```rust
/// let mut term_manager = TerminalManager::new()?;
/// let terminal = term_manager.terminal();
/// // Use terminal...
/// // Cleanup happens automatically when term_manager goes out of scope
/// ```
pub fn with_terminal<F, R>(f: F) -> Result<R, Box<dyn std::error::Error>>
where
    F: FnOnce(Terminal<CrosstermBackend<Stdout>>) -> Result<R, Box<dyn std::error::Error>>,
{
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    
    // Run the function with the terminal
    let result = f(terminal);
    
    // Cleanup terminal state
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    
    result
}

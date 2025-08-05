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

/// Extension trait for Terminal to handle suspend/resume operations for external commands
pub trait TerminalExt {
    /// Suspend the TUI terminal state to run an external command
    fn suspend_for_external_command(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Resume the TUI terminal state after external command finishes
    fn resume_from_external_command(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    
    /// Execute a closure with suspended terminal (convenience method)
    fn with_suspended_terminal<F, R>(&mut self, f: F) -> Result<R, Box<dyn std::error::Error>>
    where
        F: FnOnce() -> Result<R, Box<dyn std::error::Error>>;
}

impl<B: ratatui::backend::Backend> TerminalExt for Terminal<B> 
where B: ratatui::backend::Backend,
{
    fn suspend_for_external_command(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Leave alternate screen and disable raw mode for external command
        let mut stdout = io::stdout();
        execute!(stdout, LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }
    
    fn resume_from_external_command(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Re-enter alternate screen and enable raw mode
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        enable_raw_mode()?;
        
        // Clear terminal to remove any artifacts from external command
        self.clear()?;
        Ok(())
    }
    
    fn with_suspended_terminal<F, R>(&mut self, f: F) -> Result<R, Box<dyn std::error::Error>>
    where
        F: FnOnce() -> Result<R, Box<dyn std::error::Error>>
    {
        // Suspend terminal
        self.suspend_for_external_command()?;
        
        // Execute the function
        let result = f();
        
        // Resume terminal (even if function failed)
        self.resume_from_external_command()?;
        
        // Return the original result
        result
    }
}

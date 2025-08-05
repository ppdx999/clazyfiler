use std::process::Command;
use crate::core::{ClazyfilerError, Result};
use crate::state::FileEntry;

/// Service responsible for external editor integration
/// Handles $EDITOR, vim, or vi launching and editor detection
#[derive(Debug)]
pub struct EditorService;

impl EditorService {
    pub fn new() -> Self {
        Self
    }

    /// Open file with editor ($EDITOR, vim, or vi)
    pub fn open_file(&self, file: &FileEntry) -> Result<()> {
        if file.is_directory {
            return Err(ClazyfilerError::editor("editor", "Cannot open directory with editor"));
        }

        let editor = self.detect_editor()?;
        
        // Handle cases where $EDITOR might contain arguments (e.g., "code -w")
        let mut cmd = if editor.contains(' ') {
            let parts: Vec<&str> = editor.split_whitespace().collect();
            let mut command = Command::new(parts[0]);
            for arg in &parts[1..] {
                command.arg(arg);
            }
            command.arg(&file.path);
            command
        } else {
            let mut command = Command::new(&editor);
            command.arg(&file.path);
            command
        };

        let status = cmd
            .status()
            .map_err(|e| ClazyfilerError::editor(&editor, &format!("Failed to launch: {}", e)))?;

        if status.success() {
            Ok(())
        } else {
            Err(ClazyfilerError::editor(&editor, &format!("Editor exited with status: {}", status)))
        }
    }

    /// Detect available editor ($EDITOR first, then vim, then vi)
    fn detect_editor(&self) -> Result<String> {
        // Check $EDITOR environment variable first
        if let Ok(editor) = std::env::var("EDITOR") {
            if !editor.trim().is_empty() {
                // Verify the editor command exists
                if self.command_exists(&editor) {
                    return Ok(editor);
                }
            }
        }

        // Check if vim is available
        if self.command_exists("vim") {
            return Ok("vim".to_string());
        }

        // Fallback to vi
        if self.command_exists("vi") {
            return Ok("vi".to_string());
        }

        Err(ClazyfilerError::editor("detection", "No suitable editor found ($EDITOR, vim, or vi)"))
    }

    /// Check if a command exists and is executable
    fn command_exists(&self, command: &str) -> bool {
        Command::new("which")
            .arg(command)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

}

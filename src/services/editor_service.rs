use std::process::Command;
use crate::core::{ClazyfilerError, Result};
use crate::state::FileEntry;

/// Service responsible for external editor integration
/// Handles vim/vi launching and editor detection
#[derive(Debug)]
pub struct EditorService;

impl EditorService {
    pub fn new() -> Self {
        Self
    }

    /// Open file with vim/vi editor
    pub fn open_file(&self, file: &FileEntry) -> Result<()> {
        if file.is_directory {
            return Err(ClazyfilerError::editor("vim", "Cannot open directory with editor"));
        }

        let editor = self.detect_editor()?;
        
        let status = Command::new(&editor)
            .arg(&file.path)
            .status()
            .map_err(|e| ClazyfilerError::editor(&editor, &format!("Failed to launch: {}", e)))?;

        if status.success() {
            Ok(())
        } else {
            Err(ClazyfilerError::editor(&editor, &format!("Editor exited with status: {}", status)))
        }
    }

    /// Detect available editor (vim first, then vi)
    fn detect_editor(&self) -> Result<String> {
        // Check if vim is available
        if let Ok(output) = Command::new("which").arg("vim").output() {
            if output.status.success() {
                return Ok("vim".to_string());
            }
        }

        // Fallback to vi
        if let Ok(output) = Command::new("which").arg("vi").output() {
            if output.status.success() {
                return Ok("vi".to_string());
            }
        }

        Err(ClazyfilerError::editor("detection", "Neither vim nor vi is available on this system"))
    }

    /// Check if an editor is available
    pub fn is_editor_available(&self) -> bool {
        self.detect_editor().is_ok()
    }
}
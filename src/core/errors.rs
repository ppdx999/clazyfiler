use std::fmt;

/// Application-specific error types for better error handling and debugging
#[derive(Debug)]
pub enum ClazyfilerError {
    /// File system related errors (reading, writing, permissions)
    FileSystem { 
        operation: String, 
        path: String, 
        source: std::io::Error 
    },
    
    /// Editor/external command errors
    Editor { 
        command: String, 
        message: String 
    },
    
    /// Terminal operation errors
    Terminal { 
        operation: String, 
        message: String 
    },
    
    /// Configuration related errors
    Config { 
        message: String 
    },
    
    /// Search operation errors
    Search { 
        query: String, 
        message: String 
    },
    
    /// Navigation errors (invalid paths, permissions)
    Navigation { 
        path: String, 
        message: String 
    },
    
    /// Content reading/parsing errors
    Content { 
        file_path: String, 
        message: String 
    },
}

impl fmt::Display for ClazyfilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClazyfilerError::FileSystem { operation, path, source } => {
                write!(f, "File system error during '{}' on '{}': {}", operation, path, source)
            }
            ClazyfilerError::Editor { command, message } => {
                write!(f, "Editor error with '{}': {}", command, message)
            }
            ClazyfilerError::Terminal { operation, message } => {
                write!(f, "Terminal error during '{}': {}", operation, message)
            }
            ClazyfilerError::Config { message } => {
                write!(f, "Configuration error: {}", message)
            }
            ClazyfilerError::Search { query, message } => {
                write!(f, "Search error for '{}': {}", query, message)
            }
            ClazyfilerError::Navigation { path, message } => {
                write!(f, "Navigation error for '{}': {}", path, message)
            }
            ClazyfilerError::Content { file_path, message } => {
                write!(f, "Content error for '{}': {}", file_path, message)
            }
        }
    }
}

impl std::error::Error for ClazyfilerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClazyfilerError::FileSystem { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Convenience type alias for Results with ClazyfilerError
pub type Result<T> = std::result::Result<T, ClazyfilerError>;

/// Helper functions for creating common errors
impl ClazyfilerError {
    pub fn file_system(operation: &str, path: &str, source: std::io::Error) -> Self {
        Self::FileSystem {
            operation: operation.to_string(),
            path: path.to_string(),
            source,
        }
    }
    
    pub fn editor(command: &str, message: &str) -> Self {
        Self::Editor {
            command: command.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn terminal(operation: &str, message: &str) -> Self {
        Self::Terminal {
            operation: operation.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn config(message: &str) -> Self {
        Self::Config {
            message: message.to_string(),
        }
    }
    
    pub fn search(query: &str, message: &str) -> Self {
        Self::Search {
            query: query.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn navigation(path: &str, message: &str) -> Self {
        Self::Navigation {
            path: path.to_string(),
            message: message.to_string(),
        }
    }
    
    pub fn content(file_path: &str, message: &str) -> Self {
        Self::Content {
            file_path: file_path.to_string(),
            message: message.to_string(),
        }
    }
}

/// Convert from common error types
impl From<std::io::Error> for ClazyfilerError {
    fn from(err: std::io::Error) -> Self {
        Self::FileSystem {
            operation: "unknown".to_string(),
            path: "unknown".to_string(),
            source: err,
        }
    }
}

impl From<Box<dyn std::error::Error>> for ClazyfilerError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        Self::Config {
            message: err.to_string(),
        }
    }
}

/// Convert to String for compatibility with existing code
impl From<ClazyfilerError> for String {
    fn from(err: ClazyfilerError) -> Self {
        err.to_string()
    }
}
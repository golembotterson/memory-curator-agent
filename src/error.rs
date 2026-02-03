// Error types for Memory Curator

use thiserror::Error;
use std::io;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, CuratorError>;

#[derive(Error, Debug)]
pub enum CuratorError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Invalid config: {0}")]
    InvalidConfig(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Memory file corruption detected")]
    MemoryFileCorrupted,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl CuratorError {
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            CuratorError::FileNotFound(_)
                | CuratorError::ParseError(_)
                | CuratorError::MemoryFileCorrupted
        )
    }
}

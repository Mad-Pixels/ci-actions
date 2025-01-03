use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Source not available: {0}")]
    NotAvailable(String),

    #[error("Failed to read from source: {0}")]
    ReadError(String),

    #[error("Failed to write to source: {0}")]
    WriteError(String),

    #[error("Operation not supported: {0}")]
    Unsupported(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Environment error: {0}")]
    EnvError(String),

    #[error("File error at path '{path}': {message}")]
    FileError { path: PathBuf, message: String },
}

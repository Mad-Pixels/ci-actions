use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExecuterError {
    #[error("Command validation error: {0}")]
    ValidationError(String),

    #[error("Command execution error: {0}")]
    ExecutionError(String),

    #[error("Stream error: {0}")]
    StreamError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Environment error: {0}")]
    EnvironmentError(String),
}

pub type ExecuterResult<T> = Result<T, ExecuterError>;

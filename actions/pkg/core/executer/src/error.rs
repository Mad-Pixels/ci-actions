use thiserror::Error;


/// Represents the various errors that can occur during command execution.
#[derive(Error, Debug)]
pub enum ExecuterError {
    /// Error related to command validation.
    #[error("Command validation error: {0}")]
    ValidationError(String),

    /// Error that occurs during command execution.
    #[error("Command execution error: {0}")]
    ExecutionError(String),

    /// Error related to stream processing (stdout/stderr).
    #[error("Stream error: {0}")]
    StreamError(String),

    /// Input/Output error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Error related to environment variables.
    #[error("Environment error: {0}")]
    EnvironmentError(String),
}

/// A type alias for results returned by executer operations.
///
/// This alias simplifies the return type by encapsulating a `Result`
/// that uses `ExecuterError` for error handling.
///
/// # Examples
///
/// ```rust
/// use executer::{ExecuterError, ExecuterResult};
///
/// fn example_function(success: bool) -> ExecuterResult<String> {
///     if success {
///         Ok("Success!".to_string())
///     } else {
///         Err(ExecuterError::ExecutionError("Failed to execute command".to_string()))
///     }
/// }
///
/// let result: ExecuterResult<String> = example_function(true);
/// assert_eq!(result.unwrap(), "Success!");
/// ```
pub type ExecuterResult<T> = Result<T, ExecuterError>;

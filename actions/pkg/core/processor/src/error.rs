use thiserror::Error;

/// Represents the various errors that can occur during processing.
#[derive(Error, Debug)]
pub enum ProcessorError {
    /// Error related to regular expressions.
    #[error("Regex error: {0}")]
    RegexError(String),
}

/// A type alias for results returned by processor operations.
///
/// This alias simplifies the return type by encapsulating a `Result`
pub type ProcessorResult<T> = Result<T, ProcessorError>;

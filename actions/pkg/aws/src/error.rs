use thiserror::Error;

/// Represents the different errors that can occur when executing AWS commands.
#[derive(Error, Debug)]
pub enum AwsError {
    /// Error when an AWS command fails.
    #[error("AWS command failed: {0}")]
    CommandError(String),

    /// Error from the underlying executor.
    #[error(transparent)]
    ExecuterError(#[from] executer::ExecuterError),
}

/// A type alias for results returned by AWS operations.
pub type AwsResult<T> = Result<T, AwsError>;

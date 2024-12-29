use thiserror::Error;

#[derive(Error, Debug)]
pub enum AWSError {
    #[error("AWS command failed: {0}")]
    CommandError(String),

    #[error("S3 operation error: {0}")]
    S3Error(String),

    #[error("Lambda operation error: {0}")]
    LambdaError(String),

    #[error(transparent)]
    ExecutionError(#[from] executer::ExecuterError),
}

pub type AWSResult<T> = Result<T, AWSError>;

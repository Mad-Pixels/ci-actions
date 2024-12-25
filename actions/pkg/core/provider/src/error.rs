use thiserror::Error;

/// Represents the various errors that can occur within the provider module.
#[derive(Error, Debug)]
pub enum ProviderError {
    /// Error indicating that a required environment variable is missing.
    #[error("Missing required environment variable: {0}")]
    MissingEnvironmentVariable(String),

    /// Error indicating that the environment configuration is invalid.
    #[error("Invalid environment configuration: {0}")]
    InvalidConfiguration(String),

    /// Error indicating that provider not found but required.
    #[error("No supported provider detected in environment")]
    ProviderNotFound,
}

/// A type alias for results returned by provider operations.
///
/// This alias simplifies the return type by encapsulating a `Result`
pub type ProviderResult<T> = Result<T, ProviderError>;

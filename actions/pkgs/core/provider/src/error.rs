use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("Missing required environment variable: {0}")]
    MissingEnvironmentVariable(String),
    
    #[error("Invalid environment configuration: {0}")]
    InvalidConfiguration(String),
}

pub type ProviderResult<T> = Result<T, ProviderError>;

use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Required;

#[derive(Debug)]
pub enum ConfigError {
    RequiredValueMissing(String),
    EnvVarMissing(String),
    InvalidValue(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::RequiredValueMissing(key) => write!(f, "Required value missing for: {}", key),
            ConfigError::EnvVarMissing(var) => write!(f, "Required environment variable missing: {}", var),
            ConfigError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
        }
    }
}

impl Error for ConfigError {}

pub type ConfigResult<T> = Result<T, ConfigError>;
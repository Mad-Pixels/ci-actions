use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Expected {expected}, got {actual}")]
    WrongType {
        expected: &'static str,
        actual: &'static str,
    },

    #[error("Value conversion failed: {0}")]
    ConversionError(String),

    #[error("Invalid number format: {0}")]
    NumberError(String),
}

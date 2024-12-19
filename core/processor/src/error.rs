use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("Regex error: {0}")]
    RegexError(String),
}

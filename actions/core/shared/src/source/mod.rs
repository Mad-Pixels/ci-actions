mod env;
mod error;
mod file;

pub use env::EnvSource;
pub use error::SourceError;
pub use file::FileFormat;
pub use file::FileSource;

use std::collections::HashMap;

use crate::types::RawValue;

pub trait Source: Send + Sync {
    fn name(&self) -> &str;

    fn is_available(&self) -> bool;

    fn load(&self) -> Result<HashMap<String, RawValue>, SourceError>;

    fn save(&self, _values: &HashMap<String, RawValue>) -> Result<(), SourceError> {
        Err(SourceError::Unsupported(
            "Save operation is not supported for this source".into(),
        ))
    }

    fn get(&self, key: &str) -> Result<Option<RawValue>, SourceError>;

    fn set(&self, _key: &str, _value: RawValue) -> Result<(), SourceError> {
        Err(SourceError::Unsupported(
            "Set operation is not supported for this source".into(),
        ))
    }
}

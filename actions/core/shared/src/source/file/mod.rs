mod format;
mod json;
mod yaml;

pub use format::{FileFormat, Format};

use std::{collections::HashMap, path::PathBuf};

use super::{Source, SourceError};
use crate::types::RawValue;

pub struct FileSource {
    path: PathBuf,
    format: FileFormat,
    cache: Option<HashMap<String, RawValue>>,
}

impl FileSource {
    pub fn new(path: impl Into<PathBuf>, format: FileFormat) -> Self {
        Self {
            path: path.into(),
            format,
            cache: None,
        }
    }

    pub fn clear_cache(&mut self) {
        self.cache = None;
    }
}

impl Source for FileSource {
    fn name(&self) -> &str {
        "file"
    }

    fn is_available(&self) -> bool {
        self.path.exists()
    }

    fn get(&self, key: &str) -> Result<Option<RawValue>, SourceError> {
        let values = self.load()?;
        Ok(values.get(key).cloned())
    }

    fn load(&self) -> Result<HashMap<String, RawValue>, SourceError> {
        if let Some(ref cache) = self.cache {
            return Ok(cache.clone());
        }

        let content = std::fs::read_to_string(&self.path).map_err(|e| SourceError::FileError {
            path: self.path.clone(),
            message: e.to_string(),
        })?;
        self.format.parse(&content)
    }

    fn save(&self, values: &HashMap<String, RawValue>) -> Result<(), SourceError> {
        let content = self.format.serialize(values)?;
        std::fs::write(&self.path, content).map_err(|e| SourceError::FileError {
            path: self.path.clone(),
            message: e.to_string(),
        })?;
        Ok(())
    }
}

use std::collections::HashMap;

use super::{json, yaml, SourceError};
use crate::types::RawValue;

pub trait Format: Send + Sync {
    fn parse(&self, content: &str) -> Result<HashMap<String, RawValue>, SourceError>;
    fn serialize(&self, values: &HashMap<String, RawValue>) -> Result<String, SourceError>;
}

pub enum FileFormat {
    Json,
    Yaml,
}

impl Format for FileFormat {
    fn parse(&self, content: &str) -> Result<HashMap<String, RawValue>, SourceError> {
        match self {
            FileFormat::Json => json::parse(content),
            FileFormat::Yaml => yaml::parse(content),
        }
    }

    fn serialize(&self, values: &HashMap<String, RawValue>) -> Result<String, SourceError> {
        match self {
            FileFormat::Json => json::serialize(values),
            FileFormat::Yaml => yaml::serialize(values),
        }
    }
}

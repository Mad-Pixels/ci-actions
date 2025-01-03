use std::collections::{HashMap, HashSet};
use std::env;

use super::{Source, SourceError};
use crate::types::RawValue;

pub struct EnvSource {
    prefix: String,

    sensitive_keys: HashSet<String>,

    cache: Option<HashMap<String, RawValue>>,
}

impl EnvSource {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            sensitive_keys: HashSet::new(),
            cache: None,
        }
    }

    pub fn with_sensitive_keys(mut self, keys: impl IntoIterator<Item = String>) -> Self {
        self.sensitive_keys.extend(keys);
        self
    }

    pub fn clear_cache(&mut self) {
        self.cache = None;
    }

    fn is_sensitive(&self, key: &str) -> bool {
        self.sensitive_keys.contains(key)
    }
}

impl Source for EnvSource {
    fn name(&self) -> &str {
        "environment"
    }

    fn is_available(&self) -> bool {
        true
    }

    fn load(&self) -> Result<HashMap<String, RawValue>, SourceError> {
        if let Some(ref cache) = self.cache {
            return Ok(cache.clone());
        }

        let values: HashMap<String, RawValue> = env::vars()
            .filter_map(|(key, value)| {
                if key.starts_with(&self.prefix) {
                    let clean_key = key[self.prefix.len()..].to_string();
                    Some((clean_key, RawValue::String(value)))
                } else {
                    None
                }
            })
            .collect();
        Ok(values)
    }

    fn get(&self, key: &str) -> Result<Option<RawValue>, SourceError> {
        let env_key = format!("{}{}", self.prefix, key);
        match env::var(&env_key) {
            Ok(value) => Ok(Some(RawValue::String(value))),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(e) => Err(SourceError::EnvError(e.to_string())),
        }
    }

    fn set(&self, key: &str, value: RawValue) -> Result<(), SourceError> {
        let env_key = format!("{}{}", self.prefix, key);
        match value {
            RawValue::String(s) => env::set_var(env_key, s),
            _ => {
                return Err(SourceError::InvalidFormat(
                    "Environment variables can only store strings".into(),
                ))
            }
        }
        Ok(())
    }
}

use crate::error::{ConfigError, ConfigResult, Required};
use std::{env, path::PathBuf};

#[derive(Debug, Clone)]
pub struct ConfigValue<T> {
    default: Option<T>,
    env_key: &'static str,
}

impl<T> ConfigValue<T> {
    pub const fn new(default: T, env_key: &'static str) -> Self {
        Self {
            default: Some(default),
            env_key,
        }
    }

    pub const fn required(env_key: &'static str) -> ConfigValue<Required> {
        ConfigValue {
            default: None,
            env_key,
        }
    }
}

impl ConfigValue<String> {
    pub fn get(&self) -> ConfigResult<String> {
        match env::var(self.env_key) {
            Ok(val) => Ok(val),
            Err(_) => {
                if let Some(default) = &self.default {
                    Ok(default.clone())
                } else {
                    Err(ConfigError::RequiredValueMissing(self.env_key.to_string()))
                }
            }
        }
    }
}

impl ConfigValue<PathBuf> {
    pub fn get(&self) -> ConfigResult<PathBuf> {
        match env::var(self.env_key) {
            Ok(val) => Ok(PathBuf::from(val)),
            Err(_) => {
                if let Some(default) = &self.default {
                    Ok(default.clone())
                } else {
                    Err(ConfigError::RequiredValueMissing(self.env_key.to_string()))
                }
            }
        }
    }
}

impl ConfigValue<Required> {
    pub fn get<T: std::str::FromStr>(&self) -> ConfigResult<T> {
        match env::var(self.env_key) {
            Ok(val) => val.parse::<T>().map_err(|_| {
                ConfigError::InvalidValue(format!("Cannot parse value for: {}", self.env_key))
            }),
            Err(_) => Err(ConfigError::RequiredValueMissing(self.env_key.to_string())),
        }
    }
}

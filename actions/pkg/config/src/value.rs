use crate::{validator::Validator, ConfigError, ConfigResult, Required};
use std::{env, path::PathBuf};

/// Represents a configuration value that can be retrieved from an environment variable.
/// It may have a default value and a set of validators to ensure the value meets
/// certain criteria.
#[derive(Clone)]
pub struct ConfigValue<T> {
    /// The default value if the environment variable is not set.
    default: Option<T>,

    /// The key of the environment variable.
    env_key: &'static str,

    /// A list of validators to validate the retrieved value.
    validators: Vec<Box<dyn Validator<T>>>,
}

impl<T: Clone> ConfigValue<T> {
    /// Creates a new `ConfigValue` with a default value and an environment variable key.
    ///
    /// # Arguments
    ///
    /// * `default` - The default value to use if the environment variable is not set.
    /// * `env_key` - The key of the environment variable to retrieve.
    ///
    /// # Example
    ///
    /// ```rust
    /// use config::ConfigValue;
    /// use std::path::PathBuf;
    ///
    /// let config_value = ConfigValue::new(PathBuf::from("/default/path"), "CONFIG_PATH");
    /// ```
    pub fn new(default: T, env_key: &'static str) -> Self {
        Self {
            default: Some(default),
            env_key,
            validators: Vec::new(),
        }
    }

    /// Attaches a validator to the `ConfigValue`.
    ///
    /// # Arguments
    ///
    /// * `validator` - An object implementing the `Validator<T>` trait.
    ///
    /// # Example
    ///
    /// ```rust
    /// use config::{ConfigValue, DirExists};
    /// use std::path::PathBuf;
    ///
    /// let config_value = ConfigValue::new(PathBuf::from("/default/dir"), "WORKING_DIR")
    ///     .with_validator(DirExists);
    /// ```
    pub fn with_validator<V>(mut self, validator: V) -> Self
    where
        V: 'static + Validator<T>,
    {
        self.validators.push(Box::new(validator));
        self
    }
}

impl ConfigValue<Required> {
    /// Creates a new `ConfigValue` that is required (no default value).
    ///
    /// # Arguments
    ///
    /// * `env_key` - The key of the environment variable to retrieve.
    ///
    /// # Example
    ///
    /// ```rust
    /// use config::{ConfigValue, Required};
    ///
    /// let required_value: ConfigValue<Required> = ConfigValue::required("REQUIRED_VAR");
    /// ```
    pub const fn required(env_key: &'static str) -> ConfigValue<Required> {
        ConfigValue {
            default: None,
            env_key,
            validators: Vec::new(),
        }
    }
}

impl ConfigValue<String> {
    /// Retrieves the `String` configuration value.
    ///
    /// It first attempts to read the value from the environment variable.
    /// If not set, it uses the default value (if any). After retrieving the value,
    /// all attached validators are executed to ensure the value is valid.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::RequiredValueMissing` if the environment variable
    /// is not set and no default value is provided.
    ///
    /// Returns `ConfigError::InvalidValue` if any validator fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use config::ConfigValue;
    ///
    /// let config_value = ConfigValue::new("default".to_string(), "MY_VAR");
    /// let value = config_value.get().unwrap();
    /// assert_eq!(value, "default");
    /// ```
    pub fn get(&self) -> ConfigResult<String> {
        let val = match env::var(self.env_key) {
            Ok(val) => val,
            Err(_) => {
                if let Some(default) = &self.default {
                    default.clone()
                } else {
                    return Err(ConfigError::RequiredValueMissing(self.env_key.to_string()));
                }
            }
        };
        for validator in &self.validators {
            validator.validate(&val)?;
        }
        Ok(val)
    }
}

impl ConfigValue<PathBuf> {
    /// Retrieves the `PathBuf` configuration value.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::RequiredValueMissing` if the environment variable
    /// is not set and no default value is provided.
    ///
    /// Returns `ConfigError::InvalidValue` if any validator fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use config::{ConfigValue, DirExists};
    /// use std::path::PathBuf;
    /// use std::env;
    /// use tempfile::tempdir;
    ///
    /// let temp = tempdir().unwrap();
    /// let temp_path = temp.path().to_path_buf();
    ///
    /// let config_value = ConfigValue::new(temp_path.clone(), "WORKING_DIR")
    ///     .with_validator(DirExists);
    ///
    /// // Set environment variable to the temp directory
    /// env::set_var("WORKING_DIR", temp_path.to_str().unwrap());
    ///
    /// let path = config_value.get().unwrap();
    /// assert_eq!(path, temp_path);
    /// ```
    pub fn get(&self) -> ConfigResult<PathBuf> {
        let val = match env::var(self.env_key) {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                if let Some(default) = &self.default {
                    default.clone()
                } else {
                    return Err(ConfigError::RequiredValueMissing(self.env_key.to_string()));
                }
            }
        };
        for validator in &self.validators {
            validator.validate(&val)?;
        }
        Ok(val)
    }
}

impl ConfigValue<Required> {
    /// Retrieves a required configuration value of a generic type `T`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to parse the configuration value into. Must implement `FromStr`.
    ///
    /// # Errors
    ///
    /// Returns `ConfigError::RequiredValueMissing` if the environment variable is not set.
    /// Returns `ConfigError::InvalidValue` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use config::{ConfigValue, Required};
    /// use std::env;
    ///
    /// // Set the environment variable for the example
    /// env::set_var("REQUIRED_INT", "42");
    ///
    /// let required_value: ConfigValue<Required> = ConfigValue::required("REQUIRED_INT");
    /// let parsed: i32 = required_value.get().unwrap();
    /// assert_eq!(parsed, 42);
    /// ```
    pub fn get<T: std::str::FromStr>(&self) -> ConfigResult<T> {
        let val_str = match env::var(self.env_key) {
            Ok(val_str) => val_str,
            Err(_) => {
                return Err(ConfigError::RequiredValueMissing(self.env_key.to_string()));
            }
        };
        let parsed = val_str.parse::<T>().map_err(|_| {
            ConfigError::InvalidValue(format!("Cannot parse value for: {}", self.env_key))
        })?;
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validator::DirExists;
    use std::env;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_string_with_default() {
        let config = ConfigValue::new("default_value".to_string(), "TEST_STRING");
        env::remove_var("TEST_STRING");
        let value = config.get().unwrap();
        assert_eq!(value, "default_value");
    }

    #[test]
    fn test_string_from_env() {
        let config = ConfigValue::new("default_value".to_string(), "TEST_STRING");
        env::set_var("TEST_STRING", "env_value");
        let value = config.get().unwrap();
        assert_eq!(value, "env_value");
    }

    #[test]
    fn test_string_required_missing() {
        let config: ConfigValue<Required> = ConfigValue::required("REQUIRED_STRING");
        env::remove_var("REQUIRED_STRING");
        let result = config.get::<String>();
        assert!(matches!(result, Err(ConfigError::RequiredValueMissing(_))));
    }

    #[test]
    fn test_string_required_present() {
        let config: ConfigValue<Required> = ConfigValue::required("REQUIRED_STRING");
        env::set_var("REQUIRED_STRING", "present");
        let value = config.get::<String>().unwrap();
        assert_eq!(value, "present");
    }

    #[test]
    fn test_pathbuf_with_default() {
        let default_path = PathBuf::from("/default/path");
        let config = ConfigValue::new(default_path.clone(), "TEST_PATHBUF");
        env::remove_var("TEST_PATHBUF");
        let path = config.get().unwrap();
        assert_eq!(path, default_path);
    }

    #[test]
    fn test_pathbuf_from_env() {
        let env_path = "/env/path";
        let config = ConfigValue::new(PathBuf::from("/default/path"), "TEST_PATHBUF");
        env::set_var("TEST_PATHBUF", env_path);
        let path = config.get().unwrap();
        assert_eq!(path, PathBuf::from(env_path));
    }

    #[test]
    fn test_required_get_parsed() {
        let config: ConfigValue<Required> = ConfigValue::required("REQUIRED_INT");
        env::set_var("REQUIRED_INT", "42");
        let value: i32 = config.get().unwrap();
        assert_eq!(value, 42);
    }

    #[test]
    fn test_required_get_parse_error() {
        let config: ConfigValue<Required> = ConfigValue::required("REQUIRED_INT");
        env::set_var("REQUIRED_INT", "not_an_int");
        let result: ConfigResult<i32> = config.get();
        assert!(matches!(result, Err(ConfigError::InvalidValue(_))));
    }

    #[test]
    fn test_with_validator_success() {
        let temp_dir = tempdir().unwrap();
        let config =
            ConfigValue::new(temp_dir.path().to_path_buf(), "VALID_DIR").with_validator(DirExists);
        env::set_var("VALID_DIR", temp_dir.path());
        let path = config.get().unwrap();
        assert_eq!(path, temp_dir.path());
    }

    #[test]
    fn test_with_validator_failure() {
        let config = ConfigValue::new(PathBuf::from("/non/existent/dir"), "VALID_DIR")
            .with_validator(DirExists);
        env::set_var("VALID_DIR", "/non/existent/dir");
        let result = config.get();
        assert!(matches!(result, Err(ConfigError::InvalidValue(_))));
    }

    #[test]
    fn test_clone_config_value() {
        let config =
            ConfigValue::new("value".to_string(), "CLONE_TEST").with_validator(|v: &String| {
                if v.is_empty() {
                    Err(ConfigError::InvalidValue("Empty string".to_string()))
                } else {
                    Ok(())
                }
            });
        let cloned = config.clone();
        env::set_var("CLONE_TEST", "cloned_value");
        let original = config.get().unwrap();
        let clone_val = cloned.get().unwrap();
        assert_eq!(original, "cloned_value");
        assert_eq!(clone_val, "cloned_value");
    }

    #[test]
    fn test_clone_config_value_with_named_validator() {
        #[derive(Clone)]
        struct NonEmptyValidator;

        impl Validator<String> for NonEmptyValidator {
            fn validate(&self, value: &String) -> ConfigResult<()> {
                if value.is_empty() {
                    Err(ConfigError::InvalidValue("String is empty".to_string()))
                } else {
                    Ok(())
                }
            }

            fn clone_box(&self) -> Box<dyn Validator<String>> {
                Box::new(self.clone())
            }
        }

        let config = ConfigValue::new("value".to_string(), "CLONE_TEST_NAMED")
            .with_validator(NonEmptyValidator);
        let cloned = config.clone();
        env::set_var("CLONE_TEST_NAMED", "cloned_value_named");
        let original = config.get().unwrap();
        let clone_val = cloned.get().unwrap();
        assert_eq!(original, "cloned_value_named");
        assert_eq!(clone_val, "cloned_value_named");
    }
}

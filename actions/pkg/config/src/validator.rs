use crate::{ConfigError, ConfigResult};
use std::path::PathBuf;

/// The `Validator` trait defines a common interface for validating configuration values.
///
/// Implementors of this trait can perform any necessary checks on a configuration value
/// and return a `ConfigResult<()>` indicating success or the appropriate `ConfigError`.
pub trait Validator<T>: Send + Sync {
    /// Validates the provided value.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the value to validate.
    ///
    /// # Errors
    ///
    /// Returns a `ConfigError::InvalidValue` if the validation fails.
    fn validate(&self, value: &T) -> ConfigResult<()>;

    /// Creates a boxed clone of the validator.
    ///
    /// This method is necessary to allow cloning of trait objects.
    fn clone_box(&self) -> Box<dyn Validator<T>>;
}

impl<T> Clone for Box<dyn Validator<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl<T, F> Validator<T> for F
where
    F: 'static + Fn(&T) -> ConfigResult<()> + Clone + Send + Sync,
{
    fn validate(&self, value: &T) -> ConfigResult<()> {
        self(value)
    }

    fn clone_box(&self) -> Box<dyn Validator<T>> {
        Box::new(self.clone())
    }
}

/// Validator that checks if a `PathBuf` points to an existing file.
#[derive(Clone)]
pub struct FileExists;

impl Validator<PathBuf> for FileExists {
    fn validate(&self, value: &PathBuf) -> ConfigResult<()> {
        if !value.is_file() {
            return Err(ConfigError::InvalidValue(format!(
                "File does not exist: {:?}",
                value
            )));
        }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Validator<PathBuf>> {
        Box::new(self.clone())
    }
}

/// Validator that checks if a `PathBuf` points to an existing directory.
#[derive(Clone)]
pub struct DirExists;

impl Validator<PathBuf> for DirExists {
    fn validate(&self, value: &PathBuf) -> ConfigResult<()> {
        if !value.is_dir() {
            return Err(ConfigError::InvalidValue(format!(
                "Directory does not exist: {:?}",
                value
            )));
        }
        Ok(())
    }

    fn clone_box(&self) -> Box<dyn Validator<PathBuf>> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_file_exists_success() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        File::create(&file_path).unwrap();

        let validator = FileExists;
        let result = validator.validate(&file_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_exists_failure() {
        let path = PathBuf::from("/non/existent/file.txt");
        let validator = FileExists;
        let result = validator.validate(&path);
        assert!(matches!(result, Err(ConfigError::InvalidValue(_))));
    }

    #[test]
    fn test_dir_exists_success() {
        let dir = tempdir().unwrap();
        let validator = DirExists;
        let result = validator.validate(&dir.path().to_path_buf());
        assert!(result.is_ok());
    }

    #[test]
    fn test_dir_exists_failure() {
        let path = PathBuf::from("/non/existent/dir");
        let validator = DirExists;
        let result = validator.validate(&path);
        assert!(matches!(result, Err(ConfigError::InvalidValue(_))));
    }

    #[test]
    fn test_custom_validator_success() {
        let non_empty = |s: &String| -> ConfigResult<()> {
            if s.is_empty() {
                Err(ConfigError::InvalidValue("String is empty".to_string()))
            } else {
                Ok(())
            }
        };

        let validator: Box<dyn Validator<String>> = Box::new(non_empty.clone());
        let result = validator.validate(&"valid".to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_custom_validator_failure() {
        // Define a custom validator that ensures a string is not empty
        let non_empty = |s: &String| -> ConfigResult<()> {
            if s.is_empty() {
                Err(ConfigError::InvalidValue("String is empty".to_string()))
            } else {
                Ok(())
            }
        };

        let validator: Box<dyn Validator<String>> = Box::new(non_empty.clone());
        let result = validator.validate(&"".to_string());
        assert!(matches!(result, Err(ConfigError::InvalidValue(_))));
    }

    #[test]
    fn test_validator_clone() {
        let validator = DirExists.clone();
        let dir = tempdir().unwrap();
        let path = dir.path().to_path_buf();
        let result = validator.validate(&path);
        assert!(result.is_ok());
    }
}
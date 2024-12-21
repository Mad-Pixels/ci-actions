use crate::{Context, ExecuterError, ValidationRule};

/// A validation rule that ensures the working directory exists and is a directory.
///
/// The `PathRule` checks if the specified working directory exists and is indeed a directory.
pub struct PathRule;

impl PathRule {
    /// Creates a new `PathRule`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::validate::rules::PathRule;
    
    /// let rule = PathRule::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl ValidationRule for PathRule {
    /// Validates the working directory in the context.
    ///
    /// Ensures that the working directory exists and is a directory.
    ///
    /// # Arguments
    ///
    /// * `context` - The context containing the working directory to validate.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if the working directory does not exist or is not a directory.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::{ValidationRule, Context, ExecuterError};
    /// use executer::validate::rules::PathRule;
    /// use std::collections::HashMap;
    /// use std::path::PathBuf;
    /// 
    /// let rule = PathRule::new();
    ///
    /// let valid_context = Context::new(vec!["echo".to_string()], HashMap::new(), Some(PathBuf::from(".")));
    /// assert!(rule.validate(&valid_context).is_ok());
    ///
    /// let invalid_context = Context::new(vec!["echo".to_string()], HashMap::new(), Some(PathBuf::from("/nonexistent/path")));
    /// assert!(rule.validate(&invalid_context).is_err());
    /// ```
    fn validate(&self, context: &Context) -> Result<(), ExecuterError> {
        if let Some(path) = &context.cwd {
            if !path.exists() {
                return Err(ExecuterError::ValidationError(format!(
                    "Working directory does not exist: {}",
                    path.display()
                )));
            }
            if !path.is_dir() {
                return Err(ExecuterError::ValidationError(format!(
                    "Path is not a directory: {}",
                    path.display()
                )));
            }
        }
        Ok(())
    }

    /// Returns the name of the validation rule.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::validate::rules::PathRule;
    /// use executer::ValidationRule;
    ///
    /// let rule = PathRule::new();
    /// assert_eq!(rule.name(), "path");
    /// ```
    fn name(&self) -> &'static str {
        "path"
    }

    /// Returns the priority of the validation rule.
    ///
    /// Lower numbers indicate higher priority.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::validate::rules::PathRule;
    /// use executer::ValidationRule;
    ///
    /// let rule = PathRule::new();
    /// assert_eq!(rule.priority(), 2);
    /// ```
    fn priority(&self) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn create_context(cwd: Option<PathBuf>) -> Context {
        Context::new(vec!["test".to_string()], HashMap::new(), cwd)
    }

    #[test]
    fn test_no_path() {
        let rule = PathRule::new();
        let context = create_context(None);
        assert!(rule.validate(&context).is_ok());
    }

    #[test]
    fn test_valid_path() {
        let rule = PathRule::new();
        let context = create_context(Some(PathBuf::from(".")));
        assert!(rule.validate(&context).is_ok());
    }

    #[test]
    fn test_nonexistent_path() {
        let rule = PathRule::new();
        let context = create_context(Some(PathBuf::from("/nonexistent/path")));
        assert!(rule.validate(&context).is_err());
    }
}

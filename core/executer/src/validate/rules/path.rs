use crate::validate::rule::ValidationContext;
use crate::validate::traits::ValidationRule;
use crate::error::ExecuterError;

pub struct PathRule;

impl PathRule {
    pub fn new() -> Self {
        Self
    }
}

impl ValidationRule for PathRule {
    fn validate(&self, context: &ValidationContext) -> Result<(), ExecuterError> {
        if let Some(path) = &context.cwd {
            if !path.exists() {
                return Err(ExecuterError::ValidationError(
                    format!("Working directory does not exist: {}", path.display())
                ));
            }
            if !path.is_dir() {
                return Err(ExecuterError::ValidationError(
                    format!("Path is not a directory: {}", path.display())
                ));
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str { "path" }

    fn priority(&self) -> i32 { 2 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn create_context(cwd: Option<PathBuf>) -> ValidationContext {
        ValidationContext::new(vec!["test".to_string()], HashMap::new(), cwd)
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
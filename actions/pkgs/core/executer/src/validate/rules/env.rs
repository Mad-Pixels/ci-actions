use crate::{Context, ExecuterError, ValidationRule};

/// A validation rule that ensures environment variables have valid names and values.
///
/// The `EnvRule` checks that each environment variable has a non-empty name and value.
pub struct EnvRule;

impl EnvRule {
    /// Creates a new `EnvRule`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::validate::rules::EnvRule;
    ///
    /// let rule = EnvRule::new();
    /// ```
    pub fn new() -> Self {
        Self
    }
}

impl ValidationRule for EnvRule {
    /// Validates the environment variables in the context.
    ///
    /// Ensures that each environment variable has a non-empty name and value.
    ///
    /// # Arguments
    ///
    /// * `context` - The context containing the environment variables to validate.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if any environment variable has an empty name or value.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::{ValidationRule, Context, ExecuterError};
    /// use executer::validate::rules::EnvRule;
    /// use std::collections::HashMap;
    /// 
    /// let rule = EnvRule::new();
    ///
    /// let mut valid_env = HashMap::new();
    /// valid_env.insert("PATH".to_string(), "/usr/bin".to_string());
    /// let valid_context = Context::new(vec!["echo".to_string()], valid_env, None);
    /// assert!(rule.validate(&valid_context).is_ok());
    ///
    /// let mut invalid_env = HashMap::new();
    /// invalid_env.insert("".to_string(), "value".to_string());
    /// let invalid_context = Context::new(vec!["echo".to_string()], invalid_env, None);
    /// assert!(rule.validate(&invalid_context).is_err());
    /// ```
    fn validate(&self, context: &Context) -> Result<(), ExecuterError> {
        for (key, value) in &context.env {
            if key.trim().is_empty() {
                return Err(ExecuterError::ValidationError(
                    "Environment variable name cannot be empty".to_string(),
                ));
            }
            if value.trim().is_empty() {
                return Err(ExecuterError::ValidationError(format!(
                    "Environment variable '{}' has empty value",
                    key
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
    /// use executer::validate::rules::EnvRule;
    /// use executer::ValidationRule;
    ///
    /// let rule = EnvRule::new();
    /// assert_eq!(rule.name(), "environment");
    /// ```
    fn name(&self) -> &'static str {
        "environment"
    }

    /// Returns the priority of the validation rule.
    ///
    /// Lower numbers indicate higher priority.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::validate::rules::EnvRule;
    /// use executer::ValidationRule;
    ///
    /// let rule = EnvRule::new();
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

    fn create_context(env: HashMap<String, String>) -> Context {
        Context::new(vec!["test".to_string()], env, None)
    }

    #[test]
    fn test_valid_env() {
        let mut env = HashMap::new();
        env.insert("KEY".to_string(), "value".to_string());

        let rule = EnvRule::new();
        let context = create_context(env);
        assert!(rule.validate(&context).is_ok());
    }

    #[test]
    fn test_empty_key() {
        let mut env = HashMap::new();
        env.insert("".to_string(), "value".to_string());

        let rule = EnvRule::new();
        let context = create_context(env);
        assert!(rule.validate(&context).is_err());
    }

    #[test]
    fn test_empty_value() {
        let mut env = HashMap::new();
        env.insert("KEY".to_string(), "".to_string());

        let rule = EnvRule::new();
        let context = create_context(env);
        assert!(rule.validate(&context).is_err());
    }
}

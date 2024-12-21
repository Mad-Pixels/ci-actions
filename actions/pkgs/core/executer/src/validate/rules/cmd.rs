use crate::{Context, ExecuterError, ValidationRule};

/// A validation rule that ensures commands do not contain forbidden characters.
///
/// The `CmdRule` checks each argument in the command for any characters
/// that are deemed unsafe or potentially harmful.
pub struct CmdRule {
    forbidden_chars: Vec<char>,
}

impl CmdRule {
    /// Creates a new `CmdRule` with default forbidden characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::rules::CmdRule;
    ///
    /// let rule = CmdRule::new();
    /// ```
    pub fn new() -> Self {
        Self {
            forbidden_chars: vec!['&', '|', ';', '`', '\\'],
        }
    }

    /// Creates a new `CmdRule` with a custom list of forbidden characters.
    ///
    /// # Arguments
    ///
    /// * `chars` - A vector of characters that are considered forbidden.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::rules::CmdRule;
    ///
    /// let rule = CmdRule::with_forbidden_chars(vec!['$', '#']);
    /// ```
    pub fn with_forbidden_chars(chars: Vec<char>) -> Self {
        Self {
            forbidden_chars: chars,
        }
    }
}

impl ValidationRule for CmdRule {
    /// Validates the command context by checking for forbidden characters.
    ///
    /// # Arguments
    ///
    /// * `context` - The context containing the command to validate.
    ///
    /// # Errors
    ///
    /// Returns a `ValidationError` if any command argument contains forbidden characters.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::{ValidationRule, Context, ExecuterError};
    /// use executer::rules::CmdRule;
    /// use std::collections::HashMap;
    ///
    /// let rule = CmdRule::new();
    /// let context = Context::new(vec!["ls".to_string(), "-l".to_string()], HashMap::new(), None);
    /// assert!(rule.validate(&context).is_ok());
    ///
    /// let bad_context = Context::new(vec!["ls".to_string(), "&".to_string()], HashMap::new(), None);
    /// assert!(rule.validate(&bad_context).is_err());
    /// ```
    fn validate(&self, context: &Context) -> Result<(), ExecuterError> {
        if context.command.is_empty() {
            return Err(ExecuterError::ValidationError(
                "Empty command sequence".to_string(),
            ));
        }
        for (i, arg) in context.command.iter().enumerate() {
            if i > 0 && context.command[i - 1] == "-c" {
                continue;
            }
            if arg.chars().any(|c| self.forbidden_chars.contains(&c)) {
                return Err(ExecuterError::ValidationError(format!(
                    "Invalid command argument '{}': contains forbidden characters",
                    arg
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
    /// use executer::rules::CmdRule;
    /// use executer::ValidationRule;
    ///
    /// let rule = CmdRule::new();
    /// assert_eq!(rule.name(), "cmd");
    /// ```
    fn name(&self) -> &'static str {
        "cmd"
    }

    /// Returns the priority of the validation rule.
    ///
    /// Lower numbers indicate higher priority.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::rules::CmdRule;
    /// use executer::ValidationRule;
    ///
    /// let rule = CmdRule::new();
    /// assert_eq!(rule.priority(), 0);
    /// ```
    fn priority(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn create_context(cmd: Vec<String>) -> Context {
        Context::new(cmd, HashMap::new(), None)
    }

    #[test]
    fn test_empty_command() {
        let rule = CmdRule::new();
        let context = create_context(vec![]);
        assert!(rule.validate(&context).is_err());
    }

    #[test]
    fn test_valid_command() {
        let rule = CmdRule::new();
        let context = create_context(vec!["ls".to_string(), "-l".to_string()]);
        assert!(rule.validate(&context).is_ok());
    }

    #[test]
    fn test_shell_command() {
        let rule = CmdRule::new();
        let context = create_context(vec![
            "sh".to_string(),
            "-c".to_string(),
            "echo $HOME".to_string(),
        ]);
        assert!(rule.validate(&context).is_ok());
    }

    #[test]
    fn test_invalid_command() {
        let rule = CmdRule::new();
        let context = create_context(vec!["ls".to_string(), "&".to_string()]);
        assert!(rule.validate(&context).is_err());
    }
}

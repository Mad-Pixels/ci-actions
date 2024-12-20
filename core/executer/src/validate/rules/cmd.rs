use crate::validate::traits::ValidationRule;
use crate::error::ExecuterError;
use crate::context::Context;

pub struct CmdRule {
    forbidden_chars: Vec<char>,
}

impl CmdRule {
    pub fn new() -> Self {
        Self {
            forbidden_chars: vec!['&', '|', ';', '`', '\\'],
        }
    }

    pub fn with_forbidden_chars(chars: Vec<char>) -> Self {
        Self {
            forbidden_chars: chars,
        }
    }
}

impl ValidationRule for CmdRule {
    fn validate(&self, context: &Context) -> Result<(), ExecuterError> {
        if context.command.is_empty() {
            return Err(ExecuterError::ValidationError("Empty command sequence".to_string()));
        }
        for (i, arg) in context.command.iter().enumerate() {
            if i > 0 && context.command[i-1] == "-c" {
                continue;
            }
            if arg.chars().any(|c| self.forbidden_chars.contains(&c)) {
                return Err(ExecuterError::ValidationError(
                    format!("Invalid command argument '{}': contains forbidden characters", arg)
                ));
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str { "cmd" }

    fn priority(&self) -> i32 { 0 }
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
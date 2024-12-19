mod validator;
mod rules;
mod rule;

pub use rule::{ValidationRule, ValidationContext};
pub use rules::{CmdRule, EnvRule, PathRule};
pub use validator::Validator;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::path::PathBuf;

    fn create_cmd_context(cmd: Vec<String>) -> ValidationContext {
        ValidationContext::new(cmd, HashMap::new(), None)
    }

    fn create_env_context(env: HashMap<String, String>) -> ValidationContext {
        ValidationContext::new(vec!["test".to_string()], env, None)
    }

    fn create_path_context(path: Option<PathBuf>) -> ValidationContext {
        ValidationContext::new(vec!["test".to_string()], HashMap::new(), path)
    }

    #[test]
    fn test_valid_command_validation() {
        let validator = Validator::default();
        let context = create_cmd_context(vec!["ls".to_string(), "-l".to_string()]);
        assert!(validator.validate(&context).is_ok());
    }

    #[test]
    fn test_invalid_command_validation() {
        let validator = Validator::default();
        
        let context = create_cmd_context(vec![]);
        assert!(validator.validate(&context).is_err());

        let context = create_cmd_context(vec!["ls".to_string(), "&".to_string()]);
        assert!(validator.validate(&context).is_err());
    }

    #[test]
    fn test_valid_env_validation() {
        let validator = Validator::default();
        let mut env = HashMap::new();
        env.insert("PATH".to_string(), "/usr/bin".to_string());
        let context = create_env_context(env);
        assert!(validator.validate(&context).is_ok());
    }

    #[test]
    fn test_invalid_env_validation() {
        let validator = Validator::default();
        
        let mut env = HashMap::new();
        env.insert("TEST".to_string(), "".to_string());
        let context = create_env_context(env);
        assert!(validator.validate(&context).is_err());
    }

    #[test]
    fn test_valid_path_validation() {
        let validator = Validator::default();
        let context = create_path_context(Some(PathBuf::from(".")));
        assert!(validator.validate(&context).is_ok());
    }

    #[test]
    fn test_invalid_path_validation() {
        let validator = Validator::default();
        let context = create_path_context(Some(PathBuf::from("/path/that/does/not/exist")));
        assert!(validator.validate(&context).is_err());
    }

    #[test]
    fn test_validation_order() {
        use rule::ValidationRule;

        struct TestRule {
            name: &'static str,
            priority: i32,
            should_fail: bool,
        }

        impl ValidationRule for TestRule {
            fn validate(&self, _: &ValidationContext) -> crate::error::ExecuterResult<()> {
                if self.should_fail {
                    Err(crate::error::ExecuterError::ValidationError(
                        "Test failure".to_string(),
                    ))
                } else {
                    Ok(())
                }
            }

            fn name(&self) -> &'static str {
                self.name
            }

            fn priority(&self) -> i32 {
                self.priority
            }
        }

        let rules: Vec<Box<dyn ValidationRule>> = vec![
            Box::new(TestRule {
                name: "low priority",
                priority: 10,
                should_fail: false,
            }) as Box<dyn ValidationRule>,
            Box::new(TestRule {
                name: "high priority",
                priority: 1,
                should_fail: true,
            }) as Box<dyn ValidationRule>,
        ];
        let validator = Validator::new(rules);
        let context = ValidationContext::new(vec![], HashMap::new(), None);
        
        assert!(validator.validate(&context).is_err());
    }

    #[test]
    fn test_shell_command_validation() {
        let validator = Validator::default();
        let context = create_cmd_context(vec![
            "sh".to_string(),
            "-c".to_string(),
            "echo $HOME".to_string(),
        ]);
        assert!(validator.validate(&context).is_ok());
    }

    #[test]
    fn test_complex_validation() {
        let validator = Validator::default();
        let mut env = HashMap::new();
        env.insert("TEST_VAR".to_string(), "test_value".to_string());

        let context = ValidationContext::new(
            vec!["ls".to_string(), "-l".to_string()],
            env,
            Some(PathBuf::from(".")),
        );
        assert!(validator.validate(&context).is_ok());
    }
}
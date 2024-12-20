use crate::validate::traits::ValidationRule;
use crate::error::ExecuterError;
use crate::context::Context;

pub struct EnvRule;

impl EnvRule {
    pub fn new() -> Self {
        Self
    }
}

impl ValidationRule for EnvRule {
    fn validate(&self, context: &Context) -> Result<(), ExecuterError> {
        for (key, value) in &context.env {
            if key.trim().is_empty() {
                return Err(ExecuterError::ValidationError(
                    "Environment variable name cannot be empty".to_string()
                ));
            }
            if value.trim().is_empty() {
                return Err(ExecuterError::ValidationError(
                    format!("Environment variable '{}' has empty value", key)
                ));
            }
        }
        Ok(())
    }

    fn name(&self) -> &'static str { "environment" }

    fn priority(&self) -> i32 { 2 }
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
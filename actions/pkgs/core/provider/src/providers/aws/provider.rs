use std::collections::HashMap;

use crate::error::{ProviderError, ProviderResult};
use crate::Provider;

use super::constants::REQUIRED_ENV_VARS;
use super::patterns::AWS_PATTERNS;

#[derive(Clone)]
pub struct AWSProvider {
    environment: HashMap<String, String>,
}

impl AWSProvider {
    pub fn new(environment: HashMap<String, String>) -> Self {
        Self { environment }
    }

    fn validate(&self) -> ProviderResult<()> {
        for var in REQUIRED_ENV_VARS {
            if !self.environment.contains_key(*var) {
                return Err(ProviderError::MissingEnvironmentVariable(var.to_string()));
            }
        }
        Ok(())
    }
}

impl Provider for AWSProvider {
    fn get_environment(&self) -> HashMap<String, String> {
        self.environment.clone()
    }

    fn get_sensitive(&self) -> HashMap<String, String> {
        self.environment.clone()
    }

    fn get_predefined_masked_objects(&self) -> Vec<String> {
        AWS_PATTERNS.to_vec()
    }

    fn validate(&self) -> ProviderResult<()> {
        self.validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_env() -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("AWS_ACCESS_KEY_ID".to_string(), "key".to_string());
        env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "secret".to_string());
        env
    }

    #[test]
    fn test_new_and_get_environment() {
        let env = create_test_env();
        let aws = AWSProvider::new(env.clone());
        assert_eq!(aws.get_environment(), env);
    }

    #[test]
    fn test_get_sensitive() {
        let env = create_test_env();
        let aws = AWSProvider::new(env.clone());
        assert_eq!(aws.get_sensitive(), env);
    }

    #[test]
    fn test_validate_success() {
        let env = create_test_env();
        let aws = AWSProvider::new(env);
        assert!(aws.validate().is_ok());
    }

    #[test]
    fn test_validate_missing_key() {
        let env = HashMap::new();
        let aws = AWSProvider::new(env);
        match aws.validate() {
            Err(ProviderError::MissingEnvironmentVariable(var)) => {
                assert_eq!(var, "AWS_ACCESS_KEY_ID");
            }
            _ => panic!("Expected MissingEnvironmentVariable error"),
        }
    }

    #[test]
    fn test_get_predefined_masked_objects() {
        let aws = AWSProvider::new(HashMap::new());
        let masked_objects = aws.get_predefined_masked_objects();
        assert!(!masked_objects.is_empty());
        assert!(masked_objects[0].contains("arn:aws:iam"));
    }
}
use std::collections::HashMap;
use std::env;

use crate::error::{ProviderError, ProviderResult};
use crate::Provider;

use super::constants::REQUIRED_ENV_VARS;
use super::patterns::AWS_PATTERNS;

/// AWS Cloud Provider implementation.
///
/// The `AWSProvider` struct manages AWS-specific environment variables,
/// validates their presence, and provides predefined patterns for masking
/// sensitive AWS resources.
#[derive(Clone)]
pub struct AWSProvider {
    /// Environment variables for AWS.
    environment: HashMap<String, String>,
}

impl AWSProvider {
    /// Creates a new AWSProvider instance with the given environment variables.
    ///
    /// # Arguments
    ///
    /// * `environment` - A `HashMap` containing AWS-related environment variables.
    ///
    /// # Example
    ///
    /// ```rust
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// let mut env = HashMap::new();
    /// env.insert("AWS_ACCESS_KEY_ID".to_string(), "test-key".to_string());
    /// env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "test-secret".to_string());
    ///
    /// let aws_provider = AWSProvider::new(env.clone());
    /// ```
    pub fn new(environment: HashMap<String, String>) -> Self {
        Self { environment }
    }

    /// Validates that all required environment variables are present.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if all required variables are present.
    /// - `Err(ProviderError::MissingEnvironmentVariable)` if any required variable is missing.
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
    /// Retrieves all environment variables related to AWS.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing AWS environment variables as key-value pairs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// let mut env = HashMap::new();
    /// env.insert("AWS_ACCESS_KEY_ID".to_string(), "test-key".to_string());
    /// env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "test-secret".to_string());
    ///
    /// let aws_provider = AWSProvider::new(env.clone());
    /// let environment = aws_provider.get_environment();
    /// assert_eq!(environment.get("AWS_ACCESS_KEY_ID").unwrap(), "test-key");
    /// ```
    fn get_environment(&self) -> HashMap<String, String> {
        self.environment.clone()
    }

    /// Retrieves predefined patterns for masking sensitive AWS resources.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing regex patterns as strings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// let aws_provider = AWSProvider::new(HashMap::new());
    /// let patterns = aws_provider.get_predefined_masked_objects();
    /// assert!(!patterns.is_empty());
    /// ```
    fn get_predefined_masked_objects(&self) -> Vec<String> {
        AWS_PATTERNS.to_vec()
    }

    /// Validates the AWS provider configuration by ensuring all required
    /// environment variables are present.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if validation succeeds.
    /// - `Err(ProviderError)` if validation fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// use provider::{AWSProvider, Provider, ProviderError};
    /// use std::collections::HashMap;
    ///
    /// let mut env = HashMap::new();
    /// env.insert("AWS_ACCESS_KEY_ID".to_string(), "test-key".to_string());
    /// env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "test-secret".to_string());
    ///
    /// let aws_provider = AWSProvider::new(env.clone());
    /// assert!(aws_provider.validate().is_ok());
    ///
    /// let invalid_provider = AWSProvider::new(HashMap::new());
    /// assert!(invalid_provider.validate().is_err());
    /// ```
    fn validate(&self) -> ProviderResult<()> {
        self.validate()
    }

    /// Cleans up provider-specific environment variables.
    ///
    /// This method removes all environment variables used by the AWS provider.
    /// It's useful for cleaning up the environment after tests or when
    /// switching between different cloud providers.
    ///
    /// # Example
    ///
    /// ```rust
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    /// use std::env;
    ///
    /// // Set environment variables
    /// env::set_var("AWS_ACCESS_KEY_ID", "test-key");
    ///
    /// let aws_provider = AWSProvider::new(HashMap::new());
    ///
    /// // Clean up
    /// aws_provider.clean();
    /// assert!(env::var("AWS_ACCESS_KEY_ID").is_err());
    /// ```
    fn clean(&self) {
        for var in REQUIRED_ENV_VARS {
            env::remove_var(var);
        }
    }

    /// Returns all environment variable values as a vector.
    ///
    /// # Returns
    ///
    /// A `Vec<String>` containing all environment variable values.
    ///
    /// # Example
    ///
    /// ```rust
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// let mut env = HashMap::new();
    /// env.insert("AWS_ACCESS_KEY_ID".to_string(), "test-key".to_string());
    /// env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "test-secret".to_string());
    ///
    /// let provider: Box<dyn Provider> = Box::new(AWSProvider::new(env));
    /// let values = provider.values();
    /// assert_eq!(values.len(), 2);
    /// assert!(values.contains(&"test-key"));
    /// assert!(values.contains(&"test-secret"));
    /// ```
    fn values(&self) -> Vec<&str> {
        self.environment.values().map(|s| s.as_str()).collect()
    }

    /// Return Provider name.
    fn name(&self) -> String {
        "AWS".to_string()
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

    #[test]
    fn test_clean_missing_vars() {
        let aws = AWSProvider::new(HashMap::new());
        aws.clean();
    }

    #[test]
    fn test_values() {
        let env = create_test_env();
        let aws: Box<dyn Provider> = Box::new(AWSProvider::new(env));
        let values = aws.values();

        assert_eq!(values.len(), 2);
        assert!(values.contains(&"key"));
        assert!(values.contains(&"secret"));
    }
}

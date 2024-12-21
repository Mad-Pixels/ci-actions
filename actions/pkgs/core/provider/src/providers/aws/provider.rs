use std::collections::HashMap;

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

    /// Retrieves sensitive environment variables that should be protected.
    ///
    /// # Returns
    ///
    /// A `HashMap` containing sensitive AWS environment variables as key-value pairs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// let mut env = HashMap::new();
    /// env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "secret".to_string());
    ///
    /// let aws_provider = AWSProvider::new(env.clone());
    /// let sensitive = aws_provider.get_sensitive();
    /// assert_eq!(sensitive.get("AWS_SECRET_ACCESS_KEY").unwrap(), "secret");
    /// ```
    fn get_sensitive(&self) -> HashMap<String, String> {
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

//! # Provider Crate
//!
//! The `provider` crate offers implementations for various cloud providers,
//! facilitating environment variable management, configuration validation.
//!
//! ## Modules
//!
//! - [`error`]: Defines error types and result aliases used across the crate.
//! - [`providers`]: Contains implementations of specific cloud providers.
//! - [`traits`]: Defines the `Provider` trait that all providers must implement.
//!
//! ## Usage
//!
//! Below is a basic example of how to create and validate an AWS provider.
//!
//! ```rust
//! use provider::{Provider, AWSProvider, ProviderError};
//! use std::collections::HashMap;
//!
//! fn main() -> Result<(), ProviderError> {
//!     // Setup environment variables
//!     let mut env = HashMap::new();
//!     env.insert("AWS_ACCESS_KEY_ID".to_string(), "test-key".to_string());
//!     env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "test-secret".to_string());
//!
//!     // Initialize AWS provider
//!     let aws_provider = AWSProvider::new(env);
//!
//!     // Validate provider configuration
//!     aws_provider.validate()?;
//!
//!     // Retrieve environment variables
//!     let environment = aws_provider.get_environment();
//!     println!("AWS Environment: {:?}", environment);
//!
//!     // Retrieve environment values
//!     let values = aws_provider.values();
//!     println!("Sensitive Variables: {:?}", values);
//!
//!     // Retrieve predefined masked objects
//!     let masked_objects = aws_provider.get_predefined_masked_objects();
//!     println!("Masked Patterns: {:?}", masked_objects);
//! 
//!     // Retrieve provider name
//!     println!("Provider: {}", aws_provider.name());
//! 
//!     // Remove creadentials from environment variables
//!     aws_provider.clean();
//!
//!     Ok(())
//! }
//! ```

mod providers;
mod traits;
mod error;

use std::{collections::HashMap, env};
use crate::providers::aws::constants::REQUIRED_ENV_VARS;

pub use error::{ProviderError, ProviderResult};
pub use providers::aws::AWSProvider;
pub use traits::Provider;

/// Attempts to automatically detect and create a provider based on environment variables.
///
/// # Returns
///
/// - `Ok(Box<dyn Provider>)` if a supported provider is detected
/// - `Err(ProviderError::ProviderNotFound)` if no supported provider is detected
///
/// # Example
///
/// ```rust
/// use std::env;
/// use provider::auto_detect;
///
/// env::set_var("AWS_ACCESS_KEY_ID", "key");
/// env::set_var("AWS_SECRET_ACCESS_KEY", "secret");
///
/// let provider = auto_detect().expect("Failed to detect provider");
/// assert!(provider.validate().is_ok());
/// ```
pub fn auto_detect() -> ProviderResult<Box<dyn Provider>> {
    let env_vars: HashMap<String, String> = env::vars().collect();

    let has_aws = REQUIRED_ENV_VARS.iter()
        .all(|var| env_vars.contains_key(*var));

    if has_aws {
        let filtered_vars: HashMap<String, String> = REQUIRED_ENV_VARS
            .iter()
            .filter_map(|&key| {
                env_vars.get(key)
                    .map(|value| (key.to_string(), value.to_string()))
            })
            .collect();
        return Ok(Box::new(AWSProvider::new(filtered_vars)));
    }

    // Add checks for other providers here when they are added
    // Example:
    // if has_gcp {
    //     return Ok(Box::new(GCPProvider::new(env_vars)));
    // }

    Err(ProviderError::ProviderNotFound)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::collections::HashMap;

    fn cleanup_env() {
        env::remove_var("AWS_ACCESS_KEY_ID");
        env::remove_var("AWS_SECRET_ACCESS_KEY");
        // Add other provider vars when implemented
    }

    fn setup_aws_credentials() -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("AWS_ACCESS_KEY_ID".to_string(), "test-key".to_string());
        env.insert(
            "AWS_SECRET_ACCESS_KEY".to_string(),
            "test-secret".to_string(),
        );
        env
    }

    #[test]
    fn test_aws_provider_validation_failure() {
        let provider = AWSProvider::new(HashMap::new());

        let result = provider.validate();
        assert!(result.is_err());

        match result {
            Err(ProviderError::MissingEnvironmentVariable(var)) => {
                assert_eq!(var, "AWS_ACCESS_KEY_ID");
            }
            _ => panic!("Expected MissingEnvironmentVariable error"),
        }
    }

    #[test]
    fn test_aws_provider_masking_patterns() {
        let provider = AWSProvider::new(HashMap::new());
        let patterns = provider.get_predefined_masked_objects();
        assert!(!patterns.is_empty());

        let has_iam = patterns.iter().any(|p| p.contains("arn:aws:iam"));
        let has_s3 = patterns.iter().any(|p| p.contains("arn:aws:s3"));
        let has_lambda = patterns.iter().any(|p| p.contains("arn:aws:lambda"));

        assert!(has_iam, "Should contain IAM patterns");
        assert!(has_s3, "Should contain S3 patterns");
        assert!(has_lambda, "Should contain Lambda patterns");
        for pattern in patterns {
            regex::Regex::new(&pattern).expect("Pattern should be valid regex");
        }
    }

    #[test]
    fn test_environment_isolation() {
        let mut env1 = setup_aws_credentials();
        let env2 = env1.clone();

        env1.insert("EXTRA_VAR".to_string(), "value".to_string());
        let provider1 = AWSProvider::new(env1);
        let provider2 = AWSProvider::new(env2.clone());

        assert_ne!(provider1.get_environment(), provider2.get_environment());
        assert_eq!(
            provider1.get_environment().get("AWS_ACCESS_KEY_ID"),
            provider2.get_environment().get("AWS_ACCESS_KEY_ID")
        );
    }

    #[test]
    fn test_auto_detect_aws() {
        cleanup_env();
        
        env::set_var("AWS_ACCESS_KEY_ID", "test-key");
        env::set_var("AWS_SECRET_ACCESS_KEY", "test-secret");

        let provider = auto_detect().expect("Should detect AWS provider");
        assert!(provider.validate().is_ok());
        
        cleanup_env();
    }

    #[test]
    fn test_auto_detect_none() {
        cleanup_env();
        
        match auto_detect() {
            Err(ProviderError::ProviderNotFound) => (),
            _ => panic!("Should return ProviderNotFound when no provider detected"),
        }
    }

    #[test]
    fn test_auto_detect_partial_aws() {
        cleanup_env();
        
        env::set_var("AWS_ACCESS_KEY_ID", "test-key");
        // Missing AWS_SECRET_ACCESS_KEY

        match auto_detect() {
            Err(ProviderError::ProviderNotFound) => (),
            _ => panic!("Should return ProviderNotFound when AWS credentials are incomplete"),
        }
        
        cleanup_env();
    }
}

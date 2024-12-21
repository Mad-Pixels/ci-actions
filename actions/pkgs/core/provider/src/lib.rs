mod error;
mod providers;
mod traits;

pub use error::ProviderError;
pub use providers::aws::AWSProvider;
pub use traits::Provider;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
    fn test_aws_provider_lifecycle() {
        let env = setup_aws_credentials();

        let provider = AWSProvider::new(env.clone());
        assert!(provider.validate().is_ok());

        let environment = provider.get_environment();
        assert_eq!(environment.get("AWS_ACCESS_KEY_ID").unwrap(), "test-key");
        assert_eq!(
            environment.get("AWS_SECRET_ACCESS_KEY").unwrap(),
            "test-secret"
        );

        let sensitive = provider.get_sensitive();
        assert_eq!(sensitive, environment);
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
}

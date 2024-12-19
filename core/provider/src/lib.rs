pub mod aws;

use std::collections::HashMap;

pub trait Provider {
    fn get_environment(&self) -> HashMap<String, String>;
    fn get_sensitive(&self) -> HashMap<String, String>;
    fn validate(&self) -> Result<(), String>;
    fn get_predefined_masked_objects(&self) -> Vec<String> { Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aws::AWS;

    #[test]
    fn test_aws_new_and_get_environment() {
        let mut env = HashMap::new();
        env.insert("AWS_ACCESS_KEY_ID".to_string(), "key".to_string());
        env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "secret".to_string());

        let aws = AWS::new(env.clone());
        assert_eq!(aws.get_environment(), env);
    }

    #[test]
    fn test_aws_get_sensitive() {
        let mut env = HashMap::new();
        env.insert("AWS_ACCESS_KEY_ID".to_string(), "key".to_string());
        env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "secret".to_string());

        let aws = AWS::new(env.clone());
        assert_eq!(aws.get_sensitive(), env);
    }

    #[test]
    fn test_aws_validate_success() {
        let mut env = HashMap::new();
        env.insert("AWS_ACCESS_KEY_ID".to_string(), "key".to_string());
        env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "secret".to_string());

        let aws = AWS::new(env);
        assert!(aws.validate().is_ok());
    }

    #[test]
    fn test_aws_validate_missing_key() {
        let env = HashMap::new();
        let aws = AWS::new(env);
        let result = aws.validate();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing required key: AWS_ACCESS_KEY_ID");
    }

    #[test]
    fn test_aws_get_predefined_masked_objects() {
        let env = HashMap::new();
        let aws = AWS::new(env);

        let masked_objects = aws.get_predefined_masked_objects();
        assert!(masked_objects.len() > 0);
        assert!(masked_objects[0].contains("arn:aws:iam"));
    }
}

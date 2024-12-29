use std::collections::HashMap;
use std::env;

/// Manages AWS environment variables.
pub struct AwsEnv {
    environment: HashMap<String, String>,
}

impl AwsEnv {
    pub fn new() -> Self {
        let environment: HashMap<String, String> = env::vars()
            .filter(|(key, _)| key.starts_with("AWS_VAR_"))
            .map(|(key, value)| {
                let name = key.strip_prefix("AWS_VAR_").unwrap();
                (name.to_lowercase(), value)
            })
            .collect();

        Self { environment }
    }

    pub fn values(&self) -> Vec<&str> {
        self.environment.values().map(|s| s.as_str()).collect()
    }

    pub fn as_map(&self) -> HashMap<String, String> {
        self.environment.clone()
    }

    fn normalize_key(key: &str) -> String {
        let key = key.to_lowercase();
        if key.starts_with("aws_var_") {
            key.strip_prefix("aws_var_").unwrap().to_string()
        } else {
            key
        }
    }

    pub fn add(&mut self, key: &str, value: String) {
        let key = Self::normalize_key(key);
        self.environment.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        let key = Self::normalize_key(key);
        self.environment.remove(&key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        let key = Self::normalize_key(key);
        self.environment.contains_key(&key)
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        let key = Self::normalize_key(key);
        self.environment.get(&key)
    }
}

impl Default for AwsEnv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_env() {
        env::set_var("AWS_VAR_REGION", "us-west-2");
        env::set_var("AWS_VAR_INSTANCE_TYPE", "t2.micro");
        env::set_var("NOT_AWS_VAR", "should-not-be-included");

        let env = AwsEnv::new();

        let values = env.values();
        assert!(values.contains(&"us-west-2"));
        assert!(values.contains(&"t2.micro"));
        assert!(!values.contains(&"should-not-be-included"));

        // Check that keys are lowercase
        assert!(env.contains_key("region"));
        assert!(env.contains_key("instance_type"));

        env::remove_var("AWS_VAR_REGION");
        env::remove_var("AWS_VAR_INSTANCE_TYPE");
        env::remove_var("NOT_AWS_VAR");
    }

    #[test]
    fn test_add_and_remove() {
        let mut env = AwsEnv::new();

        env.add("REGION", "us-east-1".to_string());
        assert_eq!(env.get("region").unwrap(), "us-east-1");
        assert_eq!(env.get("REGION").unwrap(), "us-east-1"); // Should work with any case

        env.add("INSTANCE_TYPE", "t3.micro".to_string());
        assert_eq!(env.get("instance_type").unwrap(), "t3.micro");
        assert_eq!(env.get("INSTANCE_TYPE").unwrap(), "t3.micro");

        assert_eq!(env.remove("REGION").unwrap(), "us-east-1");
        assert!(env.get("region").is_none());
    }

    #[test]
    fn test_contains_key() {
        let mut env = AwsEnv::new();
        env.add("TEST_KEY", "test_value".to_string());

        assert!(env.contains_key("test_key"));
        assert!(env.contains_key("TEST_KEY"));
        assert!(!env.contains_key("non_existent_key"));
    }

    #[test]
    fn test_case_insensitive() {
        env::set_var("AWS_VAR_PROJECT_NAME", "test-project");
        let env = AwsEnv::new();
        assert_eq!(env.get("project_name").unwrap(), "test-project");
        assert_eq!(env.get("PROJECT_NAME").unwrap(), "test-project");

        env::remove_var("AWS_VAR_PROJECT_NAME");
    }
}

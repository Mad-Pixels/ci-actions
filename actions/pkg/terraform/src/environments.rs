use std::collections::HashMap;
use std::env;

pub struct TerraformEnv {
    environment: HashMap<String, String>,
}

impl TerraformEnv {
    pub fn new() -> Self {
        let environment: HashMap<String, String> = env::vars()
            .filter(|(key, _)| key.starts_with("TF_VAR_"))
            .map(|(key, value)| {
                let (prefix, name) = key.split_at(7);
                let lowercase_key = format!("{}{}", prefix, name.to_lowercase());
                (lowercase_key, value)
            })
            .collect();

        Self { environment }
    }

    /// Returns all values of Terraform environment variables
    pub fn values(&self) -> Vec<&str> {
        self.environment.values().map(|s| s.as_str()).collect()
    }

    /// Returns all environment variables as HashMap
    pub fn as_map(&self) -> HashMap<String, String> {
        self.environment
            .iter()
            .map(|(key, value)| {
                let clean_key = key.strip_prefix("TF_VAR_").unwrap_or(key).to_string();
                (clean_key, value.clone())
            })
            .collect()
    }

    /// Add new Terraform environment variable
    pub fn add(&mut self, key: &str, value: String) {
        let key_lowercase = key.to_lowercase();
        let full_key = if !key_lowercase.starts_with("tf_var_") {
            format!("TF_VAR_{}", key_lowercase)
        } else {
            format!("TF_VAR_{}", key_lowercase.strip_prefix("tf_var_").unwrap())
        };
        self.environment.insert(full_key, value);
    }

    /// Remove Terraform environment variable
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let key_lowercase = key.to_lowercase();
        let full_key = if !key_lowercase.starts_with("tf_var_") {
            format!("TF_VAR_{}", key_lowercase)
        } else {
            format!("TF_VAR_{}", key_lowercase.strip_prefix("tf_var_").unwrap())
        };
        self.environment.remove(&full_key)
    }

    /// Check if environment variable exists
    pub fn contains_key(&self, key: &str) -> bool {
        let key_lowercase = key.to_lowercase();
        let full_key = if !key_lowercase.starts_with("tf_var_") {
            format!("TF_VAR_{}", key_lowercase)
        } else {
            format!("TF_VAR_{}", key_lowercase.strip_prefix("tf_var_").unwrap())
        };
        self.environment.contains_key(&full_key)
    }

    /// Get value of specific environment variable
    pub fn get(&self, key: &str) -> Option<&String> {
        let key_lowercase = key.to_lowercase();
        let full_key = if !key_lowercase.starts_with("tf_var_") {
            format!("TF_VAR_{}", key_lowercase)
        } else {
            format!("TF_VAR_{}", key_lowercase.strip_prefix("tf_var_").unwrap())
        };
        self.environment.get(&full_key)
    }
}

impl Default for TerraformEnv {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terraform_env() {
        env::set_var("TF_VAR_REGION", "us-west-2");
        env::set_var("TF_VAR_INSTANCE_TYPE", "t2.micro");
        env::set_var("NOT_TF_VAR", "should-not-be-included");

        let env = TerraformEnv::new();

        let values = env.values();
        assert!(values.contains(&"us-west-2"));
        assert!(values.contains(&"t2.micro"));
        assert!(!values.contains(&"should-not-be-included"));

        // Check that keys are lowercase
        assert!(env.contains_key("region"));
        assert!(env.contains_key("instance_type"));

        env::remove_var("TF_VAR_REGION");
        env::remove_var("TF_VAR_INSTANCE_TYPE");
        env::remove_var("NOT_TF_VAR");
    }

    #[test]
    fn test_add_and_remove() {
        let mut env = TerraformEnv::new();

        env.add("REGION", "us-east-1".to_string());
        assert_eq!(env.get("region").unwrap(), "us-east-1");
        assert_eq!(env.get("REGION").unwrap(), "us-east-1"); // Should work with any case

        env.add("TF_VAR_INSTANCE", "t3.micro".to_string());
        assert_eq!(env.get("instance").unwrap(), "t3.micro");
        assert_eq!(env.get("INSTANCE").unwrap(), "t3.micro");

        assert_eq!(env.remove("REGION").unwrap(), "us-east-1");
        assert!(env.get("region").is_none());
    }

    #[test]
    fn test_contains_key() {
        let mut env = TerraformEnv::new();
        env.add("TEST_KEY", "test_value".to_string());

        assert!(env.contains_key("test_key"));
        assert!(env.contains_key("TEST_KEY"));
        assert!(env.contains_key("TF_VAR_test_key"));
        assert!(env.contains_key("TF_VAR_TEST_KEY"));
        assert!(!env.contains_key("non_existent_key"));
    }

    #[test]
    fn test_case_insensitive() {
        env::set_var("TF_VAR_PROJECT_NAME", "test-project");
        let env = TerraformEnv::new();
        assert_eq!(env.get("project_name").unwrap(), "test-project");
        assert_eq!(env.get("PROJECT_NAME").unwrap(), "test-project");

        env::remove_var("TF_VAR_PROJECT_NAME");
    }
}

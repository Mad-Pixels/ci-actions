use std::collections::HashMap;
use std::env;

pub struct AwsEnv {
    environment: HashMap<String, String>,
}

impl AwsEnv {
    pub fn new() -> Self {
        let environment: HashMap<String, String> = env::vars()
            .filter(|(key, _)| key.starts_with("AWS_"))
            .map(|(key, value)| {
                let (prefix, name) = key.split_at(7);
                let lowercase_key = format!("{}{}", prefix, name.to_lowercase());
                (lowercase_key, value)
            })
            .collect();

        Self { environment }
    }

    /// Returns all values fo AWS environment variables
    pub fn values(&self) -> Vec<&str> {
        self.environment.values().map(|s| s.as_str()).collect()
    }

    /// Returns all environments variables as HashMap
    pub fn as_map(&self) -> HashMap<String, String> {
        self.environment
            .iter()
            .map(|(key, value)| {
                let clean_key = key.strip_prefix("AWS_").unwrap_or(key).to_string();
                (clean_key, value.clone())
            })
            .collect()
    }

    /// Add new AWS environment variable
    pub fn add(&mut self, key: &str, value: String) {
        let key_lowercase = key.to_lowercase();
        let full_key = if !key_lowercase.starts_with("aws_") {
            format!("AWS_{}", key_lowercase)
        } else {
            format!("AWS_{}", key_lowercase.strip_prefix("aws_").unwrap())
        };
        self.environment.insert(full_key, value);
    }

    /// Remove Terraform environment variable
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let key_lowercase = key.to_lowercase();
        let full_key = if !key_lowercase.starts_with("aws_") {
            format!("AWS_{}", key_lowercase)
        } else {
            format!("AWS_{}", key_lowercase.strip_prefix("aws_").unwrap())
        };
        self.environment.remove(&full_key)
    }

    pub fn contains_key(&self, key: &str) -> bool {
        let key_lowercase = key.to_lowercase();
        let full_key = if !key_lowercase.starts_with("aws_") {
            format!("AWS_{}", key_lowercase)
        } else {
            format!("AWS_{}", key_lowercase.strip_prefix("aws_").unwrap())
        };
        self.environment.contains_key(&full_key)
    }

    /// Get value of specific environment variable
    pub fn get(&self, key: &str) -> Option<&String> {
        let key_lowercase = key.to_lowercase();
        let full_key = if !key_lowercase.starts_with("aws_") {
            format!("AWS_{}", key_lowercase)
        } else {
            format!("AWS_{}", key_lowercase.strip_prefix("aws_").unwrap())
        };
        self.environment.get(&full_key)
    }
}

impl Default for AwsEnv {
    fn default() -> Self {
        Self::new()
    }
}

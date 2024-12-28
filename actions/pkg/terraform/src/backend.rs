use std::collections::HashMap;
use std::env;

pub struct TerraformBackend {
    pub environment: HashMap<String, String>,
}

impl TerraformBackend {
    pub fn new() -> Self {
        Self {
            environment: env::vars()
                .filter(|(key, _)| key.starts_with("BACKEND_"))
                .map(|(key, value)| (key.strip_prefix("BACKEND_").unwrap().to_string(), value))
                .collect(),
        }
    }

    /// Returns all values of Terraform environment variables
    pub fn values(&self) -> Vec<&str> {
        self.environment.values().map(|s| s.as_str()).collect()
    }
}

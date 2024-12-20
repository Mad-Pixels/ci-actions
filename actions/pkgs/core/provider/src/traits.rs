use std::collections::HashMap;
use crate::error::ProviderResult;

/// Trait for cloud provider implementations
pub trait Provider {
    /// Returns all environment variables
    fn get_environment(&self) -> HashMap<String, String>;
    
    /// Returns sensitive environment variables that should be protected
    fn get_sensitive(&self) -> HashMap<String, String>;
    
    /// Returns predefined patterns for masking sensitive data
    fn get_predefined_masked_objects(&self) -> Vec<String> {
        Vec::new()
    }

    /// Validates provider configuration
    fn validate(&self) -> ProviderResult<()>;
}
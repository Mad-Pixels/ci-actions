use crate::error::ProviderResult;
use std::collections::HashMap;

/// Trait for cloud provider implementations.
///
/// The `Provider` trait defines a standard interface for cloud providers,
/// ensuring consistency across different provider implementations.
pub trait Provider {
    fn get_environment(&self) -> HashMap<String, String>;

    fn get_sensitive(&self) -> HashMap<String, String>;

    fn get_predefined_masked_objects(&self) -> Vec<String> {
        Vec::new()
    }

    fn validate(&self) -> ProviderResult<()>;
}

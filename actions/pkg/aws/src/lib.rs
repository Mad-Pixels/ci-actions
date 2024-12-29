pub mod chain;
pub mod command;
pub mod constants;
pub mod environments;
pub mod error;
pub mod executor;

pub use chain::CommandChain;
pub use command::AwsCommand;
pub use constants::{AWS_BIN, CMD};
pub use environments::AwsEnv;
pub use executor::AwsExecutor;

/// Represents the configuration for AWS operations.
///
/// Currently, this struct is a placeholder. You can expand it to include configuration
/// settings as needed.
pub struct AwsConfig {}

impl AwsConfig {
    /// Creates a new `AwsConfig` instance.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AwsConfig {
    fn default() -> Self {
        Self::new()
    }
}

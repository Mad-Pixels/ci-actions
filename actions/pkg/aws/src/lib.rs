pub mod chain;
pub mod command;
pub mod constants;
pub mod environments;
pub mod error;
pub mod executor;

use std::path::PathBuf;

pub use chain::CommandChain;
pub use command::AwsCommand;
use config::ConfigResult;
pub use constants::*;
pub use environments::AwsEnv;
pub use executor::AwsExecutor;

/// Represents the configuration for AWS operations.
pub struct AwsConfig {}

impl AwsConfig {
    /// Creates a new `AwsConfig` instance.
    pub fn new() -> Self {
        Self {}
    }

    /// Gets the AWS CLI executable path.
    pub fn get_bin(&self) -> ConfigResult<PathBuf> {
        AWS_BIN.get()
    }

    /// Gets the AWS command to execute.
    pub fn get_cmd(&self) -> ConfigResult<String> {
        CMD.get()
    }

    /// Gets the S3 destination bucket/path.
    pub fn get_destination(&self) -> ConfigResult<PathBuf> {
        S3_DESTINATION.get()
    }

    /// Gets the exclude patterns as a vector of strings.
    pub fn get_exclude(&self) -> ConfigResult<Option<Vec<String>>> {
        S3_EXCLUDE.get().map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.split(',').map(|s| s.trim().to_string()).collect())
            }
        })
    }

    /// Gets the include patterns as a vector of strings.
    pub fn get_include(&self) -> ConfigResult<Option<Vec<String>>> {
        S3_INCLUDE.get().map(|s| {
            if s.is_empty() {
                None
            } else {
                Some(s.split(',').map(|s| s.trim().to_string()).collect())
            }
        })
    }

    /// Gets the delete flag status.
    pub fn get_delete(&self) -> ConfigResult<bool> {
        S3_DELETE.get()
    }

    /// Gets the dry run flag status.
    pub fn get_dry_run(&self) -> ConfigResult<bool> {
        S3_DRY_RUN.get()
    }

    /// Gets the force flag status.
    pub fn get_force(&self) -> ConfigResult<bool> {
        S3_FORCE.get()
    }
}

impl Default for AwsConfig {
    fn default() -> Self {
        Self::new()
    }
}

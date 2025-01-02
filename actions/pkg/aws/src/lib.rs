pub mod chain;
pub mod command;
pub mod constants;
pub mod environments;
pub mod error;
pub mod executor;

use std::path::PathBuf;

pub use chain::CommandChain;
pub use command::{AwsCommand, LambdaUpdateType};
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
        S3_EXCLUDE
            .get()
            .map(|v| if v.is_empty() { None } else { Some(v) })
    }

    /// Gets the include patterns as a vector of strings.
    pub fn get_include(&self) -> ConfigResult<Option<Vec<String>>> {
        S3_INCLUDE
            .get()
            .map(|v| if v.is_empty() { None } else { Some(v) })
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

    /// Gets the CloudFront distribution ID.
    pub fn get_cloudfront_distribution(&self) -> ConfigResult<String> {
        CLOUDFRONT_DISTRIBUTION.get()
    }

    /// Gets the CloudFront invalidation paths.
    pub fn get_cloudfront_paths(&self) -> ConfigResult<Vec<String>> {
        CLOUDFRONT_PATHS.get()
    }

    /// Gets the Lambda function name.
    pub fn get_lambda_function(&self) -> ConfigResult<String> {
        LAMBDA_FUNCTION.get()
    }

    /// Gets the Lambda ZIP file path.
    pub fn get_lambda_zip(&self) -> ConfigResult<PathBuf> {
        LAMBDA_ZIP.get()
    }

    /// Gets the Lambda container image URI.
    pub fn get_lambda_image(&self) -> ConfigResult<String> {
        LAMBDA_IMAGE.get()
    }

    /// Gets the Lambda publish version flag.
    pub fn get_lambda_publish(&self) -> ConfigResult<bool> {
        LAMBDA_PUBLISH.get()
    }
}

impl Default for AwsConfig {
    fn default() -> Self {
        Self::new()
    }
}

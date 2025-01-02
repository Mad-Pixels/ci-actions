use config::{ConfigValue, FileExists, Required};
use lazy_static::lazy_static;
use std::path::PathBuf;

/// ENV keys
pub const ENV_AWS_CMD: &str = "ACTION_AWS_CMD";
pub const ENV_AWS_BIN: &str = "ACTION_AWS_BIN";

pub const ENV_AWS_CLOUDFRONT_DISTRIBUTION: &str = "ACTION_AWS_CLOUDFRONT_DISTRIBUTION";
pub const ENV_AWS_CLOUDFRONT_PATHS: &str = "ACTION_AWS_CLOUDFRONT_PATHS";

pub const ENV_AWS_LAMBDA_FUNCTION: &str = "ACTION_AWS_LAMBDA_FUNCTION";
pub const ENV_AWS_LAMBDA_ZIP: &str = "ACTION_AWS_LAMBDA_ZIP";
pub const ENV_AWS_LAMBDA_IMAGE: &str = "ACTION_AWS_LAMBDA_IMAGE";
pub const ENV_AWS_LAMBDA_PUBLISH: &str = "ACTION_AWS_LAMBDA_PUBLISH";

pub const ENV_AWS_S3_DESTINATION: &str = "ACTION_AWS_S3_DESTINATION";
pub const ENV_AWS_S3_EXCLUDE: &str = "ACTION_AWS_S3_EXCLUDE";
pub const ENV_AWS_S3_INCLUDE: &str = "ACTION_AWS_S3_INCLUDE";
pub const ENV_AWS_S3_DELETE: &str = "ACTION_AWS_S3_DELETE";
pub const ENV_AWS_S3_DRY_RUN: &str = "ACTION_AWS_S3_DRY_RUN";
pub const ENV_AWS_S3_FORCE: &str = "ACTION_AWS_S3_FORCE";

/// Default values
pub const DEFAULT_AWS_BIN: &str = "/usr/local/bin/aws";
pub const DEFAULT_EMPTY: &str = "";

pub const DEFAULT_CLOUDFRONT_PATHS: [&str; 1] = ["/*"];

lazy_static! {
    /// Configuration value for the AWS command.
    pub static ref CMD: ConfigValue<Required> =
        ConfigValue::<Required>::required(ENV_AWS_CMD);

    /// Configuration value for the AWS CLI executable path.
    pub static ref AWS_BIN: ConfigValue<PathBuf> =
        ConfigValue::new(PathBuf::from(DEFAULT_AWS_BIN), ENV_AWS_BIN)
            .with_validator(FileExists);

    /// Configuration value for S3 destination bucket/path.
    pub static ref S3_DESTINATION: ConfigValue<PathBuf> =
        ConfigValue::new(PathBuf::from(DEFAULT_EMPTY), ENV_AWS_S3_DESTINATION);

    /// Configuration value for exclude patterns.
    pub static ref S3_EXCLUDE: ConfigValue<Vec<String>> =
        ConfigValue::new(Vec::new(), ENV_AWS_S3_EXCLUDE);

    /// Configuration value for include patterns.
    pub static ref S3_INCLUDE: ConfigValue<Vec<String>> =
        ConfigValue::new(Vec::new(), ENV_AWS_S3_INCLUDE);

    /// Configuration value for delete flag.
    pub static ref S3_DELETE: ConfigValue<bool> =
        ConfigValue::new(false, ENV_AWS_S3_DELETE);

    /// Configuration value for dry run flag.
    pub static ref S3_DRY_RUN: ConfigValue<bool> =
        ConfigValue::new(false, ENV_AWS_S3_DRY_RUN);

    /// Configuration value for force flag.
    pub static ref S3_FORCE: ConfigValue<bool> =
        ConfigValue::new(false, ENV_AWS_S3_FORCE);

    /// Configuration value for CloudFront distribution ID
    pub static ref CLOUDFRONT_DISTRIBUTION: ConfigValue<String> =
        ConfigValue::new(DEFAULT_EMPTY.to_string(), ENV_AWS_CLOUDFRONT_DISTRIBUTION);

    /// Configuration value for CloudFront invalidation paths
    pub static ref CLOUDFRONT_PATHS: ConfigValue<Vec<String>> =
    ConfigValue::new(
        DEFAULT_CLOUDFRONT_PATHS.iter().map(|s| s.to_string()).collect(),
        ENV_AWS_CLOUDFRONT_PATHS
    );

    /// Configuration value for Lambda function name
    pub static ref LAMBDA_FUNCTION: ConfigValue<String> =
        ConfigValue::new(DEFAULT_EMPTY.to_string(), ENV_AWS_LAMBDA_FUNCTION);

    /// Configuration value for Lambda ZIP file path
    pub static ref LAMBDA_ZIP: ConfigValue<PathBuf> =
        ConfigValue::new(PathBuf::from(DEFAULT_EMPTY), ENV_AWS_LAMBDA_ZIP);

    /// Configuration value for Lambda container image URI
    pub static ref LAMBDA_IMAGE: ConfigValue<String> =
        ConfigValue::new(DEFAULT_EMPTY.to_string(), ENV_AWS_LAMBDA_IMAGE);

    /// Configuration value for Lambda publish version flag
    pub static ref LAMBDA_PUBLISH: ConfigValue<bool> =
        ConfigValue::new(false, ENV_AWS_LAMBDA_PUBLISH);
}

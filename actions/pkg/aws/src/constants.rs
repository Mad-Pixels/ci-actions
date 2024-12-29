use config::{ConfigValue, FileExists, Required};
use lazy_static::lazy_static;
use std::path::PathBuf;

/// Environment variable for specifying the AWS command.
pub const ENV_AWS_CMD: &str = "ACTION_AWS_CMD";

/// Environment variable for specifying the path to the AWS CLI executable.
pub const ENV_AWS_BIN: &str = "ACTION_AWS_BIN";

/// Default path to the AWS CLI executable.
pub const DEFAULT_AWS_BIN: &str = "/usr/local/bin/aws";

lazy_static! {
    /// Configuration value for the AWS command.
    pub static ref CMD: ConfigValue<Required> =
        ConfigValue::<Required>::required(ENV_AWS_CMD);

    /// Configuration value for the AWS CLI executable path.
    pub static ref AWS_BIN: ConfigValue<PathBuf> =
        ConfigValue::new(PathBuf::from(DEFAULT_AWS_BIN), ENV_AWS_BIN)
            .with_validator(FileExists);
}

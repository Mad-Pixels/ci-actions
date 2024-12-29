use config::{ConfigValue, FileExists, Required};
use lazy_static::lazy_static;
use std::path::PathBuf;

pub const ENV_AWS_CMD: &str = "ACTION_AWS_CMD";
pub const ENV_AWS_BIN: &str = "ACTION_AWS_BIN";

pub const DEFAULT_AWS_BIN: &str = "/usr/local/bin/aws";

lazy_static! {
    pub static ref CMD: ConfigValue<Required> = 
        ConfigValue::<Required>::required(ENV_AWS_CMD);
    pub static ref AWS_BIN: ConfigValue<PathBuf> =
        ConfigValue::new(PathBuf::from(DEFAULT_AWS_BIN), ENV_AWS_BIN)
            .with_validator(FileExists);
}
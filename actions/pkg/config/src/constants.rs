use crate::value::ConfigValue;
use crate::error::Required;

use lazy_static::lazy_static;
use std::path::PathBuf;

// ENV keys
pub const ENV_CMD: &str = "ACTION_CMD";
pub const ENV_TERRAFORM_BIN: &str = "ACTION_TERRAFORM_PATH";
pub const ENV_WORKING_DIR: &str = "ACTION_WORKING_DIR";

// Default values
pub const DEFAULT_TERRAFORM_BIN: &str = "/usr/local/bin/terraform";
pub const DEFAULT_WORKING_DIR: &str = ".";

lazy_static! {
    // Required values
    pub static ref CMD: ConfigValue<Required> = ConfigValue::<Required>::required(ENV_CMD);

    // Optional values with defaults
    pub static ref TERRAFORM_BIN: ConfigValue<PathBuf> = ConfigValue::new(
        PathBuf::from(DEFAULT_TERRAFORM_BIN),
        ENV_TERRAFORM_BIN
    );

    pub static ref WORKING_DIR: ConfigValue<PathBuf> = ConfigValue::new(
        PathBuf::from(DEFAULT_WORKING_DIR),
        ENV_WORKING_DIR
    );
}
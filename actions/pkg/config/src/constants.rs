use crate::value::ConfigValue;
use crate::error::Required;

use lazy_static::lazy_static;
use std::path::PathBuf;

// ENV keys: general
pub const ENV_CMD: &str = "ACTION_CMD";
pub const ENV_MASK: &str = "ACTION_MASK";
pub const ENV_LOG_LEVEL: &str = "ACTION_LOG_LEVEL";
pub const ENV_WORKING_DIR: &str = "ACTION_WORKING_DIR";

// ENV keys: terraform 
pub const ENV_TERRAFORM_BIN: &str = "ACTION_TERRAFORM_PATH";
pub const ENV_TERRAFORM_OUUTPUT: &str = "ACTION_TERRAFORM_OUTPUT";
pub const ENV_TERRAFORM_WORKSPACE: &str = "ACTION_TERRAFORM_WORKSPACE";

// Default values
pub const DEFAULT_TERRAFORM_OUTPUT: &str = "./tf_output_file";
pub const DEFAULT_TERRAFORM_BIN: &str = "/usr/local/bin/terraform";
pub const DEFAULT_LOG_LEVEL: &str = "info";
pub const DEFAULT_WORKING_DIR: &str = ".";
pub const DEFAULT_MASK: &str = "*****";
pub const DEFAULT_EMPTY: &str = "";

lazy_static! {
    // Required values
    pub static ref CMD: ConfigValue<Required> = ConfigValue::<Required>::required(ENV_CMD);

    // Optional values with defaults
    pub static ref TERRAFORM_WORKSPACE: ConfigValue<String> = ConfigValue::new(
        DEFAULT_EMPTY.to_string(),
        ENV_TERRAFORM_WORKSPACE
    );
    pub static ref TERRAFORM_OUTPUT: ConfigValue<PathBuf> = ConfigValue::new(
        PathBuf::from(DEFAULT_TERRAFORM_OUTPUT), 
        ENV_TERRAFORM_OUUTPUT
    );

    pub static ref TERRAFORM_BIN: ConfigValue<PathBuf> = ConfigValue::new(
        PathBuf::from(DEFAULT_TERRAFORM_BIN),
        ENV_TERRAFORM_BIN
    );

    pub static ref WORKING_DIR: ConfigValue<PathBuf> = ConfigValue::new(
        PathBuf::from(DEFAULT_WORKING_DIR),
        ENV_WORKING_DIR
    );

    pub static ref LOG_LEVEL: ConfigValue<String> = ConfigValue::new(
        DEFAULT_LOG_LEVEL.to_string(),
        ENV_LOG_LEVEL 
    );

    pub static ref MASK: ConfigValue<String> = ConfigValue::new(
        DEFAULT_MASK.to_string(), 
        ENV_MASK
    );
}
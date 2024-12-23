use config::{Required, ConfigValue, FileExists};

use lazy_static::lazy_static;
use std::path::PathBuf;

// ENV keys
pub const ENV_TERRAFORM_CMD: &str = "ACTION_TERRAFORM_CMD";
pub const ENV_TERRAFORM_BIN: &str = "ACTION_TERRAFORM_BIN";
pub const ENV_TERRAFORM_OUTPUT: &str = "ACTION_TERRAFORM_OUTPUT";
pub const ENV_TERRAFORM_WORKSPACE: &str = "ACTION_TERRAFORM_WORKSPACE";

// Default values
pub const DEFAULT_TERRAFORM_OUTPUT: &str = "./tf_output_file";
pub const DEFAULT_TERRAFORM_BIN: &str = "/usr/local/bin/terraform";
pub const DEFAULT_EMPTY: &str = "";

lazy_static! {
    pub static ref CMD: ConfigValue<Required> = ConfigValue::<Required>::required(ENV_TERRAFORM_CMD);

    pub static ref TERRAFORM_WORKSPACE: ConfigValue<String> = ConfigValue::new(
        DEFAULT_EMPTY.to_string(),
        ENV_TERRAFORM_WORKSPACE
    );

    pub static ref TERRAFORM_OUTPUT: ConfigValue<PathBuf> = ConfigValue::new(
        PathBuf::from(DEFAULT_TERRAFORM_OUTPUT),
        ENV_TERRAFORM_OUTPUT
    );

    pub static ref TERRAFORM_BIN: ConfigValue<PathBuf> = ConfigValue::new(
        PathBuf::from(DEFAULT_TERRAFORM_BIN),
        ENV_TERRAFORM_BIN
    ).with_validator(FileExists);
}
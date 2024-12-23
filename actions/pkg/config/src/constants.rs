use crate::validator::DirExists;
use crate::value::ConfigValue;

use lazy_static::lazy_static;
use std::path::PathBuf;
 
// ENV keys
pub const ENV_MASK: &str = "ACTION_MASK";
pub const ENV_LOG_LEVEL: &str = "ACTION_LOG_LEVEL";
pub const ENV_WORKING_DIR: &str = "ACTION_WORKING_DIR";

// Default values
pub const DEFAULT_LOG_LEVEL: &str = "info";
pub const DEFAULT_WORKING_DIR: &str = ".";
pub const DEFAULT_MASK: &str = "*****";

lazy_static! {
    pub static ref WORKING_DIR: ConfigValue<PathBuf> = ConfigValue::new(
        PathBuf::from(DEFAULT_WORKING_DIR),
        ENV_WORKING_DIR
    ).with_validator(DirExists);

    pub static ref LOG_LEVEL: ConfigValue<String> = ConfigValue::new(
        DEFAULT_LOG_LEVEL.to_string(),
        ENV_LOG_LEVEL
    );

    pub static ref MASK: ConfigValue<String> = ConfigValue::new(
        DEFAULT_MASK.to_string(),
        ENV_MASK
    );
}

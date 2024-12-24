mod constants;
mod error;
mod validator;
mod value;

pub use constants::*;
pub use error::{ConfigError, ConfigResult, Required};
pub use validator::{DirExists, FileExists};
pub use value::ConfigValue;

use std::path::PathBuf;
pub struct MainConfig {}

impl MainConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_working_dir(&self) -> ConfigResult<PathBuf> {
        WORKING_DIR.get()
    }

    pub fn get_log_level(&self) -> ConfigResult<String> {
        LOG_LEVEL.get()
    }

    pub fn get_mask(&self) -> ConfigResult<String> {
        MASK.get()
    }
}

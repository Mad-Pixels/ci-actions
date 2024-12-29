pub mod command;
pub mod constants;
pub mod error;
pub mod executor;

pub use command::AWSCommand;
pub use constants::*;
pub use executor::AWSExecutor;

use config::ConfigResult;
use std::path::PathBuf;

pub struct AWSConfig {}

impl AWSConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_bin(&self) -> ConfigResult<PathBuf> {
        AWS_BIN.get()
    }

    pub fn get_cmd(&self) -> ConfigResult<String> {
        CMD.get()
    }
}

impl Default for AWSConfig {
    fn default() -> Self {
        Self::new()
    }
}
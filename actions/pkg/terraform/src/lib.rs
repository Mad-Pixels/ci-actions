pub mod constants;
pub mod command;
pub mod error;

pub mod executor;
pub mod environments;
pub use environments::TerraformEnv;
pub use constants::*;

use config::ConfigResult;
use std::path::PathBuf;

pub struct TerraformConfig {}

impl TerraformConfig {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_output_file(&self) -> ConfigResult<PathBuf> {
        TERRAFORM_OUTPUT.get()
    }

    pub fn get_bin(&self) -> ConfigResult<PathBuf> {
        TERRAFORM_BIN.get()
    }

    pub fn get_cmd(&self) -> ConfigResult<String> {
        CMD.get()
    }
}

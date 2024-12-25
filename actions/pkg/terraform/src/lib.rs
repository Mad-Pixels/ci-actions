pub mod chain;
pub mod command;
pub mod constants;
pub mod error;

pub mod environments;
pub mod executor;
pub use constants::*;
pub use environments::TerraformEnv;

pub use chain::CommandChain;
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

    pub fn get_workspace(&self) -> ConfigResult<String> {
        TERRAFORM_WORKSPACE.get()
    }

    pub fn get_cmd(&self) -> ConfigResult<String> {
        CMD.get()
    }
}

impl Default for TerraformConfig {
    fn default() -> Self {
        Self::new()
    }
}

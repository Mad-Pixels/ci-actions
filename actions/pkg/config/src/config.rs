use crate::constants::*;
use crate::error::ConfigResult;

use std::path::PathBuf;

pub struct Config {}

impl Config {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_cmd(&self) -> ConfigResult<String> {
        CMD.get()
    }

    pub fn get_terraform_output(&self) -> ConfigResult<PathBuf> {
        TERRAFORM_OUTPUT.get()
    }

    pub fn get_terraform_bin(&self) -> ConfigResult<PathBuf> {
        TERRAFORM_BIN.get()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn cleanup_env() {
        env::remove_var("ACTION_TERRAFORM_PATH");
        env::remove_var("ACTION_CMD");
        env::remove_var("ACTION_WORKING_DIR");
    }

    #[test]
    fn test_get_terraform_bin_default() {
        cleanup_env();
        let config = Config::new();
        let result = config.get_terraform_bin().unwrap();
        assert_eq!(result, PathBuf::from("/usr/local/bin/terraform"));
    }

    #[test]
    fn test_get_terraform_bin_from_env() {
        cleanup_env();
        env::set_var("ACTION_TERRAFORM_PATH", "/custom/path/terraform");
        let config = Config::new();
        let result = config.get_terraform_bin().unwrap();
        assert_eq!(result, PathBuf::from("/custom/path/terraform"));
    }

    #[test]
    fn test_cmd_required() {
        cleanup_env();
        let config = Config::new();
        assert!(config.get_cmd().is_err());

        env::set_var("ACTION_CMD", "plan");
        assert_eq!(config.get_cmd().unwrap(), "plan");
    }
}

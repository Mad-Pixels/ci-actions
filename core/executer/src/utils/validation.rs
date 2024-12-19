use std::collections::HashMap;
use std::path::PathBuf;
use crate::ExecuterError;

pub fn validate_command(cmd: &[String]) -> Result<(), ExecuterError> {
    if cmd.is_empty() {
        return Err(ExecuterError::ValidationError("Empty command sequence".to_string()));
    }

    for arg in cmd {
        if arg.contains(['&', '|', ';', '`', '\\']) {
            return Err(ExecuterError::ValidationError(
                format!("Invalid command argument: {}", arg)
            ));
        }
    }
    Ok(())
}

pub fn validate_env(env: &HashMap<String, String>) -> Result<(), ExecuterError> {
    for (k, v) in env {
        if k.trim().is_empty() || v.trim().is_empty() {
            return Err(ExecuterError::ValidationError(
                "Environment variables cannot be empty".to_string()
            ));
        }
    }
    Ok(())
}

pub fn validate_cwd(cwd: &Option<PathBuf>) -> Result<(), ExecuterError> {
    if let Some(path) = cwd {
        if !path.exists() {
            return Err(ExecuterError::ValidationError(
                format!("Working directory does not exist: {}", path.display())
            ));
        }
        if !path.is_dir() {
            return Err(ExecuterError::ValidationError(
                format!("Path is not a directory: {}", path.display())
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_command() {
        assert!(validate_command(&["ls".to_string(), "-l".to_string()]).is_ok());
        assert!(validate_command(&[]).is_err());
        assert!(validate_command(&["ls".to_string(), "&".to_string()]).is_err());
    }

    #[test]
    fn test_validate_env() {
        let mut env = HashMap::new();
        env.insert("KEY".to_string(), "value".to_string());
        assert!(validate_env(&env).is_ok());

        env.insert("EMPTY".to_string(), "".to_string());
        assert!(validate_env(&env).is_err());
    }

    #[test]
    fn test_validate_cwd() {
        assert!(validate_cwd(&None).is_ok());
        assert!(validate_cwd(&Some(PathBuf::from("."))).is_ok());
        assert!(validate_cwd(&Some(PathBuf::from("/nonexistent"))).is_err());
    }
}
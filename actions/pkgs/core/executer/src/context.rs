use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Context {
    pub command: Vec<String>,
    pub env: HashMap<String, String>,
    pub cwd: Option<PathBuf>,
    pub timeout: Option<u64>,
}

impl Context {
    pub fn new(command: Vec<String>, env: HashMap<String, String>, cwd: Option<PathBuf>) -> Self {
        Self {
            command,
            env,
            cwd,
            timeout: None,
        }
    }

    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

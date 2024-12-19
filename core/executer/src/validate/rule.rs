#[derive(Debug, Clone)]
pub struct ValidationContext {
    pub env: std::collections::HashMap<String, String>,
    pub cwd: Option<std::path::PathBuf>,
    pub command: Vec<String>,
}

impl ValidationContext {
    pub fn new(
        command: Vec<String>,
        env: std::collections::HashMap<String, String>,
        cwd: Option<std::path::PathBuf>,
    ) -> Self {
        Self { command, env, cwd }
    }
}

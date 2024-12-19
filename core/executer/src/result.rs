#[derive(Debug)]
pub struct ExecutionResult {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
    pub masked_stdout: Option<String>,
    pub masked_stderr: Option<String>,
}

impl ExecutionResult {
    pub fn new(
        status: i32,
        stdout: String,
        stderr: String,
        masked_stdout: Option<String>,
        masked_stderr: Option<String>,
    ) -> Result<Self, String> {
        if status < 0 {
            return Err("Status code cannot be negative".to_string());
        }
        Ok(Self {
            status,
            stdout,
            stderr,
            masked_stdout,
            masked_stderr,
        })
    }
}
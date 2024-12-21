use std::collections::HashMap;
use std::path::PathBuf;

/// Represents the context in which a command is executed.
///
/// The `Context` struct holds information about the command to be executed,
/// its environment variables, the current working directory, and an optional
/// timeout for the command execution.
#[derive(Debug, Clone)]
pub struct Context {
    /// The command and its arguments to be executed.
    pub command: Vec<String>,

    /// Environment variables for the command execution.
    pub env: HashMap<String, String>,

    /// The current working directory for the command execution.
    /// If `None`, the command inherits the working directory of the parent process.
    pub cwd: Option<PathBuf>,

    /// An optional timeout (in seconds) for the command execution.
    /// If set, the command will be killed if it does not complete within the specified duration.
    pub timeout: Option<u64>,
}

impl Context {
    /// Creates a new `Context` instance.
    ///
    /// # Arguments
    ///
    /// * `command` - A vector of strings representing the command and its arguments.
    /// * `env` - A `HashMap` containing environment variables for the command execution.
    /// * `cwd` - An optional `PathBuf` specifying the current working directory for the command.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::Context;
    /// use std::collections::HashMap;
    /// use std::path::PathBuf;
    ///
    /// let command = vec!["echo".to_string(), "Hello, World!".to_string()];
    /// let env = HashMap::new();
    /// let cwd = Some(PathBuf::from("/tmp"));
    ///
    /// let context = Context::new(command, env, cwd);
    /// ```
    pub fn new(command: Vec<String>, env: HashMap<String, String>, cwd: Option<PathBuf>) -> Self {
        Self {
            command,
            env,
            cwd,
            timeout: None,
        }
    }

    /// Sets a timeout for the command execution.
    ///
    /// This method allows you to specify a timeout duration (in seconds) after which
    /// the command will be terminated if it has not completed.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout duration in seconds.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::Context;
    /// use std::collections::HashMap;
    ///
    /// let command = vec!["sleep".to_string(), "10".to_string()];
    /// let env = HashMap::new();
    ///
    /// let context = Context::new(command, env, None).with_timeout(5);
    /// ```
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = Some(timeout);
        self
    }
}

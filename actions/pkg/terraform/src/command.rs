/// Defines the operations that can be performed on Terraform workspaces.
#[derive(Debug, Clone)]
pub enum WorkspaceOperation {
    /// List all available workspaces.
    List,

    /// Create a new workspace with the given name.
    New(String),

    /// Select an existing workspace by name.
    Select(String),

    /// Delete a workspace by name.
    Delete(String),
}

/// Represents the various Terraform commands that can be executed.
#[derive(Debug, Clone)]
pub enum TerraformCommand {
    /// Initialize a Terraform working directory.
    ///
    /// # Fields
    ///
    /// - `dir`: The directory where Terraform is initialized.
    /// - `backend_config`: Optional backend configuration parameters.
    Init {
        dir: std::path::PathBuf,
        backend_config: Option<std::collections::HashMap<String, String>>,
    },

    /// Create an execution plan.
    ///
    /// # Fields
    ///
    /// - `dir`: The directory where the plan is created.
    /// - `vars`: Variables to pass to the Terraform configuration.
    /// - `out`: Optional path to save the generated plan.
    Plan {
        dir: std::path::PathBuf,
        vars: std::collections::HashMap<String, String>,
        out: Option<std::path::PathBuf>,
    },

    /// Apply the changes required to reach the desired state of the configuration.
    ///
    /// # Fields
    ///
    /// - `dir`: The directory where the apply is executed.
    /// - `plan_file`: Optional path to a plan file.
    /// - `auto_approve`: Automatically approve the plan without prompting.
    Apply {
        dir: std::path::PathBuf,
        plan_file: Option<std::path::PathBuf>,
        auto_approve: bool,
    },

    /// Manage Terraform workspaces.
    ///
    /// # Fields
    ///
    /// - `dir`: The directory where workspace operations are performed.
    /// - `operation`: The workspace operation to execute.
    Workspace {
        dir: std::path::PathBuf,
        operation: WorkspaceOperation,
    },
}

impl TerraformCommand {
    /// Converts the `TerraformCommand` into a list of command-line arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::path::PathBuf;
    /// use std::collections::HashMap;
    /// use terraform::command::TerraformCommand;
    ///
    /// let init_command = TerraformCommand::Init {
    ///     dir: PathBuf::from("/path/to/dir"),
    ///     backend_config: Some(HashMap::from([
    ///         ("key1".to_string(), "value1".to_string()),
    ///         ("key2".to_string(), "value2".to_string()),
    ///     ])),
    /// };
    ///
    /// let args = init_command.to_args();
    /// assert_eq!(
    ///     args,
    ///     vec![
    ///         "init".to_string(),
    ///         "-backend-config=key1=value1".to_string(),
    ///         "-backend-config=key2=value2".to_string()
    ///     ]
    /// );
    /// ```
    pub fn to_args(&self) -> Vec<String> {
        match self {
            Self::Init {
                dir: _,
                backend_config,
            } => {
                let mut args = vec!["init".to_string(), "-reconfigure".to_string()];
                if let Some(config) = backend_config {
                    let mut keys: Vec<_> = config.keys().collect();
                    keys.sort();

                    for key in keys {
                        if let Some(value) = config.get(key) {
                            args.push(format!("-backend-config={}={}", key, value));
                        }
                    }
                }
                args
            }
            Self::Plan { dir: _, vars, out } => {
                let mut args = vec!["plan".to_string()];

                let mut var_keys: Vec<_> = vars.keys().collect();
                var_keys.sort();

                for key in var_keys {
                    if let Some(value) = vars.get(key) {
                        args.push(format!("-var={}={}", key, value));
                    }
                }
                if let Some(out_file) = out {
                    args.push("-out".to_string());
                    args.push(out_file.to_string_lossy().to_string());
                }
                args
            }
            Self::Apply {
                dir: _,
                plan_file,
                auto_approve,
            } => {
                let mut args = vec!["apply".to_string()];
                if *auto_approve {
                    args.push("-auto-approve".to_string());
                }
                if let Some(file) = plan_file {
                    args.push(file.to_string_lossy().to_string());
                }
                args
            }
            Self::Workspace { dir: _, operation } => {
                let mut args = vec!["workspace".to_string()];
                match operation {
                    WorkspaceOperation::List => args.push("list".to_string()),
                    WorkspaceOperation::New(name) => {
                        args.push("new".to_string());
                        args.push(name.clone());
                    }
                    WorkspaceOperation::Select(name) => {
                        args.push("select".to_string());
                        args.push(name.clone());
                    }
                    WorkspaceOperation::Delete(name) => {
                        args.push("delete".to_string());
                        args.push(name.clone());
                    }
                }
                args
            }
        }
    }
}

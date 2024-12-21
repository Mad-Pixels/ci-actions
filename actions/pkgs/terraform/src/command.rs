#[derive(Debug, Clone)]
pub enum WorkspaceOperation {
    List,
    New(String),
    Select(String),
    Delete(String),
}

#[derive(Debug, Clone)]
pub enum TerraformCommand {
    Init {
        dir: std::path::PathBuf,
        backend_config: Option<std::collections::HashMap<String, String>>,
    },
    Plan {
        dir: std::path::PathBuf,
        vars: std::collections::HashMap<String, String>,
        out: Option<std::path::PathBuf>,
    },
    Apply {
        dir: std::path::PathBuf,
        plan_file: Option<std::path::PathBuf>,
        auto_approve: bool,
    },
    Workspace {
        dir: std::path::PathBuf,
        operation: WorkspaceOperation,
    },
}

impl TerraformCommand {
    pub fn to_args(&self) -> Vec<String> {
        match self {
            Self::Init {
                dir: _,
                backend_config,
            } => {
                let mut args = vec!["init".to_string()];
                if let Some(config) = backend_config {
                    for (key, value) in config {
                        args.push(format!("-backend-config={}={}", key, value));
                    }
                }
                args
            }
            Self::Plan { dir: _, vars, out } => {
                let mut args = vec!["plan".to_string()];
                for (key, value) in vars {
                    args.push(format!("-var={}={}", key, value));
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

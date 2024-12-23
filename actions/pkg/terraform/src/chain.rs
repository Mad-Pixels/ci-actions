use crate::command::{TerraformCommand, WorkspaceOperation};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CommandChain {
    dir: PathBuf,
    vars: HashMap<String, String>,
    backend_config: Option<HashMap<String, String>>,
    workspace: Option<String>,
    out: Option<PathBuf>,
    auto_approve: bool,
}

impl CommandChain {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            vars: HashMap::new(),
            backend_config: None,
            workspace: None,
            out: None,
            auto_approve: false,
        }
    }

    pub fn with_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.vars = vars;
        self
    }

    pub fn with_backend_config(mut self, config: HashMap<String, String>) -> Self {
        self.backend_config = Some(config);
        self
    }

    pub fn with_workspace(mut self, workspace: Option<String>) -> Self {
        self.workspace = workspace;
        self
    }

    pub fn with_out(mut self, out: Option<PathBuf>) -> Self {
        self.out = out;
        self
    }

    pub fn with_auto_approve(mut self, auto_approve: bool) -> Self {
        self.auto_approve = auto_approve;
        self
    }

    fn build_init(&self) -> TerraformCommand {
        TerraformCommand::Init {
            dir: self.dir.clone(),
            backend_config: self.backend_config.clone(),
        }
    }

    fn build_workspace(&self) -> Option<Vec<TerraformCommand>> {
        self.workspace.as_ref().map(|ws| {
            vec![
                TerraformCommand::Workspace {
                    dir: self.dir.clone(),
                    operation: WorkspaceOperation::New(ws.clone()),
                },
                TerraformCommand::Workspace {
                    dir: self.dir.clone(),
                    operation: WorkspaceOperation::Select(ws.clone()),
                },
            ]
        })
    }

    fn build_plan(&self) -> TerraformCommand {
        TerraformCommand::Plan {
            dir: self.dir.clone(),
            vars: self.vars.clone(),
            out: self.out.clone(),
        }
    }

    fn build_apply(&self) -> TerraformCommand {
        TerraformCommand::Apply {
            dir: self.dir.clone(),
            plan_file: self.out.clone(),
            auto_approve: self.auto_approve,
        }
    }

    pub fn plan_chain(&self) -> Vec<TerraformCommand> {
        let mut commands = vec![self.build_init()];

        if let Some(workspace_cmds) = self.build_workspace() {
            commands.extend(workspace_cmds);
        }

        commands.push(self.build_plan());
        commands
    }

    pub fn apply_chain(&self) -> Vec<TerraformCommand> {
        let mut commands = vec![self.build_init()];

        if let Some(workspace_cmds) = self.build_workspace() {
            commands.extend(workspace_cmds);
        }

        commands.push(self.build_apply());
        commands
    }
}
use crate::command::{TerraformCommand, WorkspaceOperation};
use crate::error::{TerraformError, TerraformResult};
use executer::{Context, Output, Subprocess, Target, Validator};
use processor::Collection;
use std::path::PathBuf;

pub struct TerraformExecutor {
    subprocess: Subprocess,
    terraform_path: PathBuf,
}

impl TerraformExecutor {
    pub fn new(processor: Collection, terraform_path: PathBuf) -> Self {
        let output = Output::new(processor, Target::Stdout, Target::Stderr);

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        Self {
            subprocess,
            terraform_path,
        }
    }

    pub async fn execute(&self, command: TerraformCommand) -> TerraformResult<i32> {
        let mut args = command.to_args();
        let working_dir = match &command {
            TerraformCommand::Init { dir, .. } => dir,
            TerraformCommand::Plan { dir, .. } => dir,
            TerraformCommand::Apply { dir, .. } => dir,
            TerraformCommand::Workspace { dir, .. } => dir,
        };

        let mut cmd = vec![self.terraform_path.to_string_lossy().to_string()];
        cmd.extend(args);

        let context = Context::new(
            cmd,
            std::collections::HashMap::new(),
            Some(working_dir.clone()),
        );

        self.subprocess
            .execute(context)
            .await
            .map_err(TerraformError::from)
    }

    pub async fn init(
        &self,
        dir: PathBuf,
        backend_config: Option<std::collections::HashMap<String, String>>,
    ) -> TerraformResult<i32> {
        self.execute(TerraformCommand::Init {
            dir,
            backend_config,
        })
        .await
    }

    pub async fn plan(
        &self,
        dir: PathBuf,
        vars: std::collections::HashMap<String, String>,
        out: Option<PathBuf>,
    ) -> TerraformResult<i32> {
        self.execute(TerraformCommand::Plan { dir, vars, out })
            .await
    }

    pub async fn apply(
        &self,
        dir: PathBuf,
        plan_file: Option<PathBuf>,
        auto_approve: bool,
    ) -> TerraformResult<i32> {
        self.execute(TerraformCommand::Apply {
            dir,
            plan_file,
            auto_approve,
        })
        .await
    }

    pub async fn workspace(
        &self,
        dir: PathBuf,
        operation: WorkspaceOperation,
    ) -> TerraformResult<i32> {
        self.execute(TerraformCommand::Workspace { dir, operation })
            .await
    }
}

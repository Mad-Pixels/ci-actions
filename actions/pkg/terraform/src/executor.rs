use crate::chain::CommandChain;
use crate::command::{TerraformCommand, WorkspaceOperation};
use crate::error::{TerraformError, TerraformResult};

use executer::{Context, Output, Subprocess, Target, Validator};
use processor::ProcessorCollection;
use std::collections::HashMap;
use std::path::PathBuf;

/// Executor responsible for running Terraform commands.
pub struct TerraformExecutor {
    subprocess: Subprocess,
    terraform_path: PathBuf,
}

impl TerraformExecutor {
    /// Creates a new instance of `TerraformExecutor`.
    ///
    /// # Arguments
    ///
    /// * `processor` - A collection of maskers for processing output.
    /// * `terraform_path` - The path to the Terraform executable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use terraform::executor::TerraformExecutor;
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    /// use std::path::PathBuf;
    ///
    /// let env = HashMap::new();
    /// let provider = AWSProvider::new(env.clone());
    ///
    /// let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    /// let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///  
    /// let processor = ProcessorCollection::new(processors);
    /// let terraform_path = PathBuf::from("/usr/local/bin/terraform");
    /// let executor = TerraformExecutor::new(processor, terraform_path);
    /// ```
    pub fn new(processor: ProcessorCollection, terraform_path: PathBuf) -> Self {
        let output = Output::new(processor, Target::Stdout, Target::Stderr);

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        Self {
            subprocess,
            terraform_path,
        }
    }

    /// Executes a given Terraform command asynchronously.
    ///
    /// # Arguments
    ///
    /// * `command` - The `TerraformCommand` to execute.
    ///
    /// # Returns
    ///
    /// * `TerraformResult<i32>` - The result of the command execution containing the exit code.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::PathBuf;
    /// use terraform::error::TerraformError;
    /// use terraform::executor::TerraformExecutor;
    ///
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), TerraformError> {
    ///     let env = HashMap::new();
    ///     let provider = AWSProvider::new(env.clone());
    ///     
    ///     let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    ///     let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    ///     let processor = ProcessorCollection::new(processors);
    ///     let terraform_path = PathBuf::from("/usr/local/bin/terraform");
    ///     let executor = TerraformExecutor::new(processor, terraform_path);
    ///     
    ///     let backend_config = HashMap::from([("key".to_string(), "value".to_string())]);
    ///     executor.init(PathBuf::from("/path/to/dir"), Some(backend_config)).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute(&self, command: TerraformCommand) -> TerraformResult<i32> {
        let args = command.to_args();
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

    /// Initializes a Terraform working directory.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory to initialize.
    /// * `backend_config` - Optional backend configuration parameters.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::PathBuf;
    /// use terraform::error::TerraformError;
    /// use terraform::executor::TerraformExecutor;
    ///
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), TerraformError> {
    ///     let env = HashMap::new();
    ///     let provider = AWSProvider::new(env.clone());
    ///     
    ///     let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    ///     let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    ///     let processor = ProcessorCollection::new(processors);
    ///     let terraform_path = PathBuf::from("/usr/local/bin/terraform");
    ///     let executor = TerraformExecutor::new(processor, terraform_path);
    ///     
    ///     let backend_config = HashMap::from([("key".to_string(), "value".to_string())]);
    ///     executor.init(PathBuf::from("/path/to/dir"), Some(backend_config)).await?;
    ///     Ok(())
    /// }
    /// ```
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

    /// Creates an execution plan.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory where the plan is created.
    /// * `vars` - Variables to pass to the Terraform configuration.
    /// * `out` - Optional path to save the generated plan.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::path::PathBuf;
    /// use terraform::error::TerraformError;
    /// use terraform::executor::TerraformExecutor;
    ///
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), TerraformError> {
    ///     let env = HashMap::new();
    ///     let provider = AWSProvider::new(env.clone());
    ///     
    ///     let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    ///     let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    ///     let processor = ProcessorCollection::new(processors);
    ///     let terraform_path = PathBuf::from("/usr/local/bin/terraform");
    ///     let executor = TerraformExecutor::new(processor, terraform_path);
    ///     
    ///     let vars = HashMap::from([("instance_type".to_string(), "t2.micro".to_string())]);
    ///     let plan_path = Some(PathBuf::from("/path/to/plan.out"));
    ///     executor.plan(PathBuf::from("/path/to/dir"), vars, plan_path).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn plan(
        &self,
        dir: PathBuf,
        vars: std::collections::HashMap<String, String>,
        out: Option<PathBuf>,
    ) -> TerraformResult<i32> {
        self.execute(TerraformCommand::Plan { dir, vars, out })
            .await
    }

    /// Applies the changes required to reach the desired state.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory where the apply is executed.
    /// * `plan_file` - Optional path to a pre-generated plan file.
    /// * `auto_approve` - Automatically approve the plan without prompting.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use terraform::executor::TerraformExecutor;
    /// use terraform::error::TerraformError;
    /// use std::path::PathBuf;
    ///
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), TerraformError> {
    ///     let env = HashMap::new();
    ///     let provider = AWSProvider::new(env.clone());
    ///     
    ///     let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    ///     let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    ///     let processor = ProcessorCollection::new(processors);
    ///     let terraform_path = PathBuf::from("/usr/local/bin/terraform");
    ///     let executor = TerraformExecutor::new(processor, terraform_path);
    ///     
    ///     executor.apply(
    ///         PathBuf::from("/path/to/dir"),
    ///         Some(PathBuf::from("/path/to/plan.out")),
    ///         true,
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
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

    /// Manages Terraform workspaces.
    ///
    /// # Arguments
    ///
    /// * `dir` - The directory where workspace operations are performed.
    /// * `operation` - The workspace operation to execute.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use terraform::executor::TerraformExecutor;
    /// use terraform::command::WorkspaceOperation;
    /// use terraform::error::TerraformError;
    /// use std::path::PathBuf;
    ///
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), TerraformError> {
    ///     let env = HashMap::new();
    ///     let provider = AWSProvider::new(env.clone());
    ///     
    ///     let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    ///     let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    ///     let processor = ProcessorCollection::new(processors);
    ///     let terraform_path = PathBuf::from("/usr/local/bin/terraform");
    ///     let executor = TerraformExecutor::new(processor, terraform_path);
    ///     
    ///     executor.workspace(
    ///         PathBuf::from("/path/to/dir"),
    ///         WorkspaceOperation::List,
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn workspace(
        &self,
        dir: PathBuf,
        operation: WorkspaceOperation,
    ) -> TerraformResult<i32> {
        self.execute(TerraformCommand::Workspace { dir, operation })
            .await
    }

    pub async fn execute_chain(&self, commands: Vec<TerraformCommand>) -> TerraformResult<i32> {
        let mut last_result = 0;
        for cmd in &commands {
            let result = self.execute(cmd.clone()).await;
            match result {
                Ok(code) => {
                    if let TerraformCommand::Workspace {
                        operation: WorkspaceOperation::New(_),
                        ..
                    } = cmd
                    {
                        if code != 0 {
                            continue;
                        }
                    }
                    last_result = code;
                    if code != 0 {
                        return Ok(code);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(last_result)
    }

    pub async fn execute_plan_chain(
        &self,
        dir: PathBuf,
        vars: HashMap<String, String>,
        workspace: Option<String>,
        out: Option<PathBuf>,
    ) -> TerraformResult<i32> {
        let chain = CommandChain::new(dir)
            .with_vars(vars)
            .with_out(out)
            .with_workspace(workspace);

        self.execute_chain(chain.plan_chain()).await
    }

    pub async fn execute_apply_chain(
        &self,
        dir: PathBuf,
        plan_file: Option<PathBuf>,
        workspace: Option<String>,
        auto_approve: bool,
    ) -> TerraformResult<i32> {
        let chain = CommandChain::new(dir)
            .with_out(plan_file)
            .with_workspace(workspace)
            .with_auto_approve(auto_approve);

        self.execute_chain(chain.apply_chain()).await
    }
}

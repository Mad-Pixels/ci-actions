use crate::chain::CommandChain;
use crate::command::AwsCommand;
use crate::error::{AwsError, AwsResult};
use executer::{Context, Subprocess};
use std::collections::HashMap;
use std::path::PathBuf;

/// Options for synchronizing AWS resources.
#[derive(Debug, Clone)]
pub struct SyncOptions {
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub delete: bool,
    pub dry_run: bool,
    pub force: bool,
}

impl SyncOptions {
    /// Creates a new `SyncOptions` instance with default values.
    pub fn new() -> Self {
        Self {
            exclude: None,
            include: None,
            delete: false,
            dry_run: false,
            force: false,
        }
    }

    /// Sets the exclude patterns.
    pub fn with_exclude(mut self, exclude: Option<Vec<String>>) -> Self {
        self.exclude = exclude;
        self
    }

    /// Sets the include patterns.
    pub fn with_include(mut self, include: Option<Vec<String>>) -> Self {
        self.include = include;
        self
    }

    /// Sets the delete flag.
    pub fn with_delete(mut self, delete: bool) -> Self {
        self.delete = delete;
        self
    }

    /// Sets the dry_run flag.
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Sets the force flag.
    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Executor responsible for running AWS commands.
pub struct AwsExecutor {
    subprocess: Subprocess,
    aws_path: PathBuf,
    aws_cmd: String,
}

impl AwsExecutor {
    /// Creates a new instance of `AwsExecutor`.
    ///
    /// # Arguments
    ///
    /// * `subprocess` - The `Subprocess` instance for executing commands.
    /// * `aws_path` - The path to the AWS CLI executable.
    /// * `aws_cmd` - The AWS command to execute (e.g., "s3").
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use processor::{ProcessorCollection, ProcessorItem, maskers::MaskerRegex};
    /// use aws::executor::AwsExecutor;
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    /// use std::path::PathBuf;
    /// use executer::{Output, Subprocess, Target, Validator};
    /// 
    /// # fn main() {
    /// let env = HashMap::new();
    /// let provider = AWSProvider::new(env);
    /// let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    /// let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    /// let processor = ProcessorCollection::new(processors);
    /// let aws_path = PathBuf::from("/usr/local/bin/aws");
    /// let aws_cmd = String::from("s3");
    ///
    /// let subprocess = Subprocess::new(
    ///     Output::new(processor, Target::Stdout, Target::Stderr),
    ///     Validator::default(),
    /// );
    /// let executor = AwsExecutor::new(subprocess, aws_path, aws_cmd);
    /// # }
    /// ```
    pub fn new(subprocess: Subprocess, aws_path: PathBuf, aws_cmd: String) -> Self {
        Self {
            subprocess,
            aws_path,
            aws_cmd,
        }
    }

    /// Executes a given AWS command asynchronously.
    ///
    /// # Arguments
    ///
    /// * `command` - The `AwsCommand` to execute.
    ///
    /// # Returns
    ///
    /// * `AwsResult<i32>` - The result of the command execution containing the exit code.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use aws::executor::{AwsExecutor, SyncOptions};
    /// use aws::error::AwsError;
    /// use std::path::PathBuf;
    /// use processor::{ProcessorCollection, ProcessorItem, maskers::MaskerRegex};
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    /// use executer::{Output, Subprocess, Target, Validator};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), AwsError> {
    ///     let env = HashMap::new();
    ///     let provider = AWSProvider::new(env);
    ///     
    ///     let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    ///     let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    ///     let processor = ProcessorCollection::new(processors);
    ///     let subprocess = Subprocess::new(
    ///         Output::new(processor, Target::Stdout, Target::Stderr),
    ///         Validator::default(),
    ///     );
    ///     
    ///     let executor = AwsExecutor::new(
    ///         subprocess,
    ///         PathBuf::from("/usr/local/bin/aws"),
    ///         String::from("s3"),
    ///     );
    ///     
    ///     executor.sync(
    ///         PathBuf::from("/path/to/source"),
    ///         PathBuf::from("s3://my-bucket"),
    ///         SyncOptions::new()
    ///             .with_exclude(Some(vec!["*.tmp".to_string()]))
    ///             .with_include(Some(vec!["*.log".to_string()]))
    ///             .with_delete(true)
    ///             .with_force(true),
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute(&self, command: AwsCommand) -> AwsResult<i32> {
        let args = command.to_args();

        let mut cmd = vec![self.aws_path.to_string_lossy().to_string()];
        cmd.push(self.aws_cmd.clone());
        cmd.extend(args);

        let context = Context::new(cmd, HashMap::new(), Some(PathBuf::from(".")));

        self.subprocess
            .execute(context)
            .await
            .map_err(AwsError::from)
    }

    /// Synchronizes files between source and destination.
    ///
    /// # Arguments
    ///
    /// * `source` - Source directory or S3 bucket.
    /// * `destination` - Destination directory or S3 bucket.
    /// * `options` - Synchronization options.
    ///
    /// # Returns
    ///
    /// * `AwsResult<i32>` - The result of the synchronization command execution.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use aws::executor::{AwsExecutor, SyncOptions};
    /// # use aws::error::AwsError;
    /// # use std::path::PathBuf;
    /// # use processor::{ProcessorCollection, ProcessorItem, maskers::MaskerRegex};
    /// # use provider::{AWSProvider, Provider};
    /// # use executer::{Output, Subprocess, Target, Validator};
    /// # use std::collections::HashMap;
    /// # 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), AwsError> {
    /// # let env = HashMap::new();
    /// # let provider = AWSProvider::new(env);
    /// # let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    /// # let processors = vec![ProcessorItem::Regex(regexp_processor)];
    /// # let processor = ProcessorCollection::new(processors);
    /// # let subprocess = Subprocess::new(
    /// #     Output::new(processor, Target::Stdout, Target::Stderr),
    /// #     Validator::default(),
    /// # );
    /// let executor = AwsExecutor::new(
    ///     subprocess,
    ///     PathBuf::from("/usr/local/bin/aws"),
    ///     String::from("s3"),
    /// );
    ///     
    /// let sync_options = SyncOptions::new()
    ///     .with_exclude(Some(vec!["*.tmp".to_string()]))
    ///     .with_include(Some(vec!["*.log".to_string()]))
    ///     .with_delete(true)
    ///     .with_force(true);
    ///
    /// executor.sync(
    ///     PathBuf::from("./local"),
    ///     PathBuf::from("s3://my-bucket"),
    ///     sync_options,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn sync(
        &self,
        source: PathBuf,
        destination: PathBuf,
        options: SyncOptions,
    ) -> AwsResult<i32> {
        let chain = CommandChain::new(source, destination)
            .with_exclude(options.exclude)
            .with_include(options.include)
            .with_delete(options.delete)
            .with_dry_run(options.dry_run)
            .with_force(options.force);

        self.execute_chain(chain.sync_chain()).await
    }

    /// Executes a chain of AWS commands asynchronously.
    ///
    /// # Arguments
    ///
    /// * `commands` - A vector of `AwsCommand` to execute.
    ///
    /// # Returns
    ///
    /// * `AwsResult<i32>` - The result containing the last exit code.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use aws::executor::{AwsExecutor, SyncOptions};
    /// # use aws::error::AwsError;
    /// # use aws::command::AwsCommand;
    /// # use std::path::PathBuf;
    /// # use processor::{ProcessorCollection, ProcessorItem, maskers::MaskerRegex};
    /// # use provider::{AWSProvider, Provider};
    /// # use executer::{Output, Subprocess, Target, Validator};
    /// # use std::collections::HashMap;
    /// # 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), AwsError> {
    /// # let env = HashMap::new();
    /// # let provider = AWSProvider::new(env);
    /// # let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    /// # let processors = vec![ProcessorItem::Regex(regexp_processor)];
    /// # let processor = ProcessorCollection::new(processors);
    /// # let subprocess = Subprocess::new(
    /// #     Output::new(processor, Target::Stdout, Target::Stderr),
    /// #     Validator::default(),
    /// # );
    /// let executor = AwsExecutor::new(
    ///     subprocess,
    ///     PathBuf::from("/usr/local/bin/aws"),
    ///     String::from("s3"),
    /// );
    ///
    /// let commands = vec![
    ///     AwsCommand::S3Sync {
    ///         source: PathBuf::from("/path/to/source"),
    ///         destination: PathBuf::from("s3://my-bucket"),
    ///         exclude: Some(vec!["*.tmp".to_string()]),
    ///         include: None,
    ///         delete: true,
    ///         dry_run: false,
    ///         force: false,
    ///     },
    /// ];
    ///
    /// executor.execute_chain(commands).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_chain(&self, commands: Vec<AwsCommand>) -> AwsResult<i32> {
        let mut last_result = 0;
        for cmd in &commands {
            let result = self.execute(cmd.clone()).await;
            match result {
                Ok(code) => {
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

    /// Executes a synchronization command chain.
    ///
    /// # Arguments
    ///
    /// * `source` - Source directory or S3 bucket.
    /// * `destination` - Destination directory or S3 bucket.
    /// * `options` - Synchronization options.
    ///
    /// # Returns
    ///
    /// * `AwsResult<i32>` - The result of the synchronization command execution.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use aws::executor::{AwsExecutor, SyncOptions};
    /// # use aws::error::AwsError;
    /// # use std::path::PathBuf;
    /// # use processor::{ProcessorCollection, ProcessorItem, maskers::MaskerRegex};
    /// # use provider::{AWSProvider, Provider};
    /// # use executer::{Output, Subprocess, Target, Validator};
    /// # use std::collections::HashMap;
    /// # 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), AwsError> {
    /// # let env = HashMap::new();
    /// # let provider = AWSProvider::new(env);
    /// # let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    /// # let processors = vec![ProcessorItem::Regex(regexp_processor)];
    /// # let processor = ProcessorCollection::new(processors);
    /// # let subprocess = Subprocess::new(
    /// #     Output::new(processor, Target::Stdout, Target::Stderr),
    /// #     Validator::default(),
    /// # );
    /// let executor = AwsExecutor::new(
    ///     subprocess,
    ///     PathBuf::from("/usr/local/bin/aws"),
    ///     String::from("s3"),
    /// );
    ///     
    /// let sync_options = SyncOptions::new()
    ///     .with_exclude(Some(vec!["*.tmp".to_string()]))
    ///     .with_include(Some(vec!["*.log".to_string()]))
    ///     .with_delete(true)
    ///     .with_force(true);
    ///
    /// executor.execute_sync_chain(
    ///     PathBuf::from("./local"),
    ///     PathBuf::from("s3://my-bucket"),
    ///     sync_options,
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn execute_sync_chain(
        &self,
        source: PathBuf,
        destination: PathBuf,
        options: SyncOptions,
    ) -> AwsResult<i32> {
        let chain = CommandChain::new(source, destination)
            .with_exclude(options.exclude)
            .with_include(options.include)
            .with_delete(options.delete)
            .with_dry_run(options.dry_run)
            .with_force(options.force);

        self.execute_chain(chain.sync_chain()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use executer::{Output, Target, Validator};
    use processor::ProcessorCollection;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_execute_sync() {
        let processor = ProcessorCollection::new(vec![]);
        let subprocess = Subprocess::new(
            Output::new(processor, Target::Stdout, Target::Stderr),
            Validator::default(),
        );

        let executor = AwsExecutor::new(
            subprocess,
            PathBuf::from("/usr/local/bin/aws"),
            String::from("s3"),
        );

        let result = executor
            .sync(
                PathBuf::from("./test-data"),
                PathBuf::from("s3://test-bucket"),
                SyncOptions::new()
                    .with_exclude(Some(vec!["*.tmp".to_string()]))
                    .with_delete(true),
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_sync_chain() {
        let processor = ProcessorCollection::new(vec![]);
        let subprocess = Subprocess::new(
            Output::new(processor, Target::Stdout, Target::Stderr),
            Validator::default(),
        );

        let executor = AwsExecutor::new(
            subprocess,
            PathBuf::from("/usr/local/bin/aws"),
            String::from("s3"),
        );

        let result = executor
            .execute_sync_chain(
                PathBuf::from("./test-data"),
                PathBuf::from("s3://test-bucket"),
                SyncOptions::new()
                    .with_exclude(Some(vec!["*.tmp".to_string()]))
                    .with_delete(true),
            )
            .await;

        assert!(result.is_ok());
    }
}
use crate::command::{AwsCommand, LambdaUpdateType};
use crate::error::{AwsError, AwsResult};

use executer::{Context, Output, Subprocess, Target, Validator};
use processor::ProcessorCollection;
use std::path::PathBuf;

/// Options for synchronizing files between source and destination.
#[derive(Debug, Clone)]
pub struct SyncOptions {
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub delete: bool,
    pub dry_run: bool,
    pub force: bool,
}

impl SyncOptions {
    pub fn new() -> Self {
        Self {
            exclude: None,
            include: None,
            delete: false,
            dry_run: false,
            force: false,
        }
    }

    pub fn with_exclude(mut self, exclude: Option<Vec<String>>) -> Self {
        self.exclude = exclude;
        self
    }

    pub fn with_include(mut self, include: Option<Vec<String>>) -> Self {
        self.include = include;
        self
    }

    pub fn with_delete(mut self, delete: bool) -> Self {
        self.delete = delete;
        self
    }

    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

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
}

impl AwsExecutor {
    /// Creates a new instance of `AwsExecutor`.
    ///
    /// # Arguments
    ///
    /// * `processor` - A collection of maskers for processing output.
    /// * `aws_path` - The path to the AWS executable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use aws::executor::AwsExecutor;
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
    /// let aws_path = PathBuf::from("/usr/local/bin/aws");
    /// let executor = AwsExecutor::new(processor, aws_path);
    /// ```
    pub fn new(processor: ProcessorCollection, aws_path: PathBuf) -> Self {
        let output = Output::new(processor, Target::Stdout, Target::Stderr);
        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        Self {
            subprocess,
            aws_path,
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
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AwsError> {
    ///     let env = HashMap::new();
    ///     let provider = AWSProvider::new(env.clone());
    ///     
    ///     let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    ///     let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    ///     let processor = ProcessorCollection::new(processors);
    ///     let aws_path = PathBuf::from("/usr/local/bin/aws");
    ///     let executor = AwsExecutor::new(processor, aws_path);
    ///     
    ///     executor.sync(
    ///         PathBuf::from("./local"),
    ///         PathBuf::from("s3://my-bucket"),
    ///         SyncOptions::new()
    ///             .with_exclude(Some(vec!["*.tmp".to_string()]))
    ///             .with_delete(true)
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn execute(&self, command: AwsCommand) -> AwsResult<i32> {
        let args = command.to_args();
        let working_dir = match &command {
            AwsCommand::S3Sync { source, .. } => {
                let default_path = PathBuf::from(".");
                let parent = source.parent().unwrap_or(&default_path);
                PathBuf::from(parent)
            }
            AwsCommand::CloudFrontInvalidate { .. } => PathBuf::from("."),
            AwsCommand::LambdaUpdateCode { .. } => PathBuf::from("."),
        };

        let mut cmd = vec![self.aws_path.to_string_lossy().to_string()];
        cmd.extend(args);

        let context = Context::new(cmd, std::collections::HashMap::new(), Some(working_dir));

        self.subprocess
            .execute(context)
            .await
            .map_err(AwsError::from)
    }

    /// Synchronizes files between a local directory and an S3 bucket or between two S3 buckets.
    ///
    /// # Arguments
    ///
    /// * `source` - Source directory or S3 bucket.
    /// * `destination` - Destination directory or S3 bucket.
    /// * `options` - Synchronization options including exclusions, inclusions, and flags.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use aws::executor::{AwsExecutor, SyncOptions};
    /// use aws::error::AwsError;
    /// use std::path::PathBuf;
    /// use processor::{maskers::MaskerEqual, maskers::MaskerRegex, ProcessorCollection, ProcessorItem};
    /// use provider::{AWSProvider, Provider};
    /// use std::collections::HashMap;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), AwsError> {
    ///     let env = HashMap::new();
    ///     let provider = AWSProvider::new(env.clone());
    ///     
    ///     let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    ///     let processors = vec![ProcessorItem::Regex(regexp_processor)];
    ///
    ///     let processor = ProcessorCollection::new(processors);
    ///     let aws_path = PathBuf::from("/usr/local/bin/aws");
    ///     let executor = AwsExecutor::new(processor, aws_path);
    ///     
    ///     executor.sync(
    ///         PathBuf::from("./local"),
    ///         PathBuf::from("s3://my-bucket"),
    ///         SyncOptions::new()
    ///             .with_exclude(Some(vec!["*.tmp".to_string()]))
    ///             .with_delete(true)
    ///     ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn sync(
        &self,
        source: PathBuf,
        destination: PathBuf,
        options: SyncOptions,
    ) -> AwsResult<i32> {
        self.execute(AwsCommand::S3Sync {
            source,
            destination,
            exclude: options.exclude,
            include: options.include,
            delete: options.delete,
            dry_run: options.dry_run,
            force: options.force,
        })
        .await
    }

    /// Invalidates CloudFront distribution cache for specified paths.
    ///
    /// # Arguments
    ///
    /// * `distribution_id` - CloudFront distribution ID
    /// * `paths` - List of paths to invalidate
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use aws::executor::AwsExecutor;
    /// use aws::error::AwsError;
    /// use processor::{ProcessorCollection, ProcessorItem};
    /// use std::path::PathBuf;
    ///
    /// # async fn example() -> Result<(), AwsError> {
    /// # let processor = ProcessorCollection::new(vec![]);
    /// # let aws_path = PathBuf::from("/usr/local/bin/aws");
    /// let executor = AwsExecutor::new(processor, aws_path);
    ///
    /// executor.invalidate_cache(
    ///     "E1234567890ABCD",
    ///     vec!["/*".to_string()]
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn invalidate_cache(
        &self,
        distribution_id: &str,
        paths: Vec<String>,
    ) -> AwsResult<i32> {
        self.execute(AwsCommand::CloudFrontInvalidate {
            distribution_id: distribution_id.to_string(),
            paths,
        })
        .await
    }

    /// Updates Lambda function code using a ZIP file.
    ///
    /// # Arguments
    ///
    /// * `function_name` - Name of the Lambda function
    /// * `zip_file` - Path to ZIP file containing function code
    /// * `publish` - Whether to publish a new version
    pub async fn update_lambda_code_zip(
        &self,
        function_name: &str,
        zip_file: PathBuf,
        publish: bool,
    ) -> AwsResult<i32> {
        self.execute(AwsCommand::LambdaUpdateCode {
            function_name: function_name.to_string(),
            update_type: LambdaUpdateType::Zip { zip_file },
            publish,
        })
        .await
    }

    /// Updates Lambda function code using a container image.
    ///
    /// # Arguments
    ///
    /// * `function_name` - Name of the Lambda function
    /// * `image_uri` - URI of the container image
    /// * `publish` - Whether to publish a new version
    pub async fn update_lambda_code_container(
        &self,
        function_name: &str,
        image_uri: &str,
        publish: bool,
    ) -> AwsResult<i32> {
        self.execute(AwsCommand::LambdaUpdateCode {
            function_name: function_name.to_string(),
            update_type: LambdaUpdateType::Container {
                image_uri: image_uri.to_string(),
            },
            publish,
        })
        .await
    }

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
}

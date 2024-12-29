use crate::command::AWSCommand;
use crate::error::{AWSError, AWSResult};
use executer::{Context, Output, Subprocess, Target, Validator};
use processor::ProcessorCollection;
use std::path::PathBuf;

pub struct AWSExecutor {
    subprocess: Subprocess,
    aws_path: PathBuf,
}

impl AWSExecutor {
    pub fn new(processor: ProcessorCollection, aws_path: PathBuf) -> Self {
        let output = Output::new(processor, Target::Stdout, Target::Stderr);
        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        Self {
            subprocess,
            aws_path,
        }
    }

    pub async fn execute(&self, command: AWSCommand) -> AWSResult<i32> {
        let args = command.to_args();
        let working_dir = match &command {
            AWSCommand::S3Sync { source, .. } => {
                let default_path = PathBuf::from(".");
                let parent = source.parent().unwrap_or(&default_path);
                PathBuf::from(parent)
            }
        };
    
        let mut cmd = vec![self.aws_path.to_string_lossy().to_string()];
        cmd.extend(args);
    
        let context = Context::new(
            cmd,
            std::collections::HashMap::new(),
            Some(working_dir),
        );
    
        self.subprocess
            .execute(context)
            .await
            .map_err(AWSError::from)
    }

    pub async fn s3_sync(
        &self,
        source: PathBuf,
        bucket: String,
        prefix: String,
        delete: bool,
    ) -> AWSResult<i32> {
        self.execute(AWSCommand::S3Sync {
            source,
            bucket,
            prefix,
            delete,
        })
        .await
    }
}
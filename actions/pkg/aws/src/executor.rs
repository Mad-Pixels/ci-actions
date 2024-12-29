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
            AWSCommand::S3Copy { source, .. } => source.parent().unwrap_or(&PathBuf::from(".")),
            AWSCommand::LambdaUpdateCode { .. } => zip_file.parent().unwrap_or(&PathBuf::from(".")),
        };

        let mut cmd = vec![self.aws_path.to_string_lossy().to_string()];
        cmd.extend(args);

        let context = Context::new(
            cmd,
            std::collections::HashMap::new(),
            Some(working_dir.to_path_buf()),
        );

        self.subprocess
            .execute(context)
            .await
            .map_err(AWSError::from)
    }

    pub async fn s3_copy(&self, source: PathBuf, bucket: String, key: String) -> AWSResult<i32> {
        self.execute(AWSCommand::S3Copy {
            source,
            bucket,
            key,
        })
        .await
    }

    pub async fn lambda_update_code(
        &self,
        function_name: String,
        publish: bool,
    ) -> AWSResult<i32> {
        self.execute(AWSCommand::LambdaUpdateCode {
            function_name,
            publish,
        })
        .await
    }
}
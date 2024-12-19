use std::collections::HashMap;
use std::path::PathBuf;
use async_trait::async_trait;
use futures::{Stream, StreamExt};
use tokio::process::Command;
use slog::Logger;
use std::pin::Pin;
use processor::Collection;
use crate::{
    BaseExecuter,
    CommandExecuter,
    ExecuterError,
    ExecutionResult,
};
use crate::utils::stream::{stream_output, StreamType};

pub struct SubprocessExecuter {
    base: BaseExecuter,
}

impl SubprocessExecuter {
    pub fn new(processor: Option<Collection>, logger: Logger) -> Self {
        Self {
            base: BaseExecuter::new(processor, logger)
        }
    }

    pub fn logger(&self) -> &Logger {
        self.base.logger()
    }

    pub fn processor(&self) -> Option<&Collection> {
        self.base.processor()
    }

    async fn build_result(
        output_lines: Vec<(StreamType, String, Option<String>)>,
        status: i32,
    ) -> Result<ExecutionResult, ExecuterError> {
        let mut stdout_lines = Vec::new();
        let mut stderr_lines = Vec::new();
        let mut masked_stdout_lines = Vec::new();
        let mut masked_stderr_lines = Vec::new();

        for (stream_type, line, masked_line) in output_lines {
            match stream_type {
                StreamType::Stdout => {
                    stdout_lines.push(line);
                    if let Some(masked) = masked_line {
                        masked_stdout_lines.push(masked);
                    }
                },
                StreamType::Stderr => {
                    stderr_lines.push(line);
                    if let Some(masked) = masked_line {
                        masked_stderr_lines.push(masked);
                    }
                }
            }
        }

        let result = ExecutionResult::new(
            status,
            stdout_lines.join("\n"),
            stderr_lines.join("\n"),
            if !masked_stdout_lines.is_empty() { Some(masked_stdout_lines.join("\n")) } else { None },
            if !masked_stderr_lines.is_empty() { Some(masked_stderr_lines.join("\n")) } else { None }
        ).map_err(|e| ExecuterError::ExecutionError(e))?;

        Ok(result)
    }
}

#[async_trait]
impl CommandExecuter for SubprocessExecuter {
    async fn execute_stream(
        &self,
        cmd: Vec<String>,
        env: Option<HashMap<String, String>>,
        cwd: Option<PathBuf>,
        mask: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, ExecuterError>> + Send + '_>>, ExecuterError> {
        let env = env.unwrap_or_default();
        self.base.validate_inputs(&cmd, &env, &cwd)?;

        slog::debug!(self.logger(), "Executing command: {}", cmd.join(" "));

        let mut command = Command::new(&cmd[0]);
        command.args(&cmd[1..])
               .envs(&env)
               .stdout(std::process::Stdio::piped())
               .stderr(std::process::Stdio::piped());

        if let Some(path) = cwd.clone() {
            command.current_dir(path);
        }

        let mut child = command.spawn()
            .map_err(|e| ExecuterError::ExecutionError(format!("Failed to spawn command: {}", e)))?;

        let stdout = child.stdout.take()
            .ok_or_else(|| ExecuterError::ExecutionError("Failed to capture stdout".to_string()))?;
        let stderr = child.stderr.take()
            .ok_or_else(|| ExecuterError::ExecutionError("Failed to capture stderr".to_string()))?;

        let processor = if mask { self.processor() } else { None };
        let output_stream = stream_output(stdout, stderr, processor, self.logger()).await;
        let mut output_lines = Vec::new();
        
        let stream = async_stream::stream! {
            let mut stream = output_stream;
            
            while let Some(result) = stream.next().await {
                match result {
                    Ok(item) => {
                        let display_line = match item.stream_type {
                            StreamType::Stdout => format!("[stdout] {}", item.masked_line.as_ref().unwrap_or(&item.line)),
                            StreamType::Stderr => format!("[stderr] {}", item.masked_line.as_ref().unwrap_or(&item.line)),
                        };
                        
                        output_lines.push((item.stream_type, item.line, item.masked_line));
                        yield Ok(display_line);
                    }
                    Err(e) => yield Err(e),
                }
            }

            let status = child.wait().await
                .map_err(|e| ExecuterError::ExecutionError(format!("Failed to wait for command: {}", e)))?;

            let result = Self::build_result(output_lines, status.code().unwrap_or(-1)).await?;
            yield Ok(format!("Command finished with status: {}", result.status));
        };

        Ok(Box::pin(stream))
    }
}
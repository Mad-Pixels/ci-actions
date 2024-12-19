use std::collections::HashMap;
use std::path::PathBuf;
use processor::Collection;
use slog::Logger;
use crate::utils::validation::{validate_command, validate_env, validate_cwd};
use crate::ExecuterError;

pub struct BaseExecuter {
    processor: Option<Collection>,
    logger: Logger,
}

impl BaseExecuter {
    pub fn new(processor: Option<Collection>, logger: Logger) -> Self {
        Self { processor, logger }
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn processor(&self) -> Option<&Collection> {
        self.processor.as_ref()
    }

    pub(crate) fn validate_inputs(
        &self,
        cmd: &[String],
        env: &HashMap<String, String>,
        cwd: &Option<PathBuf>,
    ) -> Result<(), ExecuterError> {
        validate_command(cmd)?;
        validate_env(env)?;
        validate_cwd(cwd)?;
        Ok(())
    }
}
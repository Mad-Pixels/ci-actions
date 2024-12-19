use std::collections::HashMap;
use std::path::PathBuf;
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
use slog::Logger;
use processor::Collection;
use crate::{CommandExecuter, ExecuterError, SubprocessExecuter};

pub struct IsolateExecuter {
    inner: SubprocessExecuter,
    isolated_env: HashMap<String, String>,
}

impl IsolateExecuter {
    pub fn new(
        processor: Option<Collection>,
        logger: Logger,
        isolated_env: HashMap<String, String>
    ) -> Self {
        Self {
            inner: SubprocessExecuter::new(processor, logger.clone()),
            isolated_env,
        }
    }

    fn prepare_environment(
        &self,
        additional_env: Option<HashMap<String, String>>
    ) -> HashMap<String, String> {
        let mut final_env = self.isolated_env.clone();
        
        if let Some(env) = additional_env {
            final_env.extend(env);
        }

        final_env
    }
}

#[async_trait]
impl CommandExecuter for IsolateExecuter {
    async fn execute_stream(
        &self,
        cmd: Vec<String>,
        env: Option<HashMap<String, String>>,
        cwd: Option<PathBuf>,
        mask: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, ExecuterError>> + Send + '_>>, ExecuterError> {
        let isolated_env = self.prepare_environment(env);
        
        slog::debug!(
            self.inner.logger(),
            "Executing command in isolated environment";
            "env_vars" => format!("{:?}", isolated_env.keys())
        );

        self.inner.execute_stream(cmd, Some(isolated_env), cwd, mask).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
    use processor::{Collection, Item, maskers::regex::MaskerRegex};
    use slog::{Logger, Drain, o};

    fn setup_logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        Logger::root(drain, o!())
    }

    #[tokio::test]
    async fn test_isolated_environment() {
        let logger = setup_logger();
        let mut isolated_env = HashMap::new();
        isolated_env.insert("TEST_VAR".to_string(), "test_value".to_string());

        let executer = IsolateExecuter::new(None, logger, isolated_env);

        let mut stream = executer.execute_stream(
            vec!["sh".to_string(), "-c".to_string(), "echo $TEST_VAR".to_string()],
            None,
            None,
            false
        ).await.unwrap();

        let mut output = Vec::new();
        while let Some(line) = stream.next().await {
            output.push(line.unwrap());
        }

        assert!(output.iter().any(|line| line.contains("test_value")));
    }

    #[tokio::test]
    async fn test_isolated_environment_with_masking() {
        let logger = setup_logger();
        let masker = MaskerRegex::new(vec![r"secret_\w+"], "****");
        let collection = Collection::new(vec![Item::Regex(masker)]);

        let mut isolated_env = HashMap::new();
        isolated_env.insert("TEST_VAR".to_string(), "secret_value".to_string());

        let executer = IsolateExecuter::new(Some(collection), logger, isolated_env);

        let mut stream = executer.execute_stream(
            vec!["sh".to_string(), "-c".to_string(), "echo $TEST_VAR".to_string()],
            None,
            None,
            true
        ).await.unwrap();

        let mut output = Vec::new();
        while let Some(line) = stream.next().await {
            output.push(line.unwrap());
        }

        assert!(output.iter().any(|line| line.contains("****")));
        assert!(!output.iter().any(|line| line.contains("secret_value")));
    }
}
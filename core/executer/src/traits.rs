use std::collections::HashMap;
use std::path::PathBuf;
use async_trait::async_trait;
use futures::Stream;
use std::pin::Pin;
use crate::ExecuterError;

#[async_trait]
pub trait CommandExecuter: Send + Sync {
    async fn execute_stream(
        &self,
        cmd: Vec<String>,
        env: Option<HashMap<String, String>>,
        cwd: Option<PathBuf>,
        mask: bool,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String, ExecuterError>> + Send + '_>>, ExecuterError>;
}
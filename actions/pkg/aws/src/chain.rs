use crate::command::AwsCommand;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CommandChain {
    dir: PathBuf,
    vars: HashMap<String, String>,
}

impl CommandChain {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            vars: HashMap::new(),
        }
    }

    pub fn with_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.vars = vars;
        self
    }

    fn build_sync(&self) -> AwsCommand {
        AwsCommand::S3Sync {
            source: self.dir.clone(),
            destination: PathBuf::from("s3://my-bucket"), // This should come from config
            exclude: None,
            include: None,
            delete: false,
            dry_run: false,
            force: false,
        }
    }

    pub fn sync_chain(&self) -> Vec<AwsCommand> {
        vec![self.build_sync()]
    }
}

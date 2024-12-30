use crate::command::AwsCommand;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct CommandChain {
    dir: PathBuf,
    vars: HashMap<String, String>,
    destination: Option<PathBuf>,
    exclude: Option<Vec<String>>,
    include: Option<Vec<String>>,
    delete: bool,
    dry_run: bool,
    force: bool,
}

impl CommandChain {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir,
            vars: HashMap::new(),
            destination: None,
            exclude: None,
            include: None,
            delete: false,
            dry_run: false,
            force: false,
        }
    }

    pub fn with_vars(mut self, vars: HashMap<String, String>) -> Self {
        self.vars = vars;
        self
    }

    pub fn with_destination(mut self, destination: PathBuf) -> Self {
        self.destination = Some(destination);
        self
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

    fn build_sync(&self) -> AwsCommand {
        AwsCommand::S3Sync {
            source: self.dir.clone(),
            destination: self.destination.clone().expect("Destination must be set"),
            exclude: self.exclude.clone(),
            include: self.include.clone(),
            delete: self.delete,
            dry_run: self.dry_run,
            force: self.force,
        }
    }

    pub fn sync_chain(&self) -> Vec<AwsCommand> {
        vec![self.build_sync()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_chain_defaults() {
        let chain = CommandChain::new(PathBuf::from("/test/path"));

        assert!(chain.destination.is_none());
        assert!(chain.exclude.is_none());
        assert!(chain.include.is_none());
        assert!(!chain.delete);
        assert!(!chain.dry_run);
        assert!(!chain.force);
    }

    #[test]
    fn test_command_chain_with_options() {
        let chain = CommandChain::new(PathBuf::from("/test/path"))
            .with_destination(PathBuf::from("s3://test-bucket"))
            .with_exclude(Some(vec!["*.tmp".to_string()]))
            .with_include(Some(vec!["*.log".to_string()]))
            .with_delete(true)
            .with_dry_run(true)
            .with_force(true);

        assert_eq!(
            chain.destination.unwrap(),
            PathBuf::from("s3://test-bucket")
        );
        assert_eq!(chain.exclude.unwrap(), vec!["*.tmp".to_string()]);
        assert_eq!(chain.include.unwrap(), vec!["*.log".to_string()]);
        assert!(chain.delete);
        assert!(chain.dry_run);
        assert!(chain.force);
    }

    #[test]
    #[should_panic(expected = "Destination must be set")]
    fn test_build_sync_without_destination() {
        let chain = CommandChain::new(PathBuf::from("/test/path"));
        chain.build_sync();
    }
}

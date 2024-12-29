use crate::command::AwsCommand;
use std::path::PathBuf;

/// Represents a chain of AWS commands to be executed.
#[derive(Debug, Clone)]
pub struct CommandChain {
    source: PathBuf,
    destination: PathBuf,
    exclude: Option<Vec<String>>,
    include: Option<Vec<String>>,
    delete: bool,
    dry_run: bool,
    force: bool,
}

impl CommandChain {
    /// Creates a new `CommandChain` for S3 synchronization.
    ///
    /// # Arguments
    ///
    /// * `source` - Source directory or S3 bucket.
    /// * `destination` - Destination directory or S3 bucket.
    pub fn new(source: PathBuf, destination: PathBuf) -> Self {
        Self {
            source,
            destination,
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

    /// Sets the dry run flag.
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Sets the force flag.
    pub fn with_force(mut self, force: bool) -> Self {
        self.force = force;
        self
    }

    /// Builds the S3Sync command.
    fn build_sync(&self) -> AwsCommand {
        AwsCommand::S3Sync {
            source: self.source.clone(),
            destination: self.destination.clone(),
            exclude: self.exclude.clone(),
            include: self.include.clone(),
            delete: self.delete,
            dry_run: self.dry_run,
            force: self.force,
        }
    }

    /// Returns the chain of commands for S3 synchronization.
    pub fn sync_chain(&self) -> Vec<AwsCommand> {
        vec![self.build_sync()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::AwsCommand;
    use std::path::PathBuf;

    #[test]
    fn test_command_chain_build_sync() {
        let source = PathBuf::from("/path/to/source");
        let destination = PathBuf::from("s3://my-bucket");
        let chain = CommandChain::new(source.clone(), destination.clone())
            .with_exclude(Some(vec!["*.tmp".to_string()]))
            .with_include(Some(vec!["*.log".to_string()]))
            .with_delete(true)
            .with_dry_run(false)
            .with_force(true);

        let commands = chain.sync_chain();
        assert_eq!(commands.len(), 1);

        match &commands[0] {
            AwsCommand::S3Sync {
                source: cmd_source,
                destination: cmd_destination,
                exclude,
                include,
                delete,
                dry_run,
                force,
            } => {
                assert_eq!(cmd_source, &source);
                assert_eq!(cmd_destination, &destination);
                assert_eq!(exclude.as_ref().unwrap(), &vec!["*.tmp".to_string()]);
                assert_eq!(include.as_ref().unwrap(), &vec!["*.log".to_string()]);
                assert!(delete);
                assert!(!dry_run);
                assert!(force);
            }
        }
    }

    #[test]
    fn test_command_chain_defaults() {
        let source = PathBuf::from("/path/to/source");
        let destination = PathBuf::from("s3://my-bucket");
        let chain = CommandChain::new(source.clone(), destination.clone());

        let commands = chain.sync_chain();
        assert_eq!(commands.len(), 1);

        match &commands[0] {
            AwsCommand::S3Sync {
                source: cmd_source,
                destination: cmd_destination,
                exclude,
                include,
                delete,
                dry_run,
                force,
            } => {
                assert_eq!(cmd_source, &source);
                assert_eq!(cmd_destination, &destination);
                assert!(exclude.is_none());
                assert!(include.is_none());
                assert!(!delete);
                assert!(!dry_run);
                assert!(!force);
            }
        }
    }
}

/// Represents the various AWS commands that can be executed.
#[derive(Debug, Clone)]
pub enum AwsCommand {
    /// Synchronizes files between a local directory and an S3 bucket or between two S3 buckets.
    ///
    /// # Fields
    ///
    /// - `source`: Source directory or S3 bucket.
    /// - `destination`: Destination directory or S3 bucket.
    /// - `exclude`: Optional patterns to exclude.
    /// - `include`: Optional patterns to include.
    /// - `delete`: Whether to delete files in the destination not present in the source.
    /// - `dry_run`: Whether to perform a dry run.
    /// - `force`: Whether to force synchronization.
    S3Sync {
        source: std::path::PathBuf,
        destination: std::path::PathBuf,
        exclude: Option<Vec<String>>,
        include: Option<Vec<String>>,
        delete: bool,
        dry_run: bool,
        force: bool,
    },
}

impl AwsCommand {
    /// Converts the `AwsCommand` into a list of command-line arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::path::PathBuf;
    /// use aws::command::AwsCommand;
    ///
    /// let sync_command = AwsCommand::S3Sync {
    ///     source: PathBuf::from("./local"),
    ///     destination: PathBuf::from("s3://my-bucket"),
    ///     exclude: Some(vec!["*.tmp".to_string()]),
    ///     include: None,
    ///     delete: true,
    ///     dry_run: false,
    ///     force: false,
    /// };
    ///
    /// let args = sync_command.to_args();
    /// assert_eq!(
    ///     args,
    ///     vec![
    ///         "s3".to_string(),
    ///         "sync".to_string(),
    ///         "./local".to_string(),
    ///         "s3://my-bucket".to_string(),
    ///         "--exclude=*.tmp".to_string(),
    ///         "--delete".to_string()
    ///     ]
    /// );
    /// ```
    pub fn to_args(&self) -> Vec<String> {
        match self {
            Self::S3Sync {
                source,
                destination,
                exclude,
                include,
                delete,
                dry_run,
                force,
            } => {
                let mut args = vec![
                    "s3".to_string(),
                    "sync".to_string(),
                    source.to_string_lossy().to_string(),
                    destination.to_string_lossy().to_string(),
                ];

                if let Some(excludes) = exclude {
                    for pattern in excludes {
                        args.push(format!("--exclude={}", pattern));
                    }
                }

                if let Some(includes) = include {
                    for pattern in includes {
                        args.push(format!("--include={}", pattern));
                    }
                }

                if *delete {
                    args.push("--delete".to_string());
                }

                if *dry_run {
                    args.push("--dryrun".to_string());
                }

                if *force {
                    args.push("--force".to_string());
                }

                args
            }
        }
    }
}

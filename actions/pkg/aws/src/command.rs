use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum LambdaUpdateType {
    Zip { zip_file: PathBuf },
    Container { image_uri: String },
}

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
        source: PathBuf,
        destination: PathBuf,
        exclude: Option<Vec<String>>,
        include: Option<Vec<String>>,
        delete: bool,
        dry_run: bool,
        force: bool,
    },

    CloudFrontInvalidate {
        distribution_id: String,
        paths: Vec<String>,
    },

    LambdaUpdateCode {
        function_name: String,
        update_type: LambdaUpdateType,
        publish: bool,
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
    ///         "--delete".to_string(),
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

            Self::CloudFrontInvalidate {
                distribution_id,
                paths,
            } => {
                let mut args = vec![
                    "cloudfront".to_string(),
                    "create-invalidation".to_string(),
                    "--distribution-id".to_string(),
                    distribution_id.clone(),
                ];

                let paths_json = format!(
                    "{{\"Paths\":{{\"Quantity\":{},\"Items\":[{}]}}}}",
                    paths.len(),
                    paths
                        .iter()
                        .map(|p| format!("\"{}\"", p))
                        .collect::<Vec<_>>()
                        .join(",")
                );

                args.push("--invalidation-batch".to_string());
                args.push(paths_json);
                args
            }

            Self::LambdaUpdateCode {
                function_name,
                update_type,
                publish,
            } => {
                let mut args = vec![
                    "lambda".to_string(),
                    "update-function-code".to_string(),
                    "--function-name".to_string(),
                    function_name.clone(),
                ];

                match update_type {
                    LambdaUpdateType::Zip { zip_file } => {
                        args.push("--zip-file".to_string());
                        args.push(format!("fileb://{}", zip_file.to_string_lossy()));
                    }
                    LambdaUpdateType::Container { image_uri } => {
                        args.push("--image-uri".to_string());
                        args.push(image_uri.clone());
                    }
                }

                if *publish {
                    args.push("--publish".to_string());
                }

                args
            }
        }
    }
}

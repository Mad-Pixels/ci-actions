use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum AWSCommand {
    /// Sync directory with S3 bucket
    S3Sync {
        source: PathBuf,
        bucket: String,
        prefix: String,
        delete: bool,
    },
}

impl AWSCommand {
    pub fn to_args(&self) -> Vec<String> {
        match self {
            Self::S3Sync { source: _, bucket, prefix, delete } => {
                let mut args = vec![
                    "s3".to_string(),
                    "sync".to_string(),
                    format!("s3://{}/{}", bucket, prefix),
                ];
                
                if *delete {
                    args.push("--delete".to_string());
                }
                
                args
            },
        }
    }
}
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum AWSCommand {
    S3Copy {
        source: PathBuf,
        bucket: String,
        key: String,
    },

    LambdaUpdateCode {
        function_name: String,
        publish: bool,
    }
}

impl AWSCommand {
    pub fn to_args(&self) -> Vec<String> {
        match self {
            Self::S3Copy { source: _, bucket, key } => {
                vec![
                    "s3".to_string(),
                    "cp".to_string(),
                    format!("s3://{}/{}", bucket, key),
                ]
            },
            Self::LambdaUpdateCode { function_name, publish } {
                let mut args = vec![
                    "lambda".to_string(),
                    "update-function-code".to_string(),
                    function_name.clone(),
                ];
                if *publish {
                    args.push("--publish".to_string());
                }
                args
            }
        }
    }
}
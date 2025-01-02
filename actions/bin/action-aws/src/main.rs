use aws::{executor::AwsExecutor, AwsConfig, AwsEnv, CommandChain};
use config::MainConfig;
use processor::{MaskerEqual, MaskerRegex, ProcessorCollection, ProcessorItem};

use provider::auto_detect;
use util::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_config = MainConfig::new();
    let aws_config = AwsConfig::new();

    let level = main_config.get_log_level().unwrap_or("info".to_string());
    let logger = init_logger(&level);

    let provider = match auto_detect() {
        Ok(v) => {
            slog::info!(logger, "Initialize action with provider {}", v.name());
            v
        }
        Err(e) => {
            slog::error!(logger, "Failed to detect provider"; "error" => e.to_string());
            return Err(e.into());
        }
    };

    let cwd = match main_config.get_working_dir() {
        Ok(v) => {
            slog::info!(logger, "Workdir: {:?}", v);
            v
        }
        Err(e) => {
            slog::error!(logger, "Work directory not set"; "error" => e.to_string());
            return Err(e.into());
        }
    };

    let cmd = match aws_config.get_cmd() {
        Ok(v) => {
            slog::info!(logger, "Action command: {}", v);
            v
        }
        Err(e) => {
            slog::error!(logger, "Invalid aws command"; "error" => e.to_string());
            return Err(e.into());
        }
    };

    let mask = match main_config.get_mask() {
        Ok(v) => {
            slog::debug!(logger, "mask string: {}", v);
            v
        }
        Err(e) => {
            slog::error!(logger, "Mask string not set"; "error" => e.to_string());
            return Err(e.into());
        }
    };

    let bin = match aws_config.get_bin() {
        Ok(v) => {
            slog::debug!(logger, "aws binary file: {:?}", v);
            v
        }
        Err(e) => {
            slog::error!(logger, "Invalid aws bin filepath"; "error" => e.to_string());
            return Err(e.into());
        }
    };

    let envs = AwsEnv::new();

    let masker_provider_output = match MaskerRegex::new(
        provider.get_predefined_masked_objects(),
        &mask,
    ) {
        Ok(v) => v,
        Err(e) => {
            slog::error!(logger, "Failed to initialize maskers for provider"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    let masker_provider_credentials = MaskerEqual::new(provider.values(), &mask);
    let masker_aws_envs = MaskerEqual::new(envs.values(), &mask);

    let processor = ProcessorCollection::new(vec![
        ProcessorItem::Regex(masker_provider_output),
        ProcessorItem::Equal(masker_provider_credentials),
        ProcessorItem::Equal(masker_aws_envs),
    ]);
    slog::info!(logger, "Action was initialized");

    let executor = AwsExecutor::new(processor, bin);

    let result = match cmd.as_str() {
        "s3_sync" => {
            let destination = match aws_config.get_destination() {
                Ok(v) => {
                    //slog::info!(logger, "S3 destination: {:?}", v);
                    v
                }
                Err(e) => {
                    slog::error!(logger, "S3 destination not set"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let exclude = match aws_config.get_exclude() {
                Ok(v) => {
                    if let Some(patterns) = &v {
                        slog::info!(logger, "Exclude patterns: {:?}", patterns);
                    }
                    v
                }
                Err(e) => {
                    slog::error!(logger, "Failed to get exclude patterns"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let include = match aws_config.get_include() {
                Ok(v) => {
                    if let Some(patterns) = &v {
                        slog::info!(logger, "Include patterns: {:?}", patterns);
                    }
                    v
                }
                Err(e) => {
                    slog::error!(logger, "Failed to get include patterns"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let delete = match aws_config.get_delete() {
                Ok(v) => {
                    if v {
                        slog::info!(logger, "Delete option enabled");
                    }
                    v
                }
                Err(e) => {
                    slog::error!(logger, "Failed to get delete option"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let dry_run = match aws_config.get_dry_run() {
                Ok(v) => {
                    if v {
                        slog::info!(logger, "Dry run mode enabled");
                    }
                    v
                }
                Err(e) => {
                    slog::error!(logger, "Failed to get dry run option"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let force = match aws_config.get_force() {
                Ok(v) => {
                    if v {
                        slog::info!(logger, "Force mode enabled");
                    }
                    v
                }
                Err(e) => {
                    slog::error!(logger, "Failed to get force option"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let chain = CommandChain::new(cwd)
                .with_vars(envs.as_map())
                .with_destination(destination)
                .with_exclude(exclude)
                .with_include(include)
                .with_delete(delete)
                .with_dry_run(dry_run)
                .with_force(force);

            slog::info!(logger, "Starting AWS S3 sync command");
            executor.execute_chain(chain.sync_chain()).await
        }
        "cloudfront_invalidate" => {
            let distribution_id = match aws_config.get_cloudfront_distribution() {
                Ok(v) => {
                    slog::info!(logger, "CloudFront distribution ID: {}", v);
                    v
                }
                Err(e) => {
                    slog::error!(logger, "CloudFront distribution ID not set"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let paths = match aws_config.get_cloudfront_paths() {
                Ok(v) => {
                    slog::info!(logger, "CloudFront invalidation paths: {:?}", v);
                    v
                }
                Err(e) => {
                    slog::error!(logger, "CloudFront invalidation paths not set"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let chain = CommandChain::new(cwd).with_vars(envs.as_map());

            slog::info!(logger, "Starting CloudFront invalidation");
            executor
                .execute_chain(chain.invalidate_chain(distribution_id, paths))
                .await
        }
        "lambda_update" => {
            let function_name = match aws_config.get_lambda_function() {
                Ok(v) => {
                    slog::info!(logger, "Lambda function name: {}", v);
                    v
                }
                Err(e) => {
                    slog::error!(logger, "Lambda function name not set"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let publish = match aws_config.get_lambda_publish() {
                Ok(v) => {
                    if v {
                        slog::info!(logger, "Lambda publish enabled");
                    }
                    v
                }
                Err(e) => {
                    slog::error!(logger, "Failed to get Lambda publish option"; "error" => e.to_string());
                    return Err(e.into());
                }
            };

            let chain = CommandChain::new(cwd).with_vars(envs.as_map());

            if let Ok(image_uri) = aws_config.get_lambda_image() {
                if !image_uri.is_empty() {
                    slog::info!(
                        logger,
                        "Updating Lambda with container image: {}",
                        image_uri
                    );
                    executor
                        .execute_chain(chain.lambda_update_container_chain(
                            function_name,
                            image_uri,
                            publish,
                        ))
                        .await
                } else {
                    if let Ok(zip_file) = aws_config.get_lambda_zip() {
                        if zip_file.to_string_lossy() != "" {
                            slog::info!(logger, "Updating Lambda with ZIP file: {:?}", zip_file);
                            executor
                                .execute_chain(chain.lambda_update_zip_chain(
                                    function_name,
                                    zip_file,
                                    publish,
                                ))
                                .await
                        } else {
                            slog::error!(logger, "Neither ZIP file nor container image specified");
                            return Err("Update source not specified".into());
                        }
                    } else {
                        slog::error!(logger, "Neither ZIP file nor container image specified");
                        return Err("Update source not specified".into());
                    }
                }
            } else {
                slog::error!(logger, "Container image URI not provided");
                return Err("Container image URI not provided".into());
            }
        }
        _ => {
            slog::error!(logger, "Unknown command: {}", cmd);
            return Err(format!("Unknown command: {}", cmd).into());
        }
    };

    let status = result?;
    if status == 0 {
        slog::info!(logger, "Action {} was finished successfully", cmd);
    } else {
        slog::error!(logger, "Action {} failed with status: {}", cmd, status);
    }

    Ok(())
}

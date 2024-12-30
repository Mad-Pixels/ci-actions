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

    // Create command chain with working directory and environment variables
    let chain = CommandChain::new(cwd).with_vars(envs.as_map());

    slog::info!(logger, "Starting AWS command chain");
    let commands = chain.sync_chain();

    let result = executor.execute_chain(commands).await?;
    if result == 0 {
        slog::info!(logger, "Action {} was finished successfully", cmd);
    } else {
        slog::error!(logger, "Action {} failed with status: {}", cmd, result);
    }
    Ok(())
}

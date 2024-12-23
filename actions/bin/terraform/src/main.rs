use processor::{MaskerEqual, MaskerRegex, ProcessorCollection, ProcessorItem};
use terraform::{executor::TerraformExecutor, TerraformConfig, TerraformEnv};
use config::MainConfig;

use provider::auto_detect;
use util::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let main_config = MainConfig::new();
    let tf_config = TerraformConfig::new();
    
    let level = main_config.get_log_level().unwrap_or("info".to_string());
    let logger = init_logger(&level);

    let provider = match auto_detect() {
        Ok(provider) => provider,
        Err(e) => {
            slog::error!(logger, "Failed to detect provider"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    slog::info!(logger, "Initialize action with provider {}", provider.name());

    let cwd = match main_config.get_working_dir() {
        Ok(v) => v,
        Err(e) => {
            slog::error!(logger, "Work directory not set"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    slog::info!(logger, "Workdir: {:?}", cwd);

    let cmd = match tf_config.get_cmd() {
        Ok(v) => v,
        Err(e) => {
            slog::error!(logger, "Invalid terraform command"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    slog::info!(logger, "Action command: {}", cmd);

    let mask = match main_config.get_mask() {
        Ok(v) => v,
        Err(e) => {
            slog::error!(logger, "Mask string not set"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    slog::debug!(logger, "mask string: {}", mask);

    let bin = match tf_config.get_bin() {
        Ok(v) => v,
        Err(e) => {
            slog::error!(logger, "Invalid terraform bin filepath"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    slog::debug!(logger, "terraform binary file: {:?}", bin);

    let output = match tf_config.get_output_file() {
        Ok(v) => v,
        Err(e) => {
            slog::error!(logger, "Invalid terraform output filepath"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    slog::debug!(logger, "terraform result output filepath: {:?}", output);

    let envs = TerraformEnv::new();
    let masker_provider_output = match MaskerRegex::new(provider.get_predefined_masked_objects(), &mask) {
        Ok(v) => v,
        Err(e) => {
            slog::error!(logger, "Failed to initialize maskers for provider"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    let masker_provider_credentials = MaskerEqual::new(provider.values(), &mask);
    let masker_terraform_envs = MaskerEqual::new(envs.values(), &mask);

    let processors = ProcessorCollection::new(vec![
        ProcessorItem::Regex(masker_provider_output),
        ProcessorItem::Equal(masker_provider_credentials),
        ProcessorItem::Equal(masker_terraform_envs),
    ]);
    slog::info!(logger, "Action was initialized");

    let result = TerraformExecutor::new(processors, bin).plan(cwd, envs.as_map().clone(), Some(output)).await?;
    if result == 0 {
        slog::info!(logger, "Action {} was finished", cmd);
    } else {
        slog::error!(logger, "Action {} was failed, status: {}", cmd, result);
    }
    Ok(())
}

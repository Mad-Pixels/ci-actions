use processor::{MaskerEqual, MaskerRegex, ProcessorCollection, ProcessorItem};
use terraform::{executor::TerraformExecutor, TerraformConfig, TerraformEnv, CommandChain};
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
        Ok(v) => {
            slog::info!(logger, "Initialize action with provider {}", v.name());
            v
        },
        Err(e) => {
            slog::error!(logger, "Failed to detect provider"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    

    let cwd = match main_config.get_working_dir() {
        Ok(v) => {
            slog::info!(logger, "Workdir: {:?}", v);
            v
        },
        Err(e) => {
            slog::error!(logger, "Work directory not set"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    
    let cmd = match tf_config.get_cmd() {
        Ok(v) => {
            slog::info!(logger, "Action command: {}", v);
            v
        },
        Err(e) => {
            slog::error!(logger, "Invalid terraform command"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    
    let workspace: Option<String> = match tf_config.get_workspace() {
        Ok(v) => {
            if v.is_empty() {
                None
            } else {
                slog::info!(logger, "Terraform workspace: {}", v);
                Some(v)
            }
        },
        Err(e) => {
            slog::error!(logger, "Failed to get terraform workspace"; "error" => e.to_string());
            return Err(e.into());
        }
     };


    let mask = match main_config.get_mask() {
        Ok(v) => {
            slog::debug!(logger, "mask string: {}", v);
            v
        },
        Err(e) => {
            slog::error!(logger, "Mask string not set"; "error" => e.to_string());
            return Err(e.into());
        }
    };

    let bin = match tf_config.get_bin() {
        Ok(v) => {
            slog::debug!(logger, "terraform binary file: {:?}", v);
            v
        },
        Err(e) => {
            slog::error!(logger, "Invalid terraform bin filepath"; "error" => e.to_string());
            return Err(e.into());
        }
    };

    let output = match tf_config.get_output_file() {
        Ok(v) => {
            slog::debug!(logger, "terraform result output filepath: {:?}", v);
            v
        },
        Err(e) => {
            slog::error!(logger, "Invalid terraform output filepath"; "error" => e.to_string());
            return Err(e.into());
        }
    };

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

    let executor = TerraformExecutor::new(processors, bin);

    // Создаем цепочку команд
    let chain = CommandChain::new(cwd)
        .with_vars(envs.as_map().clone())
        .with_workspace(workspace)  // опционально
        .with_out(Some(output));

    // Получаем все команды, которые будут выполнены
    let commands = chain.plan_chain();

    slog::info!(logger, "Starting terraform plan chain"; "steps" => commands.len());

    let result = executor.execute_chain(commands).await?;

    if result == 0 {
        slog::info!(logger, "Action {} was finished successfully", cmd);
    } else {
        slog::error!(logger, "Action {} failed with status: {}", cmd, result);
    }
    Ok(())
}

use processor::{MaskerEqual, MaskerRegex, ProcessorCollection, ProcessorItem};
use terraform::{executor::TerraformExecutor, TerraformConfig};
use std::collections::HashMap;
use config::GlobalConfig;

use provider::auto_detect;
use util::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GlobalConfig::new();
    let terraform_config = TerraformConfig::new();
    
    
    let level = config.get_log_level().unwrap_or("info".to_string());
    let logger = init_logger(&level);

    let provider = match auto_detect() {
        Ok(provider) => provider,
        Err(e) => {
            slog::error!(logger, "Failed to detect provider"; "error" => e.to_string());
            return Err(e.into());
        }
    };
    slog::info!(logger, "Initialize action with provider {}", provider.name());

    // let cmd = match config.get_cmd() {
    //     Ok(v) => v,
    //     Err(e) => {
    //         slog::error!(&logger, "Terraform command not set"; "error" => e.to_string()); 
    //         return Err(e.into()); 
    //     }
    // };
    
    




    let output = terraform_config.get_output_file().unwrap();
    let bin = terraform_config.get_bin().unwrap();
    let cwd = config.get_working_dir().unwrap();
    let mask = config.get_mask().unwrap();
    let cmd = terraform_config.get_cmd().unwrap();
    slog::debug!(
        logger,
        "config bin: {:?}, cwd: {:?}, cmd: {}, mask: {}, output: {:?}", bin, cwd, cmd, mask, output,
    );
    

    let mask_provider = match MaskerRegex::new(provider.get_predefined_masked_objects(), &mask) {
        Ok(masker) => masker,
        Err(e) => {
            slog::error!(logger, "Failed initialize regex maskers"; "error" => e.to_string());
            std::process::exit(1);
        }
    };
    let mask_creds = MaskerEqual::new(provider.values(), &mask);

    let processors = ProcessorCollection::new(vec![
        // ProcessorItem::Regex(mask_provider),
        // ProcessorItem::Equal(mask_creds),
    ]);

    slog::info!(logger, "action was initialized");
    let executor = TerraformExecutor::new(processors, bin);

    let mut vars = HashMap::new();
    
    print!("{:?}", cwd);
    let result = executor.plan(cwd, vars, Some(output)).await?;
    if result == 0 {
        slog::info!(logger, "Plan was finished");
    } else {
        slog::error!(logger, "Plan was failed, status: {}", result);
    }

    Ok(())
}

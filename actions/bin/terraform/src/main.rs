use config::Config;
use processor::{MaskerEqual, MaskerRegex, ProcessorCollection, ProcessorItem};
use std::collections::HashMap;
use terraform::executor::TerraformExecutor;

use provider::auto_detect;
use util::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    
    let level = config.get_log_level().unwrap_or("info".to_string());
    let logger = init_logger("info");

    
    let provider = match auto_detect() {
        Ok(provider) => provider,
        Err(e) => {
    // или более структурированный вариант:
    slog::error!(logger, "Failed to detect provider: {}", e);
    let error_str = e.to_string();
    slog::error!(logger, "Failed to detect provider"; 
        "error" => e.to_string()
    );
    

    std::process::exit(1);
            
        }
    };
    slog::debug!(logger, "Initialize action with {}", provider.name());

    let output = config.get_terraform_output().unwrap();
    let bin = config.get_terraform_bin().unwrap();
    let cwd = config.get_working_dir().unwrap();
    let mask = config.get_mask().unwrap();
    let cmd = config.get_cmd().unwrap();
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
        ProcessorItem::Regex(mask_provider),
        ProcessorItem::Equal(mask_creds),
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

use processor::{MaskerEqual, MaskerRegex, ProcessorCollection, ProcessorItem};
use terraform::executor::TerraformExecutor;
use slog::{error, info, debug};
use std::collections::HashMap;
use config::Config;

use provider::auto_detect;
use util::init_logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    let level = config.get_log_level().unwrap_or("info".to_string());
    let logger = init_logger(&level);

    let provider = match auto_detect() {
        Ok(provider) => provider,
        Err(e) => {
            error!(logger, "Failed to detect provider"; "error" => format!("{}", e));
            std::process::exit(1);
        }
    };
    debug!(logger, "Initialize action with {}", provider.name());
    
    let output = config.get_terraform_output().unwrap();
    let bin = config.get_terraform_bin().unwrap();
    let cwd = config.get_working_dir().unwrap();
    let mask = config.get_mask().unwrap();
    let cmd = config.get_cmd().unwrap();
    debug!(
        logger, 
        "config bin: {:?}, cwd: {:?}, cmd: {}, mask: {}, output: {:?}", 
        bin, 
        cwd, 
        cmd, 
        mask,
        output,
    );

    let mask_provider = match MaskerRegex::new(
        provider.get_predefined_masked_objects(), 
        &mask,
    ) {
        Ok(masker) => masker,
        Err(e) => {
            error!(logger, "Failed initialize regex maskers"; "error" => e.to_string());
            std::process::exit(1);
        }
    };
    let mask_creds = MaskerEqual::new(
        provider.values(), 
        &mask
    );

    let processors = ProcessorCollection::new(vec![
        ProcessorItem::Regex(mask_provider),
        ProcessorItem::Equal(mask_creds),
    ]);

    info!(logger, "action was initialized");
    let executor = TerraformExecutor::new(processors, bin);
    let result = executor.plan(cwd, HashMap::new(), Some(output)).await?;





    Ok(())
}
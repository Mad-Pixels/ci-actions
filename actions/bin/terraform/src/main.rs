use config::Config;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();

    
    
    let tf_bin = config.get_terraform_bin();
    print!("{:?}", tf_bin);

    Ok(())
}
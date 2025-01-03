use shared::source::{EnvSource, FileFormat, FileSource, Source};

fn main() {
    let env_source = EnvSource::new("APP_");

    let env2_source = EnvSource::new("APP2_")
        .with_sensitive_keys(vec!["PASSWORD".into(), "API_KEY".into()]);

    
    let file_source = FileSource::new(
        "config.json",
        FileFormat::Json
    );

    let env_values = match env_source.load() {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e.to_string());
            return
        }
    };
    

    let api_key = match env_source.get("API_KEY") {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e.to_string());
            return
        }
    };

    let api_key = match env_source.get("API_KEY") {
        Ok(v) => {
            print!("{:?}", v);
            return
        }
        Err(e) => {
            println!("{}", e.to_string());
            return
        }
    };
}

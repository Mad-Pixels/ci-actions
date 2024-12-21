use std::path::PathBuf;
use processor::{Collection, Item, maskers::regex::MaskerRegex, maskers::equal::MaskerEqual};
use provider::{AWSProvider, Provider};
use slog::{Logger, Drain, o};
use std::collections::HashMap;

use terraform::executor::TerraformExecutor;




fn setup_logger() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    Logger::root(drain, o!())
}

// fn create_processor() -> Collection {
//     Collection::new(processors)
// }

fn setup_aws_credentials() -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert("AWS_ACCESS_KEY_ID".to_string(), "test-key".to_string());
    env.insert("AWS_SECRET_ACCESS_KEY".to_string(), "test-secret".to_string());
    env
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = setup_aws_credentials();
    let provider = AWSProvider::new(env.clone());


    let regexp_processor = MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    let equal_processor = MaskerEqual::new(vec!["password", "key"], "***");

    let processors = vec![
            Item::Regex(regexp_processor),
            Item::Equal(equal_processor),
        ];

//provider.get_predefined_masked_objects()
    let logger = setup_logger();
    let processor = Collection::new(processors);

    // Путь к terraform может быть получен из переменных окружения или конфигурации
    let terraform_path = PathBuf::from("/usr/local/bin/terraform");

    let executor = TerraformExecutor::new(
        processor,
        logger.clone(),
        terraform_path,
    );

    // Директория с terraform конфигурацией
    let tf_dir = PathBuf::from("/Users/igoss/Desktop/person/terraform/provisioners/infra");

    // Переменные для terraform
    let mut vars = HashMap::new();


    // Запускаем plan
    let plan_result = executor.plan(
        tf_dir,
        vars,
        Some(PathBuf::from("/tmp/1terraform.plan")), // Опционально сохраняем план в файл
    ).await?;

    if plan_result == 0 {
        println!("Terraform plan completed successfully!");
    } else {
        eprintln!("Terraform plan failed with exit code: {}", plan_result);
    }

    Ok(())
}
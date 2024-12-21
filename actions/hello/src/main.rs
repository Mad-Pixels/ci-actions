use processor::{maskers::MaskerEqual, maskers::MaskerRegex, MaskerCollection, MaskerItem};
use provider::{AWSProvider, Provider};
use slog::{o, Drain, Logger};
use std::collections::HashMap;
use std::path::PathBuf;

use terraform::executor::TerraformExecutor;

// fn create_processor() -> Collection {
//     Collection::new(processors)
// }

fn setup_aws_credentials() -> HashMap<String, String> {
    let mut env = HashMap::new();
    env.insert("AWS_ACCESS_KEY_ID".to_string(), "test-key".to_string());
    env.insert(
        "AWS_SECRET_ACCESS_KEY".to_string(),
        "test-secret".to_string(),
    );
    env
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = setup_aws_credentials();
    let provider = AWSProvider::new(env.clone());

    let regexp_processor =
        MaskerRegex::new(provider.get_predefined_masked_objects(), "****").unwrap();
    let equal_processor = MaskerEqual::new(vec!["password", "key"], "***");

    let processors = vec![
        MaskerItem::Regex(regexp_processor),
        MaskerItem::Equal(equal_processor),
    ];

    //provider.get_predefined_masked_objects()
    let processor = MaskerCollection::new(processors);

    // Путь к terraform может быть получен из переменных окружения или конфигурации
    let terraform_path = PathBuf::from("/usr/local/bin/terraform");

    let executor = TerraformExecutor::new(processor, terraform_path);

    // Директория с terraform конфигурацией
    let tf_dir = PathBuf::from("/Users/igoss/Desktop/person/terraform/provisioners/infra");

    // Переменные для terraform
    let mut vars = HashMap::new();


    // Запускаем plan
    let plan_result = executor
        .plan(
            tf_dir,
            vars,
            Some(PathBuf::from("/tmp/1terraform.plan")), // Опционально сохраняем план в файл
        )
        .await?;

    if plan_result == 0 {
        println!("Terraform plan completed successfully!");
    } else {
        eprintln!("Terraform plan failed with exit code: {}", plan_result);
    }

    Ok(())
}

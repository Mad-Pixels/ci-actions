use std::collections::HashMap;
use slog::{Logger, Drain, o};
use processor::{Collection, Item, masker_regex::MaskerRegex};
use executer::{SubprocessExecuter, IsolateExecuter, CommandExecuter};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Настраиваем логгер
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = Logger::root(drain, o!());

    // Создаем маскировщик для конфиденциальных данных
    let masker = MaskerRegex::new(
        vec![
            r"\d{4}-\d{4}-\d{4}-\d{4}", // Номера кредитных карт
            r"password=\w+",             // Пароли
            r"secret=\w+",              // Секреты
            r"token=\w+",               // Токены
        ],
        "****"
    );
    let collection = Collection::new(vec![Item::Regex(masker)]);

    // Пример 1: Простой SubprocessExecuter
    println!("\n=== Simple SubprocessExecuter Example ===");
    let executer = SubprocessExecuter::new(Some(collection.clone()), logger.clone());
    
    let mut stream = executer.execute_stream(
        vec!["echo".to_string(), "My credit card: 1234-5678-9012-3456 and password=secret123".to_string()],
        None,
        None,
        true, // Включаем маскирование
    ).await?;

    while let Some(line) = stream.next().await {
        match line {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    // Пример 2: IsolateExecuter с изолированным окружением
    println!("\n=== IsolateExecuter with Environment Example ===");
    let mut isolated_env = HashMap::new();
    isolated_env.insert("SECRET_TOKEN".to_string(), "token=very_secret_token".to_string());
    
    let isolated_executer = IsolateExecuter::new(
        Some(collection),
        logger,
        isolated_env,
    );

    // Выполняем команду в изолированном окружении
    let mut stream = isolated_executer.execute_stream(
        vec!["sh".to_string(), "-c".to_string(), "echo $SECRET_TOKEN".to_string()],
        None,
        None,
        true, // Включаем маскирование
    ).await?;

    while let Some(line) = stream.next().await {
        match line {
            Ok(output) => println!("{}", output),
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    Ok(())
}
use super::context::Context;
use crate::error::{ExecuterResult, ExecuterError};
use crate::validate::Validator;
use crate::output::Output;

use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::time::{timeout, Duration};
use tokio::process::Command;

use std::process::Stdio;
use std::sync::Arc;

pub struct Subprocess {
    stdout: Arc<Output>,
    stderr: Arc<Output>,
    validator: Validator,
}

impl Subprocess {
    pub fn new(output: Output, validator: Validator) -> Self {
        let stderr = Arc::new(output.clone());
        let stdout = Arc::new(output);
        Self { stdout, stderr, validator}
    }

    pub async fn execute(&self, context: Context) -> ExecuterResult<i32> {
        self.validator.validate(&context)?;

        let mut command = Command::new(&context.command[0]);
        command
            .args(&context.command[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        if let Some(path) = &context.cwd {
            command.current_dir(path);
        }
        let mut child = command.spawn()?;

        let stdout = child.stdout.take()
            .ok_or_else(|| ExecuterError::ExecutionError("Failed to capture stdout".to_string()))?;
        let stderr = child.stderr.take()
            .ok_or_else(|| ExecuterError::ExecutionError("Failed to capture stderr".to_string()))?;

        let stdout_output = Arc::clone(&self.stdout);
        let stderr_output = Arc::clone(&self.stderr);

        let stdout_handle = tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                stdout_output.write(&line);
            }
        });
        let stderr_handle = tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                stderr_output.write_error(&line);
            }
        });

        let status = if let Some(t) = context.timeout {
            match timeout(Duration::from_secs(t), child.wait()).await {
                Ok(status) => status?,
                Err(_) => {
                    child.kill().await?;
                    return Err(ExecuterError::ExecutionError(
                        format!("Command timed out after {} seconds", t)
                    ));
                }
            }
        } else {
            child.wait().await?
        };
        stdout_handle.await.map_err(|e| 
            ExecuterError::ExecutionError(format!("Failed to process stdout: {}", e))
        )?;
        stderr_handle.await.map_err(|e| 
            ExecuterError::ExecutionError(format!("Failed to process stderr: {}", e))
        )?;
        Ok(status.code().unwrap_or(2))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::output::Target;
    use processor::{Collection, Item, maskers::regex::MaskerRegex};
    use slog::{Logger, Drain, o};
    use tempfile::tempdir;
    use std::fs;
    use std::collections::HashMap;
    use std::path::PathBuf;

    // Предполагается, что Validator имеет метод default
    use crate::validate::Validator;

    fn setup_logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        Logger::root(drain, o!())
    }

    fn create_processor() -> Collection {
        let masker = MaskerRegex::new(vec![
            r"password=\w+",
            r"secret=\w+",
            r"token=\w+"
        ], "****").unwrap();
        Collection::new(vec![Item::Regex(masker)])
    }

    #[cfg(unix)]
    fn get_test_command_base() -> Vec<String> {
        vec!["bash".to_string(), "-c".to_string()]
    }

    #[cfg(windows)]
    fn get_test_command_base() -> Vec<String> {
        vec!["cmd".to_string(), "/C".to_string()]
    }

    fn build_command(cmd: &str) -> Vec<String> {
        let mut base = get_test_command_base();
        base.push(cmd.to_string());
        base
    }

    #[tokio::test]
    async fn test_basic_echo() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        let context = Context::new(
            build_command("echo hello"),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context)
            .await
            .expect("Failed to execute echo command");

        assert_eq!(status, 0);
        let content = fs::read_to_string(&output_path).expect("Failed to read output file");
        assert!(content.contains("hello"));
    }

    #[tokio::test]
    async fn test_sensitive_data_masking() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");
        let error_path = temp_dir.path().join("error.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(error_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        #[cfg(unix)]
        let cmd = "echo 'password=secret123' && echo 'token=abc123' 1>&2";
        #[cfg(windows)]
        let cmd = "cmd /C \"echo password=secret123 && echo token=abc123 1>&2\"";

        let context = Context::new(
            build_command(cmd),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context)
            .await
            .expect("Failed to execute command with sensitive data");

        assert_eq!(status, 0);
        let content = fs::read_to_string(&output_path).expect("Failed to read output file");
        let error_content = fs::read_to_string(&error_path).expect("Failed to read error file");

        assert!(!content.contains("secret123"));
        assert!(!error_content.contains("abc123"));
        assert!(content.contains("****"));
        assert!(error_content.contains("****"));
    }

    #[tokio::test]
    async fn test_stderr_output() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");
        let error_path = temp_dir.path().join("error.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(error_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        #[cfg(unix)]
        let cmd = "echo 'stdout message' && echo 'password=secret' >&2";
        #[cfg(windows)]
        let cmd = "cmd /C \"echo stdout message && echo password=secret 1>&2\"";

        let context = Context::new(
            build_command(cmd),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context)
            .await
            .expect("Failed to execute command with stderr");

        assert_eq!(status, 0);
        let output_content = fs::read_to_string(&output_path).expect("Failed to read output file");
        let error_content = fs::read_to_string(&error_path).expect("Failed to read error file");

        assert!(output_content.contains("stdout message"));
        assert!(!error_content.contains("secret"));
        assert!(error_content.contains("****"));
    }

    #[tokio::test]
    async fn test_exit_codes() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        // Успешное выполнение
        #[cfg(unix)]
        let cmd_success = "true";
        #[cfg(windows)]
        let cmd_success = "cmd /C \"exit /B 0\"";

        let context_success = Context::new(
            build_command(cmd_success),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context_success)
            .await
            .expect("Failed to execute success command");
        assert_eq!(status, 0);

        // Ошибка выполнения
        #[cfg(unix)]
        let cmd_error = "exit 1";
        #[cfg(windows)]
        let cmd_error = "cmd /C \"exit /B 1\"";

        let context_error = Context::new(
            build_command(cmd_error),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context_error)
            .await
            .expect("Failed to execute error command");
        assert_eq!(status, 1);
    }

    #[tokio::test]
    async fn test_sequential_commands() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        #[cfg(unix)]
        let cmd = "for i in 1 2 3; do echo \"Step $i\"; sleep 0.1; done";
        #[cfg(windows)]
        let cmd = "cmd /C \"for /L %i in (1,1,3) do @(echo Step %i && timeout /t 1 /nobreak > nul)\"";

        let context = Context::new(
            build_command(cmd),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context)
            .await
            .expect("Failed to execute sequential commands");
        assert_eq!(status, 0);
        let content = fs::read_to_string(&output_path).expect("Failed to read output file");

        assert!(content.contains("Step 1"));
        assert!(content.contains("Step 2"));
        assert!(content.contains("Step 3"));
    }

    #[tokio::test]
    async fn test_large_output() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        #[cfg(unix)]
        let cmd = "seq 1 100 | xargs -I{} echo \"password=secret{}\"";
        #[cfg(windows)]
        let cmd = "powershell -Command \"1..100 | ForEach-Object { echo \\\"password=secret$_\\\" }\"";

        let context = Context::new(
            build_command(cmd),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context)
            .await
            .expect("Failed to execute command with large output");
        assert_eq!(status, 0);
        let content = fs::read_to_string(&output_path).expect("Failed to read output file");

        // Проверяем, что все 100 паролей замаскированы
        assert_eq!(content.matches("****").count(), 100);
    }

    #[tokio::test]
    async fn test_nonexistent_command() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        let context = Context::new(
            vec!["nonexistentcommand".to_string()],
            HashMap::new(),
            None,
        );

        let result = subprocess.execute(context)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_concurrent_output() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");
        let error_path = temp_dir.path().join("error.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(error_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        #[cfg(unix)]
        let cmd = "echo 'password=secret1' && echo 'password=secret2' >&2";
        #[cfg(windows)]
        let cmd = "cmd /C \"echo password=secret1 && echo password=secret2 1>&2\"";

        let context = Context::new(
            build_command(cmd),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context)
            .await
            .expect("Failed to execute concurrent commands");

        assert_eq!(status, 0);
        let output_content = fs::read_to_string(&output_path).expect("Failed to read output file");
        let error_content = fs::read_to_string(&error_path).expect("Failed to read error file");

        assert!(!output_content.contains("secret1"));
        assert!(!error_content.contains("secret2"));
        assert!(output_content.contains("****"));
        assert!(error_content.contains("****"));
    }

    #[tokio::test]
    async fn test_empty_output() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        #[cfg(unix)]
        let cmd = "true";
        #[cfg(windows)]
        let cmd = "cmd /C \"exit /B 0\"";

        let context = Context::new(
            build_command(cmd),
            HashMap::new(),
            None,
        );

        let status = subprocess.execute(context)
            .await
            .expect("Failed to execute empty command");

        assert_eq!(status, 0);
        if output_path.exists() {
            let content = fs::read_to_string(&output_path).expect("Failed to read output file");
            assert!(content.trim().is_empty());
        } else {
            // Если файл не существует, убедитесь, что выполнение прошло успешно
            // Это зависит от реализации Output
            // Можно добавить дополнительные проверки при необходимости
        }
    }

    // Дополнительные тесты, связанные с таймаутом и рабочей директорией

    #[tokio::test]
    async fn test_command_timeout() {
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let output_path = temp_dir.path().join("output.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );

        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);

        #[cfg(unix)]
        let cmd = "sleep 5";
        #[cfg(windows)]
        let cmd = "cmd /C \"timeout /t 5 /nobreak > nul\"";

        let mut context = Context::new(
            build_command(cmd),
            HashMap::new(),
            None,
        ).with_timeout(1); // Устанавливаем таймаут в 1 секунду

        let result = subprocess.execute(context)
            .await;

        assert!(result.is_err());
        if let Err(e) = result {
            match e {
                ExecuterError::ExecutionError(msg) => {
                    assert!(msg.contains("timed out"));
                },
                _ => panic!("Unexpected error type"),
            }
        }
    }

    #[tokio::test]
    async fn test_working_directory() {
        // Создаём временную директорию
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let nested_dir = temp_dir.path().join("nested");
        fs::create_dir(&nested_dir).expect("Failed to create nested directory");
        let output_path = temp_dir.path().join("output.log");
    
        // Инициализируем Output
        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );
    
        // Создаём Validator и Subprocess
        let validator = Validator::default();
        let subprocess = Subprocess::new(output, validator);
    
        // Определяем команду для разных платформ
        #[cfg(unix)]
        let cmd = "pwd";
        #[cfg(windows)]
        let cmd = "cmd /C \"cd\"";
    
        // Создаём контекст выполнения команды с установленной рабочей директорией
        let context = Context::new(
            build_command(cmd),
            HashMap::new(),
            Some(nested_dir.clone()),
        );
    
        // Выполняем команду
        let status = subprocess.execute(context)
            .await
            .expect("Failed to execute command with working directory");
    
        // Проверяем код возврата
        assert_eq!(status, 0);
    
        // Читаем содержимое файла вывода
        let content = fs::read_to_string(&output_path).expect("Failed to read output file");
    
        // Получаем канонические пути для сравнения
        let expected = nested_dir.canonicalize().expect("Failed to canonicalize nested_dir");
        let actual = PathBuf::from(content.trim()).canonicalize().expect("Failed to canonicalize actual path");
    
        // Сравниваем канонические пути
        assert_eq!(actual, expected, "The working directory does not match the expected path");
    }
    
}

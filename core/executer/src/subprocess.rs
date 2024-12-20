use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::Command;
use crate::output::Output;
use std::process::Stdio;
use std::sync::Arc;

pub struct Subprocess {
    stdout: Arc<Output>,
    stderr: Arc<Output>,
}

impl Subprocess {
    pub fn new(output: Output) -> Self {
        let stderr = Arc::new(output.clone());
        let stdout = Arc::new(output);
        Self { stdout, stderr }
    }

    pub async fn execute(&self, cmd: Vec<String>) -> Result<i32, std::io::Error> {
        let mut command = Command::new(&cmd[0]);
        command
            .args(&cmd[1..])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let mut child = command.spawn()?;

        let stdout = child.stdout.take()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stdout"))?;
        let stderr = child.stderr.take()
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Failed to capture stderr"))?;

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
        let status = child.wait().await?;

        stdout_handle.await.unwrap_or_else(|e| {
            eprintln!("Failed to process stdout: {:?}", e);
        });
        stderr_handle.await.unwrap_or_else(|e| {
            eprintln!("Failed to process stderr: {:?}", e);
        });
        Ok(status.code().unwrap_or(-1))
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

        let subprocess = Subprocess::new(output);
        let status = subprocess.execute(build_command("echo hello"))
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

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(output_path.clone()),
            setup_logger(),
        );

        let subprocess = Subprocess::new(output);
        #[cfg(unix)]
        let cmd = "echo 'password=secret123' && echo 'token=abc123'";
        #[cfg(windows)]
        let cmd = "echo password=secret123 && echo token=abc123 1>&2";

        let status = subprocess.execute(build_command(cmd))
            .await
            .expect("Failed to execute command with sensitive data");

        assert_eq!(status, 0);
        let content = fs::read_to_string(&output_path).expect("Failed to read output file");
        assert!(!content.contains("secret123"));
        assert!(!content.contains("abc123"));
        assert!(content.contains("****"));
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
        let subprocess = Subprocess::new(output);

        #[cfg(unix)]
        let cmd = "echo 'stdout message' && echo 'password=secret' >&2";
        #[cfg(windows)]
        let cmd = "echo stdout message && echo password=secret 1>&2";

        let status = subprocess.execute(build_command(cmd))
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
        let subprocess = Subprocess::new(output);

        #[cfg(unix)]
        let cmd_success = "true";
        #[cfg(windows)]
        let cmd_success = "exit /B 0";

        let status = subprocess.execute(build_command(cmd_success))
            .await
            .expect("Failed to execute success command");
        assert_eq!(status, 0);

        #[cfg(unix)]
        let cmd_error = "exit 1";
        #[cfg(windows)]
        let cmd_error = "exit /B 1";

        let status = subprocess.execute(build_command(cmd_error))
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
        let subprocess = Subprocess::new(output);

        #[cfg(unix)]
        let cmd = "for i in 1 2 3; do echo \"Step $i\"; sleep 0.1; done";
        #[cfg(windows)]
        let cmd = "for /L %i in (1,1,3) do @(echo Step %i && timeout /t 1 /nobreak > nul)";

        let status = subprocess.execute(build_command(cmd))
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
        let subprocess = Subprocess::new(output);

        #[cfg(unix)]
        let cmd = "seq 1 100 | xargs -I{} echo \"password=secret{}\"";
        #[cfg(windows)]
        let cmd = "powershell -Command \"1..100 | ForEach-Object { echo \\\"password=secret$_\\\" }\"";

        let status = subprocess.execute(build_command(cmd))
            .await
            .expect("Failed to execute command with large output");

        assert_eq!(status, 0);
        let content = fs::read_to_string(&output_path).expect("Failed to read output file");
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

        let subprocess = Subprocess::new(output);
        let result = subprocess.execute(vec!["nonexistentcommand".to_string()])
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

        let subprocess = Subprocess::new(output);

        #[cfg(unix)]
        let cmd = "echo 'password=secret1' & echo 'password=secret2' >&2";
        #[cfg(windows)]
        let cmd = "cmd /C \"echo password=secret1 && echo password=secret2 1>&2\"";

        let status = subprocess.execute(build_command(cmd))
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

        let subprocess = Subprocess::new(output);

        #[cfg(unix)]
        let cmd = "true";
        #[cfg(windows)]
        let cmd = "cmd /C \"exit /B 0\"";
        let status = subprocess.execute(build_command(cmd))
            .await
            .expect("Failed to execute empty command");

        assert_eq!(status, 0);
        if output_path.exists() {
            let content = fs::read_to_string(&output_path).expect("Failed to read output file");
            assert!(content.trim().is_empty());
        }
    }
}

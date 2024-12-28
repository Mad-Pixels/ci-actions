use super::formatter;
use super::types::Target;
use formatter::PlainFormatter;
use slog::{o, Drain, Logger};
use std::fs::OpenOptions;
use std::io::Write;

/// Handles writing log messages to different targets.
#[derive(Clone)]
pub(crate) struct Writer {
    logger: Logger,
}

impl Writer {
    /// Creates a new `Writer` instance.
    pub fn new() -> Self {
        let drain = slog_async::Async::new(PlainFormatter.fuse()).build().fuse();
        Self {
            logger: Logger::root(drain, o!()),
        }
    }

    /// Writes a log message to the specified target.
    ///
    /// # Arguments
    ///
    /// * `line` - The log message to be written.
    /// * `target` - The target where the message should be written.
    pub fn write(&self, line: &str, target: &Target) {
        eprintln!("DEBUG - Raw input line length: {}", line.len());
        eprintln!("DEBUG - First 50 chars: {}", &line[..50.min(line.len())]);
        let safe_line = if !line.starts_with("::log::") {
            format!("::log::{}", line)
        } else {
            line.to_string()
        };

        match target {
            Target::Stdout => slog::info!(self.logger, "{}", safe_line),
            Target::Stderr => {
                if line.contains("Error:") || line.contains("error:") {
                    slog::error!(self.logger, "{}", safe_line);
                } else if line.contains("Warning:") || line.contains("warning:") {
                    slog::warn!(self.logger, "{}", safe_line);
                } else {
                    slog::info!(self.logger, "{}", safe_line);
                }
            }
            Target::File(path) => {
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .expect("Failed to open output file");

                writeln!(file, "{}", line).expect("Failed to write to file");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_file_writer() {
        let writer = Writer::new();
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.log");

        writer.write("test line", &Target::File(file_path.clone()));

        let content = std::fs::read_to_string(file_path).unwrap();
        assert_eq!(content.trim(), "test line");
    }
}

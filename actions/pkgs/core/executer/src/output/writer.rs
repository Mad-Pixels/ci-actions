use super::types::Target;

use std::fs::OpenOptions;
use std::io::Write;

/// Handles writing log messages to different targets.
#[derive(Clone)]
pub(crate) struct Writer;

impl Writer {
    /// Creates a new `Writer` instance.
    pub fn new() -> Self {
        Self
    }

    /// Writes a log message to the specified target.
    ///
    /// # Arguments
    ///
    /// * `line` - The log message to be written.
    /// * `target` - The target where the message should be written.
    pub fn write(&self, line: &str, target: &Target) {
        match target {
            Target::Stdout => println!("{}", line),
            Target::Stderr => eprintln!("{}", line),
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
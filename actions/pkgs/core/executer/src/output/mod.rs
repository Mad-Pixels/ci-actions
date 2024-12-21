mod formatter;
mod types;
mod writer;

pub use types::Target;

use formatter::PlainFormatter;
use processor::{MaskerCollection, Processor};
use slog::{o, Drain, Logger};
use writer::Writer;

/// Represents an output handler that processes and routes log messages.
///
/// The `Output` struct handles logging messages by processing them through
/// a `Collection` of processors and directing them to specified targets
/// such as files or standard output/error streams.
#[derive(Clone)]
pub struct Output {
    processor: MaskerCollection,
    output_target: Target,
    error_target: Target,
    logger: Logger,
    writer: Writer,
}

impl Output {
    /// Creates a new `Output` instance.
    ///
    /// # Arguments
    ///
    /// * `processor` - A collection of processors to handle log message processing.
    /// * `output_target` - The target where standard log messages will be written.
    /// * `error_target` - The target where error log messages will be written.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{maskers::MaskerRegex, MaskerCollection, MaskerItem};
    /// use executer::{Output, Target};
    /// use slog::{Logger, o};
    ///
    /// fn create_processor() -> MaskerCollection {
    ///     let masker = MaskerRegex::new(vec![r"password=\w+"], "****").unwrap();
    ///     MaskerCollection::new(vec![MaskerItem::Regex(masker)])
    /// }
    ///
    /// let output = Output::new(create_processor(), Target::Stdout, Target::Stderr);
    /// ```
    pub fn new(processor: MaskerCollection, output_target: Target, error_target: Target) -> Self {
        let drain = slog_async::Async::new(PlainFormatter.fuse()).build().fuse();

        Self {
            logger: Logger::root(drain, o!()),
            writer: Writer::new(),
            output_target,
            error_target,
            processor,
        }
    }

    /// Writes a standard log message to the designated output target.
    ///
    /// # Arguments
    ///
    /// * `line` - The log message to be written.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{maskers::MaskerRegex, MaskerCollection, MaskerItem};
    /// use executer::{Output, Target};
    /// use slog::{Logger, o};
    ///
    /// fn create_processor() -> MaskerCollection {
    ///     let masker = MaskerRegex::new(vec![r"password=\w+"], "****").unwrap();
    ///     MaskerCollection::new(vec![MaskerItem::Regex(masker)])
    /// }
    ///
    /// let output = Output::new(create_processor(), Target::Stdout, Target::Stderr);
    /// output.write("This is an log message");
    /// ```
    pub fn write(&self, line: &str) {
        let processed = self.processor.process(line);
        slog::info!(self.logger, "{}", processed);
        self.writer.write(&processed, &self.output_target);
    }

    /// Writes an error log message to the designated error target.
    ///
    /// # Arguments
    ///
    /// * `line` - The error message to be written.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{maskers::MaskerRegex, MaskerCollection, MaskerItem};
    /// use executer::{Output, Target};
    /// use slog::{Logger, o};
    ///
    /// fn create_processor() -> MaskerCollection {
    ///     let masker = MaskerRegex::new(vec![r"password=\w+"], "****").unwrap();
    ///     MaskerCollection::new(vec![MaskerItem::Regex(masker)])
    /// }
    ///
    /// let output = Output::new(create_processor(), Target::Stdout, Target::Stderr);
    /// output.write_error("This is an error message");
    /// ```
    pub fn write_error(&self, line: &str) {
        let processed = self.processor.process(line);
        slog::error!(self.logger, "{}", processed);
        self.writer.write(&processed, &self.error_target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use processor::{maskers::MaskerRegex, MaskerCollection, MaskerItem};

    /// Creates a processor collection with a regex masker.
    fn create_processor() -> MaskerCollection {
        let masker = MaskerRegex::new(vec![r"password=\w+"], "****").unwrap();
        MaskerCollection::new(vec![MaskerItem::Regex(masker)])
    }

    #[test]
    fn test_stdout_output() {
        let output = Output::new(create_processor(), Target::Stdout, Target::Stderr);
        output.write("password=secret");
        output.write_error("error message");
    }

    #[test]
    fn test_file_output() {
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("output.log");
        let error_path = temp_dir.path().join("error.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(error_path.clone()),
        );
        output.write("password=secret");
        output.write_error("error message");

        let output_content = std::fs::read_to_string(output_path).unwrap();
        let error_content = std::fs::read_to_string(error_path).unwrap();
        assert!(output_content.contains("****"));
        assert!(error_content.contains("error message"));
    }
}

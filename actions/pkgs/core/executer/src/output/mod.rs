mod formatter;
mod types;
mod writer;

pub use types::Target;

use formatter::PlainFormatter;
use processor::{Collection, Processor};
use slog::{o, Drain, Logger};
use writer::Writer;

#[derive(Clone)]
pub struct Output {
    processor: Collection,
    output_target: Target,
    error_target: Target,
    logger: Logger,
    writer: Writer,
}

impl Output {
    pub fn new(processor: Collection, output_target: Target, error_target: Target) -> Self {
        let drain = slog_async::Async::new(PlainFormatter.fuse()).build().fuse();

        Self {
            logger: Logger::root(drain, o!()),
            writer: Writer::new(),
            output_target,
            error_target,
            processor,
        }
    }

    pub fn write(&self, line: &str) {
        let processed = self.processor.process(line);
        slog::info!(self.logger, "{}", processed);
        self.writer.write(&processed, &self.output_target);
    }

    pub fn write_error(&self, line: &str) {
        let processed = self.processor.process(line);
        slog::error!(self.logger, "{}", processed);
        self.writer.write(&processed, &self.error_target);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use processor::{maskers::regex::MaskerRegex, Collection, Item};

    fn create_processor() -> Collection {
        let masker = MaskerRegex::new(vec![r"password=\w+"], "****").unwrap();
        Collection::new(vec![Item::Regex(masker)])
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

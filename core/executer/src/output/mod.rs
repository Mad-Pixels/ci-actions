mod writer;
mod types;

pub use types::Target;

use processor::{Collection, Processor};
use writer::Writer;
use slog::Logger;

#[derive(Clone)]
pub struct Output {
    processor: Collection,
    output_target: Target,
    error_target: Target,
    logger: Logger,
    writer: Writer,
}

impl Output {
    pub fn new(
        processor: Collection,
        output_target: Target,
        error_target: Target,
        logger: Logger,
    ) -> Self {
        Self { 
            processor,
            output_target,
            error_target,
            logger,
            writer: Writer::new(),
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
    use processor::{Collection, Item, maskers::regex::MaskerRegex};
    use slog::{Logger, Drain, o};

    fn setup_logger() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let drain = slog_term::FullFormat::new(decorator).build().fuse();
        let drain = slog_async::Async::new(drain).build().fuse();
        Logger::root(drain, o!())
    }

    fn create_processor() -> Collection {
        let masker = MaskerRegex::new(vec![r"password=\w+"], "****").unwrap();
        Collection::new(vec![Item::Regex(masker)])
    }

    #[test]
    fn test_stdout_output() {
        let logger = setup_logger();
        let output = Output::new(
            create_processor(),
            Target::Stdout,
            Target::Stderr,
            logger,
        );
        output.write("password=secret");
        output.write_error("error message");
    }

    #[test]
    fn test_file_output() {
        let logger = setup_logger();
        let temp_dir = tempfile::tempdir().unwrap();
        let output_path = temp_dir.path().join("output.log");
        let error_path = temp_dir.path().join("error.log");

        let output = Output::new(
            create_processor(),
            Target::File(output_path.clone()),
            Target::File(error_path.clone()),
            logger,
        );
        output.write("password=secret");
        output.write_error("error message");

        let output_content = std::fs::read_to_string(output_path).unwrap();
        let error_content = std::fs::read_to_string(error_path).unwrap();
        assert!(output_content.contains("****"));
        assert!(error_content.contains("error message"));
    }
}
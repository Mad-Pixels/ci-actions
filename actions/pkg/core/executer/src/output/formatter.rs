use slog::{OwnedKVList, Record};
use std::io;

/// Internal Logger formatter that prints log message as a plain text.
pub struct PlainFormatter;

impl slog::Drain for PlainFormatter {
    type Err = io::Error;
    type Ok = ();

    /// Logs a record by printing its message to the standard output.
    fn log(&self, record: &Record, _values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        println!("{}", record.msg());
        Ok(())
    }
}

use slog::{OwnedKVList, Record};
use std::io;

/// Internal Logger formatter that prints log message as a plain text.
pub struct PlainFormatter;

impl slog::Drain for PlainFormatter {
    type Err = io::Error;
    type Ok = ();

    fn log(&self, record: &Record, _values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        let msg = record.msg();
        println!("{}", msg);
        Ok(())
    }
}

use slog::{OwnedKVList, Record};
use std::io;

pub struct PlainFormatter;

impl slog::Drain for PlainFormatter {
    type Err = io::Error;
    type Ok = ();

    fn log(&self, record: &Record, _values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        println!("{}", record.msg());
        Ok(())
    }
}

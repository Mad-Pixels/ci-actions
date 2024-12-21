use slog::{Drain, OwnedKVList, Record, Key, Value, KV};
use std::io;

pub struct PlainFormatter;

impl slog::Drain for PlainFormatter {
    type Ok = ();
    type Err = io::Error;

    fn log(&self, record: &Record, _values: &OwnedKVList) -> Result<Self::Ok, Self::Err> {
        println!("{}", record.msg());
        Ok(())
    }
}
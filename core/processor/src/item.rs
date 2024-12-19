use crate::maskers::{equal::MaskerEqual, regex::MaskerRegex};
use crate::Processor;

/// Available processor implementations
#[derive(Clone)]
pub enum Item {
    /// Regular expression based processor
    Regex(MaskerRegex),
    /// Exact string match processor
    Equal(MaskerEqual),
}

impl Processor for Item {
    fn process(&self, input: &str) -> String {
        match self {
            Item::Regex(processor) => processor.process(input),
            Item::Equal(processor) => processor.process(input),
        }
    }
}
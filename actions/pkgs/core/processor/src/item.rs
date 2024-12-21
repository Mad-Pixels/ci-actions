use crate::maskers::{MaskerEqual, MaskerRegex};
use crate::Processor;

/// Available processor implementations
#[derive(Clone)]
pub enum MaskerItem {
    /// Regular expression based processor
    Regex(MaskerRegex),
    /// Exact string match processor
    Equal(MaskerEqual),
}

impl Processor for MaskerItem {
    fn process(&self, input: &str) -> String {
        match self {
            MaskerItem::Regex(processor) => processor.process(input),
            MaskerItem::Equal(processor) => processor.process(input),
        }
    }
}

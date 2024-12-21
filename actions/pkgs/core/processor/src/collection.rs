use crate::{MaskerItem, Processor};

/// Collection of processors that are applied sequentially
#[derive(Clone)]
pub struct MaskerCollection {
    processors: Vec<MaskerItem>,
}

impl MaskerCollection {
    /// Create new collection of processors
    pub fn new(processors: Vec<MaskerItem>) -> Self {
        Self { processors }
    }
}

impl Processor for MaskerCollection {
    fn process(&self, input: &str) -> String {
        self.processors
            .iter()
            .fold(input.to_string(), |acc, processor| processor.process(&acc))
    }
}

use crate::{Item, Processor};

/// Collection of processors that are applied sequentially
#[derive(Clone)]
pub struct Collection {
    processors: Vec<Item>,
}

impl Collection {
    /// Create new collection of processors
    pub fn new(processors: Vec<Item>) -> Self {
        Self { processors }
    }
}

impl Processor for Collection {
    fn process(&self, input: &str) -> String {
        self.processors
            .iter()
            .fold(input.to_string(), |acc, processor| processor.process(&acc))
    }
}

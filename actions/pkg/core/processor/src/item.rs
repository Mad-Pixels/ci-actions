use crate::maskers::{MaskerEqual, MaskerRegex};
use crate::Processor;

/// Represents different types of masking processors.
#[derive(Clone)]
pub enum MaskerItem {
    /// Regular expression-based processor.
    Regex(MaskerRegex),
    /// Exact string match processor.
    Equal(MaskerEqual),
}

impl Processor for MaskerItem {
    /// Processes the input string by delegating to the specific processor variant.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to process.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{MaskerItem, MaskerEqual, Processor};
    ///
    /// let equal_processor = MaskerEqual::new(vec!["secret"], "***");
    /// let item = MaskerItem::Equal(equal_processor);
    ///
    /// let input = "This is a secret message.";
    /// let output = item.process(input);
    /// assert_eq!(output, "This is a *** message.");
    /// ```
    fn process(&self, input: &str) -> String {
        match self {
            MaskerItem::Regex(processor) => processor.process(input),
            MaskerItem::Equal(processor) => processor.process(input),
        }
    }
}

use crate::{MaskerItem, Processor};

/// Collection of processors that are applied sequentially.
///
/// The `MaskerCollection` struct manages a list of `MaskerItem` processors and
/// applies them in order to process input strings.
#[derive(Clone)]
pub struct MaskerCollection {
    /// The list of processors to apply.
    processors: Vec<MaskerItem>,
}

impl MaskerCollection {
    /// Creates a new collection of processors.
    ///
    /// # Arguments
    ///
    /// * `processors` - A vector of `MaskerItem` processors to be applied sequentially.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{MaskerCollection, MaskerItem, Processor};
    /// use processor::maskers::{MaskerEqual, MaskerRegex};
    ///
    /// let regex_processor = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****").unwrap();
    /// let equal_processor = MaskerEqual::new(vec!["password", "key"], "***");
    ///
    /// let processors = vec![
    ///     MaskerItem::Regex(regex_processor),
    ///     MaskerItem::Equal(equal_processor),
    /// ];
    /// let collection = MaskerCollection::new(processors);
    /// ```
    pub fn new(processors: Vec<MaskerItem>) -> Self {
        Self { processors }
    }
}

impl Processor for MaskerCollection {
    /// Processes the input string by applying each processor in sequence.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to process.
    ///
    /// # Returns
    ///
    /// A new `String` with all processors applied.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{MaskerCollection, MaskerItem, Processor};
    /// use processor::maskers::{MaskerEqual, MaskerRegex};
    ///
    /// let regex_processor = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****").unwrap();
    /// let equal_processor = MaskerEqual::new(vec!["password", "key"], "***");
    ///
    /// let processors = vec![
    ///     MaskerItem::Regex(regex_processor),
    ///     MaskerItem::Equal(equal_processor),
    /// ];
    /// let collection = MaskerCollection::new(processors);
    ///
    /// let input = "My password is 1234 and my key is secret";
    /// let output = collection.process(input);
    ///
    /// assert_eq!(output, "My *** is **** and my *** is ****");
    /// ```
    fn process(&self, input: &str) -> String {
        self.processors
            .iter()
            .fold(input.to_string(), |acc, processor| processor.process(&acc))
    }
}

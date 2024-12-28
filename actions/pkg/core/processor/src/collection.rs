use crate::{Processor, ProcessorItem};

/// Collection of processors that are applied sequentially.
///
/// The `MaskerCollection` struct manages a list of `ProcessorItem` processors and
/// applies them in order to process input strings.
#[derive(Clone)]
pub struct ProcessorCollection {
    /// The list of processors to apply.
    processors: Vec<ProcessorItem>,
}

impl ProcessorCollection {
    /// Creates a new collection of processors.
    ///
    /// # Arguments
    ///
    /// * `processors` - A vector of `ProcessorItem` processors to be applied sequentially.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{ProcessorCollection, ProcessorItem, Processor};
    /// use processor::maskers::{MaskerEqual, MaskerRegex};
    ///
    /// let regex_processor = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****").unwrap();
    /// let equal_processor = MaskerEqual::new(vec!["password", "key"], "***");
    ///
    /// let processors = vec![
    ///     ProcessorItem::Regex(regex_processor),
    ///     ProcessorItem::Equal(equal_processor),
    /// ];
    /// let collection = ProcessorCollection::new(processors);
    /// ```
    pub fn new(processors: Vec<ProcessorItem>) -> Self {
        Self { processors }
    }
}

impl Processor for ProcessorCollection {
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
    /// use processor::{ProcessorCollection, ProcessorItem, Processor};
    /// use processor::maskers::{MaskerEqual, MaskerRegex};
    ///
    /// let regex_processor = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****").unwrap();
    /// let equal_processor = MaskerEqual::new(vec!["password", "key"], "***");
    ///
    /// let processors = vec![
    ///     ProcessorItem::Regex(regex_processor),
    ///     ProcessorItem::Equal(equal_processor),
    /// ];
    /// let collection = ProcessorCollection::new(processors);
    ///
    /// let input = "My password is 1234 and my key is secret";
    /// let output = collection.process(input);
    ///
    /// assert_eq!(output, "My *** is **** and my *** is ****");
    /// ```
    fn process(&self, input: &str) -> String {
        eprintln!("DEBUG - Before masking: {}", input);
        let result = self.processors.iter().fold(input.to_string(), |acc, processor| {
            match processor {
                ProcessorItem::Equal(m) => {
                    let res = m.process(&acc);
                    eprintln!("DEBUG - After Equal mask: {}", res);
                    res
                },
                ProcessorItem::Regex(m) => {
                    let res = m.process(&acc);
                    eprintln!("DEBUG - After Regex mask: {}", res);
                    res
                }
            }
        });
        eprintln!("DEBUG - Final result: {}", result);
        result
        // self.processors
        //     .iter()
        //     .fold(input.to_string(), |acc, processor| processor.process(&acc))
    }
}

/// Trait for implementing different string processing strategies.
///
/// The `Processor` trait defines a standard interface for processing input strings.
/// Implementors can define various processing behaviors, such as masking, filtering,
/// or transforming the input.
pub trait Processor {
    /// Processes the input string and returns the processed result.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to process.
    ///
    /// # Returns
    ///
    /// A new `String` containing the processed result.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::Processor;
    ///
    /// struct UpperCaseProcessor;
    ///
    /// impl Processor for UpperCaseProcessor {
    ///     fn process(&self, input: &str) -> String {
    ///         input.to_uppercase()
    ///     }
    /// }
    ///
    /// let processor = UpperCaseProcessor;
    /// let input = "hello";
    /// let output = processor.process(input);
    /// assert_eq!(output, "HELLO");
    /// ```
    fn process(&self, input: &str) -> String;
}

use crate::Processor;

/// Processor that replaces exact string matches with a mask.
///
/// The `MaskerEqual` struct allows for the replacement of specified substrings
/// with a predefined mask string. This is useful for masking sensitive information
/// that matches exact strings.
#[derive(Clone)]
pub struct MaskerEqual {
    /// List of substrings to be masked.
    substring: Vec<String>,

    /// The string to replace matched substrings with.
    mask: String,
}

impl MaskerEqual {
    /// Creates a new exact match processor.
    ///
    /// # Arguments
    ///
    /// * `substring` - A list of exact strings to mask.
    /// * `mask` - The replacement string to use for masked substrings.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{MaskerEqual, Processor};
    ///
    /// let processor = MaskerEqual::new(vec!["password", "key"], "***");
    ///
    /// let input = "My password is here and my key is safe";
    /// let output = processor.process(input);
    ///
    /// assert_eq!(output, "My *** is here and my *** is safe");
    /// ```
    pub fn new(substring: Vec<&str>, mask: &str) -> Self {
        Self {
            substring: substring.into_iter().map(|s| s.to_string()).collect(),
            mask: mask.to_string(),
        }
    }
}

impl Processor for MaskerEqual {
    /// Processes the input string by replacing exact substring matches with the mask.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to process.
    ///
    /// # Returns
    ///
    /// A new `String` with specified substrings replaced by the mask.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{MaskerEqual, Processor};
    ///
    /// let processor = MaskerEqual::new(vec!["password"], "***");
    ///
    /// let input = "My password is secret";
    /// let output = processor.process(input);
    ///
    /// assert_eq!(output, "My *** is secret");
    /// ```
    fn process(&self, input: &str) -> String {
        let mut output = input.to_string();
        for substring in &self.substring {
            output = output.replace(substring, &self.mask);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match_processing() {
        let processor = MaskerEqual::new(vec!["password", "key"], "***");
        let input = "My password is here and my key is safe";
        let output = processor.process(input);
        assert_eq!(output, "My *** is here and my *** is safe");
    }

    #[test]
    fn test_no_match_processing() {
        let processor = MaskerEqual::new(vec!["secret"], "***");
        let input = "No matching words here";
        let output = processor.process(input);
        assert_eq!(output, "No matching words here");
    }

    #[test]
    fn test_multiple_matches_processing() {
        let processor = MaskerEqual::new(vec!["test"], "****");
        let input = "This is a test. Another test.";
        let output = processor.process(input);
        assert_eq!(output, "This is a ****. Another ****.");
    }
}

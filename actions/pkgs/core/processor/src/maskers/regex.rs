use crate::error::ProcessorError;
use crate::Processor;
use regex::Regex;

/// Processor that uses regular expressions to find and mask patterns.
///
/// The `MaskerRegex` struct allows for the replacement of substrings that match
/// specified regular expression patterns with a predefined mask string. This is useful
/// for masking sensitive information that follows certain patterns, such as numbers or
/// specific keywords.
#[derive(Clone)]
pub struct MaskerRegex {
    /// List of compiled regular expressions to match against the input.
    patterns: Vec<Regex>,

    /// The string to replace matched patterns with.
    mask: String,
}

impl MaskerRegex {
    /// Creates a new regular expression-based processor.
    ///
    /// # Arguments
    ///
    /// * `patterns` - A list of regex patterns to match.
    /// * `mask` - The replacement string to use for matched patterns.
    ///
    /// # Errors
    ///
    /// Returns a `ProcessorError::RegexError` if any of the regex patterns fail to compile.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{MaskerRegex, Processor, ProcessorError};
    ///
    /// let regex_processor = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****").unwrap();
    ///
    /// let input = "My password is 1234 and my secret code is 5678";
    /// let output = regex_processor.process(input);
    ///
    /// assert_eq!(output, "My password is **** and my **** code is ****");
    /// ```
    pub fn new<T: AsRef<str>>(patterns: Vec<T>, mask: &str) -> Result<Self, ProcessorError> {
        let patterns = patterns
            .iter()
            .map(|p| Regex::new(p.as_ref()).map_err(|e| ProcessorError::RegexError(e.to_string())))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            patterns,
            mask: mask.to_string(),
        })
    }
}

impl Processor for MaskerRegex {
    /// Processes the input string by replacing patterns matched by the regexes with the mask.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to process.
    ///
    /// # Returns
    ///
    /// A new `String` with specified patterns replaced by the mask.
    ///
    /// # Example
    ///
    /// ```rust
    /// use processor::{MaskerRegex, Processor, ProcessorError};
    ///
    /// let regex_processor = MaskerRegex::new(vec![r"\d{4}"], "****").unwrap();  // Убрали паттерн "secret"
    ///
    /// let input = "My password is 1234 and my secret code is 5678";
    /// let output = regex_processor.process(input);
    ///
    /// assert_eq!(output, "My password is **** and my secret code is ****");
    /// ```
    fn process(&self, input: &str) -> String {
        let mut output = input.to_string();
        for pattern in &self.patterns {
            output = pattern.replace_all(&output, &self.mask as &str).to_string();
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_processing() {
        let processor = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****").unwrap();
        let input = "My password is 1234 and my secret code is 5678";
        let output = processor.process(input);
        assert_eq!(output, "My password is **** and my **** code is ****");
    }

    #[test]
    fn test_invalid_regex() {
        let result = MaskerRegex::new(vec![r"[invalid"], "****");
        assert!(result.is_err());
        match result {
            Err(ProcessorError::RegexError(_)) => (),
            _ => panic!("Expected RegexError"),
        }
    }

    #[test]
    fn test_multiple_regex_patterns() {
        let processor = MaskerRegex::new(vec![r"\d+", r"foo"], "X").unwrap();
        let input = "foo bar 123 baz 456 foo";
        let output = processor.process(input);
        assert_eq!(output, "X bar X baz X X");
    }

    #[test]
    fn test_no_match_processing() {
        let processor = MaskerRegex::new(vec![r"xyz"], "****").unwrap();
        let input = "No matches here.";
        let output = processor.process(input);
        assert_eq!(output, "No matches here.");
    }
}

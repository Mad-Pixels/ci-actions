use crate::error::ProcessorError;
use crate::Processor;
use regex::Regex;

/// Processor that uses regular expressions to find and mask patterns
#[derive(Clone)]
pub struct MaskerRegex {
    patterns: Vec<Regex>,
    mask: String,
}

impl MaskerRegex {
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
}

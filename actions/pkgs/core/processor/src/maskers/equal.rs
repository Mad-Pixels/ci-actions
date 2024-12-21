use crate::Processor;

/// Processor that replaces exact string matches with a mask
#[derive(Clone)]
pub struct MaskerEqual {
    substring: Vec<String>,
    mask: String,
}

impl MaskerEqual {
    /// Create new exact match processor
    ///
    /// # Arguments
    /// * `substring` - List of strings to mask
    /// * `mask` - Replacement string
    pub fn new(substring: Vec<&str>, mask: &str) -> Self {
        Self {
            substring: substring.into_iter().map(|s| s.to_string()).collect(),
            mask: mask.to_string(),
        }
    }
}

impl Processor for MaskerEqual {
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
}

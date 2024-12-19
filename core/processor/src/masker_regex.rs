use regex::Regex;
use crate::Masker;

pub struct MaskerRegex {
    patterns: Vec<Regex>,
    mask: String,
}

impl MaskerRegex {
    pub fn new(patterns: Vec<&str>, mask: &str) -> Self {
        Self { 
            patterns: patterns.into_iter().map(|p| Regex::new(p).unwrap()).collect(),
            mask: mask.to_string(),
        }
    }
}

impl Masker for MaskerRegex {
    fn process(&self, input: &str) -> String {
        let mut output = input.to_string();
        for pattern in &self.patterns {
            output = pattern.replace_all(&output, &self.mask as &str).to_string();
        }
        output
    }
}
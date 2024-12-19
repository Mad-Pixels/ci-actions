use crate::Masker;

#[derive(Clone)]
pub struct MaskerEqual {
    substring: Vec<String>,
    mask: String,
}

impl MaskerEqual {
    pub fn new(substring: Vec<&str>, mask: &str) -> Self {
        Self {
            substring: substring.into_iter().map(|s| s.to_string()).collect(),
            mask: mask.to_string(),
        }
    }
}

impl Masker for MaskerEqual {
    fn process(&self, input: &str) -> String {
        let mut output = input.to_string();
        for substring in &self.substring {
            output = output.replace(substring, &self.mask);
        }
        output
    }
}
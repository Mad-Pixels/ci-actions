use masker_equal::MaskerEqual;
use masker_regex::MaskerRegex;

pub mod masker_regex;
pub mod masker_equal;


pub trait Masker {
    fn process(&self, input: &str) -> String;
}

#[derive(Clone)]
pub enum Item {
    Regex(MaskerRegex),
    Equal(MaskerEqual),
}

impl Item {
    pub fn process(&self, input: &str) -> String {
        match self {
            Item::Regex(masker) => masker.process(input),
            Item::Equal(masker) => masker.process(input),
        }
    }
}

#[derive(Clone)]
pub struct Collection {
    maskers: Vec<Item>,
}

impl Collection {
    pub fn new(maskers: Vec<Item>) -> Self {
        Self { maskers }
    }
    pub fn process(&self, input: &str) -> String {
        self.maskers.iter().fold(input.to_string(), |acc, masker| {
            masker.process(&acc)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::masker_equal::MaskerEqual;
    use crate::masker_regex::MaskerRegex;

    #[test]
    fn test_masker_regexp() {
        let masker = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****");
        let input = "My password is 1234 and my secret code is 5678";
        let output = masker.process(input);
        assert_eq!(output, "My password is **** and my **** code is ****");
    }

    #[test]
    fn test_masker_equal() {
        let masker = MaskerEqual::new(vec!["password", "key"], "***");
        let input = "My password is here and my key is safe";
        let output = masker.process(input);
        assert_eq!(output, "My *** is here and my *** is safe");
    }

    #[test]
    fn test_masker_manager() {
        let regexp_masker = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****");
        let equal_masker = MaskerEqual::new(vec!["password", "key"], "***");

        let maskers = vec![
            Item::Regex(regexp_masker),
            Item::Equal(equal_masker),
        ];

        let manager = Collection::new(maskers);

        let input = "My password is 1234 and my key is secret";
        let output = manager.process(input);

        assert_eq!(output, "My *** is **** and my *** is ****");
    }

    #[test]
    fn test_manager_order_of_operations() {
        let first_masker = MaskerEqual::new(vec!["first"], "1st");
        let second_masker = MaskerEqual::new(vec!["second"], "2nd");

        let maskers = vec![
            Item::Equal(first_masker),
            Item::Equal(second_masker),
        ];

        let manager = Collection::new(maskers);

        let input = "This is the first and the second example.";
        let output = manager.process(input);

        assert_eq!(output, "This is the 1st and the 2nd example.");
    }
}
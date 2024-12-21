pub mod maskers;

mod collection;
mod error;
mod item;
mod traits;

pub use collection::MaskerCollection;
pub use error::ProcessorError;
pub use item::MaskerItem;
pub use maskers::{MaskerEqual, MaskerRegex};
pub use traits::Processor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_chain() {
        let regexp_processor = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****").unwrap();
        let equal_processor = MaskerEqual::new(vec!["password", "key"], "***");

        let processors = vec![MaskerItem::Regex(regexp_processor), MaskerItem::Equal(equal_processor)];
        let collection = MaskerCollection::new(processors);
        let input = "My password is 1234 and my key is secret";
        let output = collection.process(input);

        assert_eq!(output, "My *** is **** and my *** is ****");
    }

    #[test]
    fn test_processing_order() {
        let first_processor = MaskerEqual::new(vec!["first"], "1st");
        let second_processor = MaskerEqual::new(vec!["second"], "2nd");

        let processors = vec![MaskerItem::Equal(first_processor), MaskerItem::Equal(second_processor)];
        let collection = MaskerCollection::new(processors);
        let input = "This is the first and the second example.";
        let output = collection.process(input);

        assert_eq!(output, "This is the 1st and the 2nd example.");
    }
}

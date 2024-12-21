//! # Processor Crate
//!
//! The `processor` crate provides functionality to process and mask sensitive data within strings.
//! It offers a flexible system of processors that can be combined to handle various masking strategies.
//!
//! ## Modules
//!
//! - [`maskers`]: Contains implementations of different masking strategies.
//! - [`collection`]: Manages collections of processors applied sequentially.
//! - [`error`]: Defines error types and result aliases used across the crate.
//! - [`item`]: Defines the `MaskerItem` enum representing different processor types.
//! - [`traits`]: Contains the `Processor` trait that all processors must implement.
//!
//! ## Usage
//!
//! Below is a basic example of how to create a collection of processors and process an input string.
//!
//! ```rust
//! use processor::{MaskerCollection, MaskerItem, Processor, ProcessorError};
//! use processor::maskers::{MaskerEqual, MaskerRegex};
//!
//! // Create individual processors
//! let regex_processor = MaskerRegex::new(vec![r"\d{4}", r"secret"], "****").unwrap();
//! let equal_processor = MaskerEqual::new(vec!["password", "key"], "***");
//!
//! // Combine processors into MaskerItems
//! let processors = vec![
//!     MaskerItem::Regex(regex_processor),
//!     MaskerItem::Equal(equal_processor),
//! ];
//!
//! // Create a collection of processors
//! let collection = MaskerCollection::new(processors);
//!
//! // Process an input string
//! let input = "My password is 1234 and my key is secret";
//! let output = collection.process(input);
//!
//! assert_eq!(output, "My *** is **** and my *** is ****");
//! ```
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

        let processors = vec![
            MaskerItem::Regex(regexp_processor),
            MaskerItem::Equal(equal_processor),
        ];
        let collection = MaskerCollection::new(processors);
        let input = "My password is 1234 and my key is secret";
        let output = collection.process(input);

        assert_eq!(output, "My *** is **** and my *** is ****");
    }

    #[test]
    fn test_processing_order() {
        let first_processor = MaskerEqual::new(vec!["first"], "1st");
        let second_processor = MaskerEqual::new(vec!["second"], "2nd");

        let processors = vec![
            MaskerItem::Equal(first_processor),
            MaskerItem::Equal(second_processor),
        ];
        let collection = MaskerCollection::new(processors);
        let input = "This is the first and the second example.";
        let output = collection.process(input);

        assert_eq!(output, "This is the 1st and the 2nd example.");
    }
}

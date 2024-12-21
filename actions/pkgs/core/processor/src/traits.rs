/// Trait for implementing different string processing strategies
pub trait Processor {
    /// Process input string and return processed result
    fn process(&self, input: &str) -> String;
}

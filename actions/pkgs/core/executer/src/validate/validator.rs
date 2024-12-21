use crate::{Context, ExecuterResult};
use super::traits::ValidationRule;

/// A validator that runs a collection of validation rules against a context.
///
/// The `Validator` struct manages a list of `ValidationRule` implementations
/// and applies them in order based on their priority.
pub struct Validator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl Validator {
    /// Creates a new `Validator` with the given set of validation rules.
    ///
    /// The rules are sorted by their priority, with lower numbers having higher priority.
    ///
    /// # Arguments
    ///
    /// * `rules` - A vector of boxed validation rules.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::{Context, ExecuterError, Validator, ValidationRule};
    /// use std::collections::HashMap;
    ///
    /// struct SampleRule;
    ///
    /// impl ValidationRule for SampleRule {
    ///     fn validate(&self, _context: &Context) -> executer::error::ExecuterResult<()> {
    ///         Ok(())
    ///     }
    ///
    ///     fn name(&self) -> &'static str {
    ///         "SampleRule"
    ///     }
    /// }
    ///
    /// let rules: Vec<Box<dyn ValidationRule>> = vec![Box::new(SampleRule)];
    /// let validator = Validator::new(rules);
    /// ```
    pub fn new(mut rules: Vec<Box<dyn ValidationRule>>) -> Self {
        rules.sort_by_key(|rule| rule.priority());
        Self { rules }
    }

    /// Creates a default `Validator` with standard validation rules.
    /// path, cwd, cmd 
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::Validator;
    ///
    /// let validator = Validator::default();
    /// ```
    pub fn default() -> Self {
        Self::new(super::rules::standard_rules())
    }

    /// Validates the given context against all validation rules.
    ///
    /// Runs each validation rule in order of priority. If any rule fails,
    /// the validation process stops, and an error is returned.
    ///
    /// # Arguments
    ///
    /// * `context` - The command context.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the validation rules fail.
    ///
    /// # Example
    ///
    /// ```rust
    /// use executer::{Validator, ValidationRule, Context, ExecuterError};
    /// use std::collections::HashMap;
    ///
    /// struct AlwaysValidRule;
    ///
    /// impl ValidationRule for AlwaysValidRule {
    ///     fn validate(&self, _context: &Context) -> executer::error::ExecuterResult<()> {
    ///         Ok(())
    ///     }
    ///
    ///     fn name(&self) -> &'static str {
    ///         "AlwaysValidRule"
    ///     }
    /// }
    ///
    /// let rules: Vec<Box<dyn ValidationRule>> = vec![Box::new(AlwaysValidRule)];
    /// let validator = Validator::new(rules);
    /// let context = Context::new(vec!["echo".to_string()], HashMap::new(), None);
    /// assert!(validator.validate(&context).is_ok());
    /// ```
    pub fn validate(&self, context: &Context) -> ExecuterResult<()> {
        for rule in self.rules.iter() {
            rule.validate(context)?;
        }
        Ok(())
    }
}

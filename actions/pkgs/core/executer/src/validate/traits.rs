use crate::{ExecuterResult, Context};

/// A trait that defines a validation rule for a command context.
///
/// Implementors of this trait can perform various validations on a `Context`.
///
/// # Examples
///
/// ```rust
/// use executer::{ValidationRule, ExecuterError, ExecuterResult, Context};
/// use std::collections::HashMap;
///
/// struct AlwaysValidRule;
///
/// impl ValidationRule for AlwaysValidRule {
///     fn validate(&self, _context: &Context) -> ExecuterResult<()> {
///         Ok(())
///     }
///
///     fn name(&self) -> &'static str {
///         "AlwaysValidRule"
///     }
/// }
///
/// let rule = AlwaysValidRule;
/// let context = Context::new(vec!["echo".to_string()], HashMap::new(), None);
/// assert!(rule.validate(&context).is_ok());
/// ```
pub trait ValidationRule: Send + Sync {
    /// Validates the given context.
    ///
    /// Returns `Ok(())` if the context is valid, or an `ExecuterError` otherwise.
    ///
    /// # Arguments
    ///
    /// * `context` - The context to validate.
    ///
    /// # Errors
    ///
    /// Returns an error if the context fails validation.
    fn validate(&self, context: &Context) -> ExecuterResult<()>;

    /// Returns the name of the validation rule.
    fn name(&self) -> &'static str;

    /// Returns the priority of the validation rule.
    fn priority(&self) -> i32 {
        5
    }
}

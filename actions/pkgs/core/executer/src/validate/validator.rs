use super::traits::ValidationRule;
use crate::context::Context;
use crate::error::ExecuterResult;

pub struct Validator {
    rules: Vec<Box<dyn ValidationRule>>,
}

impl Validator {
    pub fn new(mut rules: Vec<Box<dyn ValidationRule>>) -> Self {
        rules.sort_by_key(|rule| rule.priority());
        Self { rules }
    }

    pub fn default() -> Self {
        Self::new(super::rules::standard_rules())
    }

    pub fn validate(&self, context: &Context) -> ExecuterResult<()> {
        for rule in self.rules.iter() {
            rule.validate(context)?;
        }
        Ok(())
    }
}

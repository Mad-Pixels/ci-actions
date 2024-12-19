use crate::error::ExecuterResult;
use super::ValidationContext;

pub trait ValidationRule: Send + Sync {
    fn validate(&self, context: &ValidationContext) -> ExecuterResult<()>;

    fn name(&self) -> &'static str;

    fn priority(&self) -> i32 { 5 }
}
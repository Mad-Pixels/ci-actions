use crate::error::ExecuterResult;
use crate::context::Context;

pub trait ValidationRule: Send + Sync {
    fn validate(&self, context: &Context) -> ExecuterResult<()>;

    fn name(&self) -> &'static str;

    fn priority(&self) -> i32 { 5 }
}
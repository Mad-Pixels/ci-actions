use crate::context::Context;
use crate::error::ExecuterResult;

pub trait ValidationRule: Send + Sync {
    fn validate(&self, context: &Context) -> ExecuterResult<()>;

    fn name(&self) -> &'static str;

    fn priority(&self) -> i32 {
        5
    }
}

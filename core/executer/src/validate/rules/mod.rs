mod path;
mod env;
mod cmd;

pub use cmd::CmdRule;
pub use env::EnvRule;
pub use path::PathRule;

pub fn standard_rules() -> Vec<Box<dyn super::traits::ValidationRule>> {
    vec![
        Box::new(cmd::CmdRule::new()),
        Box::new(env::EnvRule::new()),
        Box::new(path::PathRule::new()),
    ]
}
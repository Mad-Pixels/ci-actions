mod cmd;
mod env;
mod path;

pub use cmd::CmdRule;
pub use env::EnvRule;
pub use path::PathRule;

/// Define function with set of standart validation rules.
pub fn standard_rules() -> Vec<Box<dyn super::traits::ValidationRule>> {
    vec![
        Box::new(cmd::CmdRule::new()),
        Box::new(env::EnvRule::new()),
        Box::new(path::PathRule::new()),
    ]
}

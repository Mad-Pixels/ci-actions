//! # Executer Crate
//!
//! The `executer` crate provides functionality to execute system commands with
//! validation, output handling, and environment management. It ensures that
//! commands are safe to execute by applying a series of validation rules and
//! processes the command output accordingly.
//!
//! ## Modules
//!
//! - [`context`]: Defines the execution context, including command, environment variables, working directory, and timeout.
//! - [`error`]: Defines error types and result aliases used across the crate.
//! - [`output`]: Handles output processing, including logging and writing to various targets.
//! - [`validate`]: Contains validation rules to ensure commands are safe to execute.
//! - [`subprocess`]: Manages the execution of subprocesses with proper validation and output handling.
//!
//! ## Usage
//!
//! Below is a basic example of how to create a context and execute a command using the `Subprocess` struct.
//!
//! ```rust,no_run
//! use executer::{Context, Output, Target, Subprocess, Validator};
//! use processor::{Collection, Item};
//! use processor::maskers::regex::MaskerRegex;
//! use std::collections::HashMap;
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create a processor with maskers for sensitive data
//!     let processor = Collection::new(vec![
//!         Item::Regex(MaskerRegex::new(vec![r"password=\w+"], "****").unwrap())
//!     ]);
//!
//!     // Initialize the output handler
//!     let output = Output::new(
//!         processor,
//!         Target::Stdout,
//!         Target::Stderr,
//!     );
//!
//!     // Initialize the validator with default rules
//!     let validator = Validator::default();
//!
//!     // Create a subprocess instance
//!     let subprocess = Subprocess::new(output, validator);
//!
//!     // Define the command to execute
//!     let command = vec!["echo".to_string(), "Hello, World!".to_string()];
//!     let env = HashMap::new();
//!
//!     // Create a context without a specific working directory and with a timeout
//!     let context = Context::new(command, env, None).with_timeout(5);
//!
//!     // Execute the command
//!     match subprocess.execute(context).await {
//!         Ok(status) => println!("Command executed with status: {}", status),
//!         Err(e) => eprintln!("Command execution failed: {}", e),
//!     }
//! }
//! ```

mod context;
mod error;
mod output;
mod subprocess;
mod validate;

pub use validate::rules;
/// Re-exports for easier access to key components.
pub use validate::traits::ValidationRule;
pub use validate::Validator;

pub use error::ExecuterError;
pub use error::ExecuterResult;

pub use output::Output;
pub use output::Target;

pub use context::Context;

pub use subprocess::Subprocess;

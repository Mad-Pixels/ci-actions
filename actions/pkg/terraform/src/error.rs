use thiserror::Error;

/// Represents the different errors that can occur when executing Terraform commands.
#[derive(Error, Debug)]
pub enum TerraformError {
    /// Error when a Terraform command fails.
    #[error("Terraform command failed: {0}")]
    CommandError(String),

    /// Error related to Terraform workspace operations.
    #[error("Terraform workspace error: {0}")]
    WorkspaceError(String),

    /// Error during Terraform initialization.
    #[error("Terraform initialization error: {0}")]
    InitError(String),

    /// Error during Terraform plan creation.
    #[error("Terraform plan error: {0}")]
    PlanError(String),

    /// Error during Terraform apply execution.
    #[error("Terraform apply error: {0}")]
    ApplyError(String),

    /// Error from the underlying executor.
    #[error(transparent)]
    ExecuterError(#[from] executer::ExecuterError),
}

/// A type alias for results returned by Terraform operations.
pub type TerraformResult<T> = Result<T, TerraformError>;

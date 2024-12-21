use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerraformError {
    #[error("Terraform command failed: {0}")]
    CommandError(String),

    #[error("Terraform workspace error: {0}")]
    WorkspaceError(String),

    #[error("Terraform initialization error: {0}")]
    InitError(String),

    #[error("Terraform plan error: {0}")]
    PlanError(String),

    #[error("Terraform apply error: {0}")]
    ApplyError(String),

    #[error(transparent)]
    ExecuterError(#[from] executer::ExecuterError),
}

pub type TerraformResult<T> = Result<T, TerraformError>;

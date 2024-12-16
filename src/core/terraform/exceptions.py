class TerraformError(Exception):
    """Base terraform error"""
    pass

class TerraformValidationError(TerraformError):
    """Validation error"""
    pass

class TerraformExecutionError(TerraformError):
    """Execution terraform commands error"""
    def __init__(self, action: str, message: str):
        super().__init__(f"Terraform {action} error: {message}")
        self.action = action

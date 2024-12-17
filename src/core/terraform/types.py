from typing import Dict, Any, Optional
from dataclasses import dataclass
from enum import Enum

from src.core.executer.base import ExecutionResult

class TerraformAction(Enum):
    INIT = "init"
    PLAN = "plan"
    APPLY = "apply"
    WORKSPACE = "workspace"
    OUTPUT = "output"

@dataclass
class TerraformResult:
    """
    Represents the result of a Terraform command execution.

    Attributes:
        action (TerraformAction): The Terraform action that was performed 
                                  (e.g., INIT, PLAN, APPLY, OUTPUT, WORKSPACE).
        result (ExecutionResult): The raw execution result containing status, stdout, stderr,
                                  and optionally masked output.
        outputs (Optional[Dict[str, Any]]): Parsed Terraform outputs (typically JSON) if applicable.
                                            This is used specifically for the `terraform output` command.
    """
    action: TerraformAction
    result: ExecutionResult
    outputs: Optional[Dict[str, Any]] = None

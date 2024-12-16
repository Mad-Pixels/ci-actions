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
    """Terraform result object"""
    action: TerraformAction
    result: ExecutionResult
    outputs: Optional[Dict[str, Any]] = None

    #TODO: some additional data we can put here.

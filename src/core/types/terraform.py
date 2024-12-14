from typing import TypedDict, Union, Literal
from typing_extensions import NotRequired

class TerraformVariable(TypedDict):
    """Terraform variable definition"""
    value: Union[str, int, float, bool, list, dict]
    type: NotRequired[Literal["string", "number", "bool", "list", "map", "any"]]
    sensitive: NotRequired[bool]

class TerraformOutput(TypedDict):
    """Terraform output value"""
    sensitive: bool
    type: str
    value: Union[str, int, float, bool, list, dict]

class TerraformPlanResult(TypedDict):
    """Result of terraform plan operation"""
    success: bool
    changes: bool  # True if plan contains changes
    output: str    # Raw plan output
    error: NotRequired[str]

class TerraformApplyResult(TypedDict):
    """Result of terraform apply operation"""
    success: bool
    output: str
    error: NotRequired[str]
    outputs: NotRequired[dict[str, TerraformOutput]]

class TerraformFormatResult(TypedDict):
    """Result of terraform fmt operation"""
    success: bool
    changed: bool  # True if files were reformatted
    files: list[str]  # List of changed files
    error: NotRequired[str]
from typing import TypedDict, Optional
from pathlib import Path

class TerraformPlanConfig(TypedDict):
    """Configuration for terraform plan"""
    working_dir: str
    workspace: Optional[str]
    vars_file: Optional[str]
    backend_config: Optional[dict[str, str]]
    output_file: Optional[str]

def get_github_config() -> dict:
    """Configuration for GitHub environment"""
    return {
        "type": "github",
        "mask_secrets": True,
    }

def get_terraform_secrets() -> list[dict]:
    """Required secrets for Terraform"""
    return [
        {
            "name": "AWS_SECRET_ACCESS_KEY",
            "env_var": "AWS_SECRET_ACCESS_KEY",
            "required": True
        },
        {
            "name": "AWS_ACCESS_KEY_ID",
            "env_var": "AWS_ACCESS_KEY_ID",
            "required": True
        },
        {
            "name": "AWS_ACCOUNT_ID",
            "env_var": "AWS_ACCOUNT_ID",
            "required": True
        },
        {
            "name": "AWS_REGION",
            "env_var": "AWS_REGION",
            "default": "us-east-1"
        },
    ]

def get_terraform_variables() -> list[dict]:
    """Required variables for Terraform"""
    return [
        {
            "name": "TF_WORKSPACE",
            "env_var": "TF_WORKSPACE",
            "required": False
        }
    ]
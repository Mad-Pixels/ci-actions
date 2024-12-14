from typing import Optional
from pathlib import Path

import asyncio
import logging
import click
import os

from core.terraform.client import TerraformClient
from environment.github import GitHubEnvironment
from .config import (
    TerraformPlanConfig,
    get_github_config,
    get_terraform_secrets,
    get_terraform_variables
)

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

async def run_terraform_plan(config: TerraformPlanConfig) -> int:
    """Run terraform plan with provided configuration"""
    
    logger.info("Starting terraform plan")
    logger.info(f"Working directory: {config['working_dir']}")
    
    logger.info("Setting up environment")
    env = GitHubEnvironment(
        config=get_github_config(),
        secrets=get_terraform_secrets(),
        variables=get_terraform_variables()
    )
    
    logger.info("Preparing environment variables")
    env_vars = await env.prepare()
    logger.info(f"Environment variables set: {', '.join(env_vars.keys())}")
    
    logger.info("Creating Terraform client")
    tf = TerraformClient(
        working_dir=config["working_dir"],
        variables=env_vars,
    )
    
    logger.info("Checking terraform installation")
    result = await tf._run_command("version")
    if not result[0]:
        logger.error("Terraform not found or not executable")
        return 1
        
    logger.info("Initializing terraform")
    init_result = await tf.init(
        backend_config={
            "bucket": os.environ.get("TF_BACKEND_bucket"),
            "key": os.environ.get("TF_BACKEND_key"),
            "region": os.environ.get("TF_BACKEND_region")
        },
        reconfigure=True,
    )
    if not init_result["success"]:
        logger.error(f"Terraform init failed: {init_result.get('error')}")
        return 1

    logger.info("Running terraform plan")
    plan_result = await tf.plan(
        detailed_exitcode=True
    )

@click.command()
@click.option(
    "--working-dir",
    type=click.Path(exists=True, file_okay=False, dir_okay=True, path_type=Path),
    required=True,
    help="Directory containing Terraform configuration"
)
@click.option(
    "--workspace",
    type=str,
    help="Terraform workspace to use"
)
@click.option(
    "--vars-file",
    type=click.Path(exists=True, dir_okay=False, path_type=Path),
    help="Terraform variables file"
)
@click.option(
    "--output-file",
    type=click.Path(dir_okay=False, path_type=Path),
    help="Save plan to file"
)
def main(
    working_dir: Path,
    workspace: Optional[str] = None,
    vars_file: Optional[Path] = None,
    output_file: Optional[Path] = None
) -> None:
    """Run terraform plan in GitHub Actions environment"""
    config: TerraformPlanConfig = {
        "working_dir": str(working_dir),
        "workspace": workspace,
        "vars_file": str(vars_file) if vars_file else None,
        "output_file": str(output_file) if output_file else None,
    }
    
    exit_code = asyncio.run(run_terraform_plan(config))
    raise SystemExit(exit_code)

if __name__ == "__main__":
    main()

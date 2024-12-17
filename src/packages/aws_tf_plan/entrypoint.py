from typing import Optional, Dict, Any, List
from pathlib import Path

import asyncio
import logging
import os

from core.terraform.exceptions import TerraformExecutionError
from core.providers.aws import AWSProvider
from tf_plan.tf_plan import run_tf_plan

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s',
    datefmt='%Y-%m-%d %H:%M:%S'
)
logger = logging.getLogger("AWS::TF::plan")

async def main():
    """
    Entry point for running Terraform plan with AWS provider.
    """
    provider = AWSProvider(
        access_key_id="",
        secret_access_key="",
        session_token=os.getenv("AWS_SESSION_TOKEN") 
    )
    provider.validate()

    sensitive = {
        "AWS_REGION": "",
        "AWS_ACCOUNT_ID": "",
        "AWS_SECRET_ACCESS_KEY": provider.secret_access_key, 
        "TF_VAR_acm_crt": ""
    }
    base_env = {
        "AWS_REGION": sensitive["AWS_REGION"],
        "AWS_ACCOUNT_ID": sensitive["AWS_ACCOUNT_ID"]
    }
    tf_vars = {
        "project": "",
        "domain": "",
        "aws_region": "",
        "acm_crt": sensitive["TF_VAR_acm_crt"],
        "tf_bucket": "tfstates-madpixels",
        "tf_key": "personal/service.tfstate"
    }

    base_cwd = Path("/Users/igoss/Desktop/person/terraform/provisioners/infra")
    if not base_cwd.exists() or not base_cwd.is_dir():
        logger.error(f"Working directory does not exist or is not a directory: {base_cwd}")
        return

    # Определяем workspace и  аргументы для команды plan
    workspace = "default"  # Или другое название workspace, если необходимо
    plan_args = ["-out=aws_planfile"]

    logger.info("Starting Terraform plan workflow...")
    try:
        async for line in run_tf_plan(
            provider=provider,
            base_cwd=base_cwd,
            base_env=base_env,
            sensitive=sensitive,
            workspace=workspace,
            tf_vars=tf_vars,
            args=plan_args
        ):
            print(line, end='')  # Выводим строку как есть, без удаления пробелов

        logger.info("Terraform plan completed successfully")
    except TerraformExecutionError as e:
        logger.error(f"Terraform plan failed: {e}")
    except Exception as e:
        logger.error(f"An unexpected error occurred during Terraform plan: {e}")

if __name__ == "__main__":
    asyncio.run(main())

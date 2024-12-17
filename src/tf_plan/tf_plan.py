from typing import Optional, Dict, Any, AsyncGenerator, List
from pathlib import Path

import logging

from core.terraform.exceptions import TerraformExecutionError
from core.terraform.terraform import Terraform
from core.executer.executer import IsolateExecuter
from core.executer.masker import OutputMasker
from core.providers.base import BaseProvider

logger = logging.getLogger(__name__)

async def run_tf_plan(
    provider: BaseProvider,
    base_cwd: Path,
    base_env: Dict[str, str],
    sensitive: Dict[str, str],
    workspace: Optional[str] = None,
    tf_vars: Optional[Dict[str, str]] = None,
    args: Optional[List[str]] = None,
) -> AsyncGenerator[str, None]:
    """
    Run a Terraform plan workflow:
    1. Initialize Terraform.
    2. Optionally select a workspace.
    3. Run `terraform plan` with optional tf_vars and args.

    Args:
        provider (BaseProvider): A provider supplying credentials and sensitive data.
        base_cwd (Path): The directory where Terraform commands will be executed.
        base_env (Dict[str, str]): Base environment variables for Terraform commands.
        sensitive (Dict[str, str]): Additional sensitive variables to be masked and passed into env.
        workspace (Optional[str]): Optional workspace name to select before planning.
        tf_vars (Optional[Dict[str, str]]): Optional Terraform variables to pass as TF_VAR_*.
        args (Optional[List[str]]): Additional command-line arguments for `terraform plan`.

    Yields:
        Lines of Terraform plan output as they are generated.

    Raises:
        TerraformExecutionError: If any of the Terraform commands fail.
    """
    masker = OutputMasker()
    for val in sensitive.values():
        if val:
            masker.sensitive(val)

    terraform = Terraform(
        executer=IsolateExecuter(processor=masker),
        sensitive=sensitive,
        base_cwd=base_cwd,
        base_env=base_env,
        provider=provider
    )
    try:
        async for line in terraform.init():
            yield line
    except TerraformExecutionError as e:
        logger.error(f"Terraform init failed: {e}")
        yield f"Terraform init failed: {e}"
        return

    if workspace:
        try:
            async for line in terraform.workspace(action="select", name=workspace):
                yield line
        except TerraformExecutionError as e:
            logger.error(f"Failed to select workspace '{workspace}': {e}")
            yield f"Failed to select workspace '{workspace}': {e}"
            return

    try:
        async for line in terraform.plan(args=args, tf_vars=tf_vars):
            yield line
    except TerraformExecutionError as e:
        logger.error(f"Terraform plan failed: {e}")
        yield f"Terraform plan failed: {e}"

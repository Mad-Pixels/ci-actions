from typing import Dict, Optional, List
from pathlib import Path

import logging

from src.core.executer.executer import IsolateExecuter, ExecutionResult
from src.core.providers.base import BaseProvider
from .types import TerraformAction, TerraformResult
from .exceptions import TerraformExecutionError
from .utils import parse_terraform_output, get_default_masker

logger = logging.getLogger(__name__)

class Terraform:
    """
    A utility class for managing Terraform operations such as `init`, `plan`, `apply`, `workspace`, and `output`.
    Ensures consistent execution, environment handling, and masking of sensitive data.

    Attributes:
        _base_cwd (Path): The working directory where Terraform commands will be executed.
        _executer (IsolateExecuter): A command executor responsible for running Terraform commands.
        _env (Dict[str, str]): Base environment variables required for command execution.
        _all_sensitive (Dict[str, str]): Additional sensitive environment variables to be masked.
    """
    def __init__(
        self,
        base_cwd: Path,
        executer: IsolateExecuter,
        provider: Optional[BaseProvider]=None,
        base_env: Optional[Dict[str, str]]=None,
        sensitive: Optional[Dict[str, str]]=None,
    ):
        """
        Initializes the Terraform object.

        Args:
            base_cwd: The base directory where Terraform commands will run.
            executer: An executor implementing the CommandExecuter interface for running commands.
            provider: Optional provider for injecting authorization credentials (e.g., AWS, GCP).
            base_env: Base environment variables required for Terraform execution.
            sensitive: Additional sensitive variables that will be passed to Terraform but masked in logs.

        Workflow:
            - Combines `base_env` and provider-specific environment variables.
            - Tracks sensitive data to ensure they are passed but never logged or exposed in outputs.
            - Prepares the command executor (`executer`) to apply masking logic.
        """
        self._base_cwd = base_cwd
        self._executer = executer
        self._env = base_env or {}

        if provider:
            provider.validate()
            provider_env = provider.get_environment()
            self._env.update(provider_env)

        self._all_sensitive = sensitive or {}
        if provider:
            self._all_sensitive.update(provider.get_sensitive())

        processor = getattr(self._executer, "_processor", None)
        if sensitive and processor:
            for val in filter(None, sensitive.values()):
                processor.sensitive(val)
        
    async def init(self, args: Optional[List[str]]=None) -> TerraformResult:
        """
        Runs the `terraform init` command.

        Args:
            args: Additional command-line arguments to pass to `terraform init`.

        Returns:
            TerraformResult: Contains the action performed and execution results.
        """
        cmd = ["terrafrom", "init"]
        if args:
            cmd.extend(args)
        
        result = await self._run(TerraformAction.INIT, cmd)
        return TerraformResult(action=TerraformAction.INIT, result=result)
    
    async def plan(
        self,
        args: Optional[List[str]]=None,
        tf_vars: Optional[Dict[str, str]]=None,
    ) -> TerraformResult:
        """
        Runs the `terraform plan` command.

        Args:
            args: Additional command-line arguments for `terraform plan`.
            tf_vars: Key-value pairs representing Terraform input variables (TF_VAR_*).

        Returns:
            TerraformResult: Contains the action performed and execution results.

        Example:
            tf_vars = {"region": "us-west-2"}
            => Becomes {"TF_VAR_region": "us-west-2"}
        """
        cmd = ["terraform", "plan", "-input=false"]
        if args:
            cmd.extend(args)

        env = self._prepare_tf_env(tf_vars)
        result = await self._run(TerraformAction.PLAN, cmd, env=env)
        return TerraformResult(action=TerraformAction.PLAN, result=result)
    
    async def apply(
        self,
        args: Optional[List[str]]=None,
        tf_vars: Optional[Dict[str, str]]=None,
        auto_approve: bool = True
    ) -> TerraformResult:
        """
        Runs the `terraform apply` command to apply the Terraform plan.

        Args:
            args: Additional command-line arguments for `terraform apply`.
            tf_vars: Terraform input variables to inject into the execution environment.
            auto_approve: Skips confirmation prompts by adding `-auto-approve`.

        Returns:
            TerraformResult: Contains the action performed and execution results.
        """
        cmd = ["terraform", "apply", "-input=false"]
        if auto_approve:
            cmd.append("-auto-approve")
        if args:
            cmd.extend(args)

        env = self._prepare_tf_env(tf_vars)
        result = await self._run(TerraformAction.APPLY, cmd, env=env)
        return TerraformResult(action=TerraformAction.APPLY, result=result)
    
    async def workspace(
        self,
        action: str,
        name: Optional[str]=None,
    ) -> TerraformResult:
        """
        Manages Terraform workspaces.

        Args:
            action: Workspace action (e.g., "select", "new", "delete", etc.).
            name: Optional workspace name for the action.

        Returns:
            TerraformResult: Contains the action performed and execution results.
        """
        cmd = ["terraform", "workspace", action]
        if name:
            cmd.append(name)

        result = await self._run(TerraformAction.WORKSPACE, cmd)
        return TerraformResult(action=TerraformAction.WORKSPACE, result=result)

    async def output(self) -> TerraformResult:
        """
        Runs the `terraform output` command and parses the output into JSON.

        Returns:
            TerraformResult: Contains the action performed, execution results, and parsed outputs.
        """
        cmd = ["terraform", "output", "-json"]
        result = await self._run(TerraformAction.OUTPUT, cmd)
        outputs = parse_terraform_output(result.stdout)
        return TerraformResult(action=TerraformAction.OUTPUT, result=result, outputs=outputs)

    async def _run(
        self,
        action: TerraformAction,
        cmd: List[str],
        env: Optional[Dict[str, str]]=None,
    ) -> ExecutionResult:
        """
        Executes a Terraform command and handles sensitive data masking.

        Args:
            action: The Terraform action being performed (e.g., INIT, PLAN, APPLY).
            cmd: The command to execute as a list of arguments.
            env: Optional environment variables specific to this command.

        Returns:
            ExecutionResult: The execution result including status, stdout, and stderr.
        """
        full_envs = dict(self._env)
        if env:
            full_envs.update(env)
        full_envs.update(self._all_sensitive)
        
        try:
            return await self._executer.execute(cmd, env=full_envs, cwd=self._base_cwd, mask=True)
        except Exception as e:
            logger.error(f"Error executing terraform {action.value} command: {e}", exc_info=True)
            raise TerraformExecutionError(action.value, str(e))
    
    def _prepare_tf_env(self, tf_vars: Optional[Dict[str, str]]) -> Dict[str, str]:
        if not tf_vars:
            return {}

        env = {}
        for k, v in tf_vars.items():
            if k.startswith("TF_VAR_"):
                env[k] = v
                continue
            env[f"TF_VAR_{k}"] = v
        return env

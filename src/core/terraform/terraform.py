from typing import Dict, Any, List, Optional, AsyncGenerator
from pathlib import Path

import logging

from core.executer.executer import IsolateExecuter
from core.providers.base import BaseProvider
from .exceptions import TerraformExecutionError
from .types import TerraformAction
from .utils import parse_terraform_output

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
        provider: Optional[BaseProvider] = None,
        base_env: Optional[Dict[str, str]] = None,
        sensitive: Optional[Dict[str, str]] = None,
    ):
        """
        Initializes the Terraform object.

        Args:
            base_cwd (Path): The base directory where Terraform commands will run.
            executer (IsolateExecuter): An executor implementing the CommandExecuter interface for running commands.
            provider (Optional[BaseProvider]): Optional provider for injecting authorization credentials (e.g., AWS, GCP).
            base_env (Optional[Dict[str, str]]): Base environment variables required for Terraform execution.
            sensitive (Optional[Dict[str, str]]): Additional sensitive variables that will be passed to Terraform but masked in logs.

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
        self._env.update(self._all_sensitive)

        processor = getattr(self._executer, "_processor", None)
        if sensitive and processor:
            for val in filter(None, sensitive.values()):
                processor.sensitive(val)
                
    async def _run_command(
        self,
        action: TerraformAction,
        cmd: List[str],
        env: Optional[Dict[str, str]] = None
    ) -> AsyncGenerator[str, None]:
        """
        Executes a Terraform command and yields output lines.

        Args:
            action (TerraformAction): The Terraform action being executed.
            cmd (List[str]): The command and its arguments to execute.
            env (Optional[Dict[str, str]]): Additional environment variables.

        Yields:
            str: Lines of output from the Terraform command.

        Raises:
            TerraformExecutionError: If the command execution fails.
        """
        try:
            async for line in self._executer.execute_stream(
                cmd,
                env=env or self._env,
                cwd=self._base_cwd,
                mask=True
            ):
                yield line
        except Exception as e:
            logger.error(f"Error executing terraform {action.value} command: {e}", exc_info=True)
            raise TerraformExecutionError(action.value, str(e))
    
    async def init(self, args: Optional[List[str]] = None) -> AsyncGenerator[str, None]:
        """
        Runs the `terraform init` command and yields output lines as they become available.

        Args:
            args (Optional[List[str]]): Additional command-line arguments to pass to `terraform init`.

        Yields:
            Lines of the output as they are generated by Terraform.

        Raises:
            TerraformExecutionError: If the command execution fails.
        """
        cmd = ["terraform", "init"]
        if args:
            cmd.extend(args)
        
        async for line in self._run_command(TerraformAction.INIT, cmd):
            yield line
    
    async def plan(
        self,
        args: Optional[List[str]] = None,
        tf_vars: Optional[Dict[str, str]] = None,
    ) -> AsyncGenerator[str, None]:
        """
        Runs `terraform plan` and yields output lines as they become available.

        Args:
            args (Optional[List[str]]): Additional command-line arguments for `terraform plan`.
            tf_vars (Optional[Dict[str, str]]): Key-value pairs representing Terraform input variables (TF_VAR_*).

        Yields:
            Lines of the output as they are generated by Terraform.

        Raises:
            TerraformExecutionError: If the command execution fails.
        """
        cmd = ["terraform", "plan", "-input=false"]
        if args:
            cmd.extend(args)
        env = self._prepare_tf_env(tf_vars)

        async for line in self._run_command(TerraformAction.PLAN, cmd, env=env):
            yield line
    
    async def apply(
        self,
        args: Optional[List[str]] = None,
        tf_vars: Optional[Dict[str, str]] = None,
        auto_approve: bool = True
    ) -> AsyncGenerator[str, None]:
        """
        Runs the `terraform apply` command and yields output lines as they become available.

        Args:
            args (Optional[List[str]]): Additional command-line arguments for `terraform apply`.
            tf_vars (Optional[Dict[str, str]]): Terraform input variables to inject into the execution environment.
            auto_approve (bool): Skips confirmation prompts by adding `-auto-approve`.

        Yields:
            Lines of the output as they are generated by Terraform.

        Raises:
            TerraformExecutionError: If the command execution fails.
        """
        cmd = ["terraform", "apply", "-input=false"]
        if auto_approve:
            cmd.append("-auto-approve")
        if args:
            cmd.extend(args)

        env = self._prepare_tf_env(tf_vars)
        async for line in self._run_command(TerraformAction.APPLY, cmd, env=env):
            yield line
    
    async def workspace(
        self,
        action: str,
        name: Optional[str] = None,
    ) -> AsyncGenerator[str, None]:
        """
        Manages Terraform workspaces and yields output lines as they become available.

        Args:
            action (str): Workspace action (e.g., "select", "new", "delete", etc.).
            name (Optional[str]): Optional workspace name for the action.

        Yields:
            Lines of the output as they are generated by Terraform.

        Raises:
            TerraformExecutionError: If the command execution fails.
        """
        cmd = ["terraform", "workspace", action]
        if name:
            cmd.append(name)

        async for line in self._run_command(TerraformAction.WORKSPACE, cmd):
            yield line
    
    async def output(self) -> Dict[str, Any]:
        """
        Runs the `terraform output` command, parses the JSON output, and returns it as a dictionary.

        Returns:
            Dict[str, Any]: Parsed Terraform outputs.

        Raises:
            TerraformExecutionError: If the command execution fails or parsing fails.
        """
        cmd = ["terraform", "output", "-json"]

        output_lines = []
        try:
            async for line in self._run_command(TerraformAction.OUTPUT, cmd):
                output_lines.append(line)
        except Exception as e:
            logger.error(f"Error executing terraform output command: {e}", exc_info=True)
            raise TerraformExecutionError(TerraformAction.OUTPUT.value, str(e))
        
        stdout = ''.join(output_lines).strip()
        try:
            outputs = parse_terraform_output(stdout, logger)
            return outputs
        except Exception as e:
            logger.error(f"Error parsing terraform output: {e}", exc_info=True)
            raise TerraformExecutionError(TerraformAction.OUTPUT.value, f"Parsing error: {e}")
    
    def _prepare_tf_env(self, tf_vars: Optional[Dict[str, str]]) -> Dict[str, str]:
        """
        Prepares the environment variables for Terraform commands based on tf_vars.

        Args:
            tf_vars (Optional[Dict[str, str]]): Terraform input variables.

        Returns:
            Dict[str, str]: Environment variables formatted as TF_VAR_*.
        """
        env = {**self._env}

        if tf_vars:
            for k, v in tf_vars.items():
                if k.startswith("TF_VAR_"):  
                    env[k] = v
                    continue
                env[f"TF_VAR_{k}"] = v
        return env

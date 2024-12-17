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
    """Terraform object"""
    def __init__(
        self,
        base_cwd: Path,
        executer: IsolateExecuter,
        provider: Optional[BaseProvider]=None,
        base_env: Optional[Dict[str, str]]=None,
        sensitive: Optional[Dict[str, str]]=None,
    ):
        """
        :param base_cwd: The directory where Terraform commands will be executed
        :param executer: An instance of a class implementing the CommandExecuter interface
        :param provider: An optional provider instance for authorization
        :param base_env: Base environment variables (e.g., AWS credentials or other)
        :param sensitive: Additional sensitive data to be masked
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
        """Invoke 'terraform init' with args"""
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
        """Invoke 'terraform plan' with args"""
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
        """Invoke 'terraform apply' with args"""
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
        """Invoke 'terraform workspace' commands with args"""
        cmd = ["terraform", "workspace", action]
        if name:
            cmd.append(name)

        result = await self._run(TerraformAction.WORKSPACE, cmd)
        return TerraformResult(action=TerraformAction.WORKSPACE, result=result)

    async def output(self) -> TerraformResult:
        """Invoke 'terraform output'"""
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

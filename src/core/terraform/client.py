from typing import Optional, Dict, Union
from pathlib import Path

import logging
import json

from ..types.terraform import (
    TerraformPlanResult,
    TerraformApplyResult,
    TerraformFormatResult,
    TerraformInitResult,
    TerraformVariable,
    TerraformWorkspaceResult,
)
from ..executer.command import IsolateExecuter

logging = logging.getLogger(__name__)

class TerraformClient:
    """Client for executing Terraform commands"""

    def __init__(
        self,
        working_dir: Union[str, Path],
        executer: Optional[IsolateExecuter] = None,
        variables: Optional[Dict[str, TerraformVariable]] = None
    ):
        """
        Initialize Terraform client
        
        Args:
            working_dir: Directory containing Terraform configuration
            executer: Optional custom command executer
            variables: Optional Terraform variables
        """
        self._variables = variables or {}
        self.working_dir = Path(working_dir)
        self._executer = executer or IsolateExecuter()

    async def _run_command(self, *args: str) -> tuple[bool, str, Optional[str]]:
        """Run Terraform command and return result"""
        cmd = ["terraform", *args]
        result = await self._executer.execute(
            cmd=cmd,
            cwd=self.working_dir
        )
        return (
            result.status == 0,
            result.stdout,
            result.stderr if result.status != 0 else None
        )

    async def plan(
        self, 
        output_file: Optional[Union[str, Path]] = None,
        detailed_exitcode: bool = False
    ) -> TerraformPlanResult:
        """
        Run terraform plan
        
        Args:
            output_file: Optional file to save plan
            detailed_exitcode: Use detailed exit codes (0 - no changes, 1 - error, 2 - changes present)
        """
        args = ["plan", "-no-color"]
        if self._variables:
            for var_name, var_def in self._variables.items():
                var_value = json.dumps(var_def["value"])
                args.extend(["-var", f"{var_name}={var_value}"])
        if output_file:
            args.extend(["-out", str(output_file)])
        if detailed_exitcode:
            args.append("-detailed-exitcode")

        success, stdout, error = await self._run_command(*args)
        has_changes = False
        if detailed_exitcode:
            has_changes = not success
            success = error is None
        result: TerraformPlanResult = {
            "success": success,
            "changes": has_changes or "No changes." not in stdout,
            "output": stdout,
        }
        if error:
            result["error"] = error
        return result

    async def init(
        self,
        backend_config: Optional[Dict[str, str]] = None,
        reconfigure: bool = False,
        upgrade: bool = False,
    ) -> TerraformInitResult:
        """
        Run terraform init
        
        Args:
            backend_config: Optional backend configuration
            reconfigure: Reconfigure backend, ignoring any saved configuration
            upgrade: Update all modules and plugins to latest version
        """
        args = ["init", "-no-color"]
        if reconfigure:
            args.append("-reconfigure")
        if upgrade:
            args.append("-upgrade")
        if backend_config:
            for key, value in backend_config.items():
                args.extend(["-backend-config", f"{key}={value}"])

        success, stdout, error = await self._run_command(*args)
        result: TerraformInitResult = {
            "success": success,
            "output": stdout,
        }
        if error:
            result["error"] = error
        return result

    async def workspace_list(self) -> list[str]:
        """List all workspaces"""
        success, stdout, _ = await self._run_command("workspace", "list")
        if success:
            workspaces = []
            for line in stdout.splitlines():
                line = line.strip()
                if line:
                    workspaces.append(line.replace('* ', ''))
            return workspaces
        return []

    async def workspace_show(self) -> str:
        """Show current workspace"""
        success, stdout, _ = await self._run_command("workspace", "show")
        return stdout.strip() if success else "default"

    async def workspace_select(self, name: str) -> TerraformWorkspaceResult:
        """
        Select a workspace
        
        Args:
            name: Name of workspace to select
        """
        success, stdout, error = await self._run_command("workspace", "select", name)
        
        result: TerraformWorkspaceResult = {
            "success": success,
            "name": name,
            "output": stdout,
        }
        if error:
            result["error"] = error
        return result

    async def workspace_new(self, name: str) -> TerraformWorkspaceResult:
        """
        Create a new workspace
        
        Args:
            name: Name of workspace to create
        """
        success, stdout, error = await self._run_command("workspace", "new", name)
        
        result: TerraformWorkspaceResult = {
            "success": success,
            "name": name,
            "output": stdout,
        }
        if error:
            result["error"] = error
        return result

    async def workspace_delete(self, name: str, force: bool = False) -> TerraformWorkspaceResult:
        """
        Delete a workspace
        
        Args:
            name: Name of workspace to delete
            force: Force deletion even if workspace is not empty
        """
        args = ["workspace", "delete"]
        if force:
            args.append("-force")
        args.append(name)
        
        success, stdout, error = await self._run_command(*args)
        result: TerraformWorkspaceResult = {
            "success": success,
            "name": name,
            "output": stdout,
        }
        if error:
            result["error"] = error
        return result

    async def apply(
        self,
        auto_approve: bool = False,
        plan_file: Optional[Union[str, Path]] = None
    ) -> TerraformApplyResult:
        """
        Run terraform apply
        
        Args:
            auto_approve: Skip interactive approval
            plan_file: Optional saved plan file to apply
        """
        args = ["apply", "-no-color"]
        if auto_approve:
            args.append("-auto-approve")
        if plan_file:
            args.append(str(plan_file))
        elif self._variables:
            for var_name, var_def in self._variables.items():
                var_value = json.dumps(var_def["value"])
                args.extend(["-var", f"{var_name}={var_value}"])

        success, stdout, error = await self._run_command(*args)
        result: TerraformApplyResult = {
            "success": success,
            "output": stdout,
        }
        if error:
            result["error"] = error
        elif success:
            success, output, error = await self._run_command("output", "-json")
            if success:
                result["outputs"] = json.loads(output)
        return result

    async def fmt(
        self,
        check: bool = False,
        write: bool = True,
        recursive: bool = True
    ) -> TerraformFormatResult:
        """
        Run terraform fmt
        
        Args:
            check: Check if formatting is needed
            write: Write formatting changes to files
            recursive: Format files in subdirectories
        """
        args = ["fmt"]
        if check:
            args.append("-check")
        if not write:
            args.append("-write=false")
        if recursive:
            args.append("-recursive")
            
        success, stdout, error = await self._run_command(*args)
        changed_files = [
            file.strip() 
            for file in stdout.split("\n") 
            if file.strip()
        ]
        result: TerraformFormatResult = {
            "success": success or (check and changed_files),
            "changed": bool(changed_files),
            "files": changed_files
        }
        if error:
            result["error"] = error
        return result

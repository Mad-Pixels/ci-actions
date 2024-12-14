from typing import Optional, Dict, Union
from pathlib import Path

import logging
import json

from ..types.terraform import (
    TerraformPlanResult,
    TerraformApplyResult,
    TerraformFormatResult,
    TerraformVariable
)
from ..executer.command import IsolateExecuter

logging = logging.getLogger(__name__)

class TerraformClient:
    """Terraform client"""

    def __init__(
        self,
        cwd: Union[str, Path],
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
        self.cwd = Path(cwd)
        self._variables = variables or {}
        self._executer = executer or IsolateExecuter()
    
    async def _run_command(self, *args: str) -> tuple[bool, str, Optional[str]]:
        """Run terreform command"""
        cmd = ["terraform", *args]
        res = await self._executer.execute(
            cmd=cmd,
            cwd=self.cwd
        )
        return (
            res.status == 0,
            res.stdout,
            res.stderr if res.status != 0 else None
        )
    
    async def plan(
        self,
        output: Optional[Union[str, Path]] = None,
        detailed_status: bool = False
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
        if output:
            args.extend(["-out", str(output)])
        if detailed_status:
            args.append("-detailed-exitcode")

        success, stdout, error = await self._run_command(*args)
        has_changes = False
        if detailed_status:
            has_changes = not success
            success = error is None
        
        result: TerraformPlanResult = {
            "success": success,
            "changes": has_changes or "No changes" not in stdout,
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

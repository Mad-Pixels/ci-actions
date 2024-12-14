from typing import Optional, Dict, Sequence
from pathlib import Path
import asyncio
import os

from .base import BaseExecuter, ExecutionResult

class SubprocessExecuter(BaseExecuter):
    """Base subprocess for command execution"""

    async def _run_command(
            self, 
            cmd: Sequence[str], 
            env: Optional[Dict[str, str]] = None, 
            cwd: Optional[Path] = None
    ) -> ExecutionResult:
        process = await asyncio.create_subprocess_exec(
            *cmd,
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
            env=env,
            cwd=cwd
        )
        stdout, stderr = await process.communicate()
        return ExecutionResult(
            status=process.returncode,
            stdout=stdout.decode(),
            stderr=stderr.decode()
        )
    

class IsolateExecuter(SubprocessExecuter):
    """Isolate subprocess for command execution"""

    def __init__(self):
        super().__init__()
        self._env_backup = None

    async def execute(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None,
        mask: bool = False,
    ) -> ExecutionResult:
        if env:
            self._env_backup = dict(os.environ)

        try:
            result = await self._run_command(cmd, env, cwd)
            if mask and self._output_processor:
                result.masked_stdout = self._output_processor.mask(result.stdout)
                result.masked_stderr = self._output_processor.mask(result.stderr)
            return result

        finally:
            if self._env_backup:
                os.environ.clear()
                os.environ.update(self._env_backup)
                self._env_backup = None

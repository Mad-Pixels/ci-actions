from typing import Optional, Dict, Sequence
from pathlib import Path

import asyncio
import logging
import os

logging = logging.getLogger(__name__)

from .base import BaseExecuter, ExecutionResult

class SubprocessExecuter(BaseExecuter):
    """Base subprocess for command execution"""

    async def _run_command(
            self, 
            cmd: Sequence[str], 
            env: Optional[Dict[str, str]] = None, 
            cwd: Optional[Path] = None
    ) -> ExecutionResult:
        logging.info(f"Creating subprocess for command: {' '.join(cmd)}")
        logging.info(f"Working directory: {cwd}")
        logging.info(f"Environment variables: {list(env.keys()) if env else None}")
        
        try:
            process = await asyncio.create_subprocess_exec(
                *cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                env=env,
                cwd=cwd
            )
            logging.info(f"Process created with pid: {process.pid}")
            
            stdout, stderr = await process.communicate()
            logging.info(f"Process completed with return code: {process.returncode}")
            
            return ExecutionResult(
                status=process.returncode,
                stdout=stdout.decode(),
                stderr=stderr.decode()
            )
        except Exception as e:
            logging.error(f"Error in subprocess execution: {e}")
            raise
    

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

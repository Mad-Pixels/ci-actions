from typing import Dict, Sequence, Optional
from pathlib import Path

import asyncio
import os

from .base import BaseExecuter, CommandExecuter, ExecutionResult
from .exceptions import SubprocessError
from .utils import override_env, read_stream

class SubprocessExecuter(BaseExecuter, CommandExecuter):
    """Subprocess-based command executor"""

    async def execute(
        self, 
        cmd: Sequence[str], 
        env: Dict[str, str]={}, 
        cwd: Optional[Path]=None, 
        mask: bool=False,
    ) -> ExecutionResult:
        self._logger.debug(f"Executing command: {' '.join(cmd)}")
        self._validate_inputs(cmd, env, cwd)

        result = await self._run_command(cmd, env, cwd)
        if mask and self._processor:
            self._logger.debug("Masking output...")
            return ExecutionResult(
                status=result.status,
                stdout=result.stdout,
                stderr=result.stderr,
                masked_stdout=self._processor.mask(result.stdout),
                masked_stderr=self._processor.mask(result.stderr)
            )
        return result

    async def _run_command(
        self, 
        cmd: Sequence[str], 
        env: Dict[str, str], 
        cwd: Optional[Path]
    ) -> ExecutionResult:
        self._logger.debug(f"Creating subprocess: {' '.join(cmd)}")
        try:
            process = await asyncio.create_subprocess_exec(
                *cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                env={**os.environ, **env},
                cwd=cwd
            )
            stdout_task = asyncio.create_task(
                read_stream(process.stdout, self._logger, "STDOUT", self._processor)
            )
            stderr_task = asyncio.create_task(
                read_stream(process.stderr, self._logger, "STDERR", self._processor) 
            )
            stdout, stderr = await asyncio.gather(stdout_task, stderr_task)
            await process.wait()

            stdout = stdout.strip()
            stderr = stderr.strip()

            if process.returncode != 0:
                raise SubprocessError(
                    cmd=' '.join(cmd),
                    returncode=process.returncode,
                    stderr=stderr,
                    stdout=stdout
                )
            return ExecutionResult(
                status=process.returncode,
                stdout=stdout,
                stderr=stderr,
                masked_stdout=self._processor.mask(stdout) if self._processor else None,
                masked_stderr=self._processor.mask(stderr) if self._processor else None
            )
        except asyncio.CancelledError:
            self._logger.warning("Command execution was cancelled")
            raise
        except Exception as e:
            self._logger.error(f"Subprocess execution error: {e}", exc_info=True)
            raise

class IsolateExecuter(SubprocessExecuter):
    """Isolater environment command executor"""

    async def execute(
        self,
        cmd: Sequence[str],
        env: Dict[str, str]={},
        cwd: Optional[Path]=None,
        mask: bool = False,
    ) -> ExecutionResult:
        self._logger.debug(f"Starting IsolateExecuter.execute")
        with override_env(env):
            return await super().execute(cmd, env, cwd, mask)

from typing import Dict, Sequence, Optional
from pathlib import Path

import asyncio
import os

from .base import BaseExecuter, CommandExecuter, ExecutionResult
from .exceptions import SubprocessError
from .utils import override_env, read_stream

class SubprocessExecuter(BaseExecuter, CommandExecuter):
    """
    Executes system commands in a subprocess.

    Features:
    - Supports environment variables and working directory customization.
    - Allows masking of sensitive data in command output (stdout/stderr).
    - Handles exceptions gracefully and raises SubprocessError on failure.

    Methods:
        execute: Executes a given command asynchronously and processes the result.
        _run_command: Internal method to create a subprocess and manage input/output.
    """

    async def execute(
        self, 
        cmd: Sequence[str], 
        env: Dict[str, str]={}, 
        cwd: Optional[Path]=None, 
        mask: bool=False,
    ) -> ExecutionResult:
        """
        Execute a system command in a subprocess.

        Args:
            cmd: The command to execute as a sequence of strings (e.g., ["ls", "-l"]).
            env: Environment variables to pass to the subprocess.
            cwd: Optional working directory for the subprocess.
            mask: If True, masks sensitive data in the output using the configured processor.

        Returns:
            ExecutionResult: An object containing the status, stdout, and stderr.

        Raises:
            SubprocessError: If the command fails with a non-zero exit code.
        """
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
        """
        Internal method to execute a command and capture output.

        Args:
            cmd: The command to execute.
            env: Environment variables for the subprocess.
            cwd: Working directory for the command execution.

        Returns:
            ExecutionResult: Captured stdout, stderr, and status code.

        Raises:
            SubprocessError: Raised when the command exits with a non-zero status.
            asyncio.CancelledError: If the execution task is cancelled.
            Exception: For other unexpected errors during execution.
        """
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
    """
    Specialized executor that isolates the command execution environment.

    Features:
    - Overrides environment variables during execution for isolation.
    - Inherits all functionality from SubprocessExecuter.

    Methods:
        execute: Executes a command while temporarily overriding environment variables.
    """

    async def execute(
        self,
        cmd: Sequence[str],
        env: Dict[str, str]={},
        cwd: Optional[Path]=None,
        mask: bool = False,
    ) -> ExecutionResult:
        """
        Execute a command with an isolated environment.

        Args:
            cmd: The command to execute as a sequence of strings.
            env: Environment variables to override for this execution.
            cwd: Optional working directory for the subprocess.
            mask: Whether to mask sensitive data in the output.

        Returns:
            ExecutionResult: Result of the command execution.

        Notes:
            The environment variables are temporarily overridden using `override_env`.
        """
        self._logger.debug(f"Starting IsolateExecuter.execute")
        with override_env(env):
            return await super().execute(cmd, env, cwd, mask)

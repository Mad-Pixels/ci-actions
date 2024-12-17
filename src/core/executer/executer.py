from typing import Dict, Sequence, Optional, AsyncGenerator
from pathlib import Path

import asyncio
import os

from .base import BaseExecuter, CommandExecuter, ExecutionResult
from .utils import override_env, stream_lines
from .exceptions import SubprocessError

class SubprocessExecuter(BaseExecuter, CommandExecuter):
    """
    Executes system commands in a subprocess.

    Features:
    - Supports environment variables and working directory customization.
    - Masks sensitive data in command output (stdout/stderr).
    - Handles exceptions gracefully and raises SubprocessError on failure.
    """

    async def execute_stream(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None,
        mask: bool = False,
    ) -> AsyncGenerator[str, None]:
        """
        Execute a command and yield stdout lines as they become available.

        Args:
            cmd (Sequence[str]): The command to execute as a sequence of strings.
            env (Optional[Dict[str, str]]): Environment variables to pass.
            cwd (Optional[Path]): Working directory for the command.
            mask (bool): Whether to mask sensitive data in the output.

        Yields:
            str: Lines of the command's stdout in real-time.

        Raises:
            SubprocessError: If the command fails with a non-zero exit code.
        """
        self._logger.debug(f"Executing command (stream mode): {' '.join(cmd)}")
        env = env or {}
        self._validate_inputs(cmd, env, cwd)

        full_env = {**os.environ, **env}
        self._logger.debug(f"PATH: {full_env.get('PATH')}")

        try:
            process = await asyncio.create_subprocess_exec(
                *cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                env=full_env,
                cwd=cwd
            )

            async for line in stream_lines(process.stdout, self._logger, "STDOUT", self._processor):
                masked_line = self._processor.mask(line) if mask and self._processor else line
                yield masked_line

            returncode = await process.wait()
            stderr = await process.stderr.read()
            stderr_decoded = stderr.decode(errors="replace").strip()

            if returncode != 0:
                masked_stderr = self._processor.mask(stderr_decoded) if mask and self._processor else stderr_decoded
                self._logger.error(
                    f"Command '{' '.join(cmd)}' failed with return code {returncode}: {masked_stderr}"
                )
                raise SubprocessError(
                    cmd=' '.join(cmd),
                    returncode=returncode,
                    stderr=masked_stderr,
                    stdout=""
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
    """

    async def execute_stream(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None,
        mask: bool = False,
    ) -> AsyncGenerator[str, None]:
        """
        Execute a command with an isolated environment and yield stdout lines as they become available.

        Args:
            cmd (Sequence[str]): The command to execute as a sequence of strings.
            env (Optional[Dict[str, str]]): Environment variables to override for this execution.
            cwd (Optional[Path]): Working directory for the command.
            mask (bool): Whether to mask sensitive data in the output.

        Yields:
            str: Lines of the command's stdout in real-time.

        Raises:
            SubprocessError: If the command fails with a non-zero exit code.
        """
        self._logger.debug("Starting IsolateExecuter.execute_stream")
        env = env or {}
        with override_env(env):
            async for line in super().execute_stream(cmd, env=env, cwd=cwd, mask=mask):
                yield line

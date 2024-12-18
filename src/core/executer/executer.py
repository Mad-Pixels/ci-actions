from typing import Dict, List, Sequence, Optional, Union, AsyncGenerator
from pathlib import Path

import asyncio
import os

from .base import BaseExecuter, CommandExecuter, ExecutionResult
from .exceptions import SubprocessError, CommandExecutionError
from .utils import override_env, stream_lines


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
    ) -> AsyncGenerator[Union[str, ExecutionResult], None]:
        """
        Executes a command with real-time streaming output and returns final execution result.

        This method streams command output line by line in real-time and provides a final
        ExecutionResult object after command completion. The streaming allows processing
        large outputs without memory issues while maintaining access to the complete
        execution data.

        Args:
            cmd: Command to execute as a sequence of strings.
            env: Optional environment variables to pass to the command.
            cwd: Optional working directory for command execution.
            mask: Whether to mask sensitive data in the output.

        Yields:
            During execution: Individual lines of stdout/stderr as strings, prefixed with 
                            "[stdout] " or "[stderr] " accordingly
            After completion: ExecutionResult containing complete stdout, stderr and execution status

        Raises:
            asyncio.CancelledError: If the command execution is cancelled.
            Exception: For other execution errors (OS errors, etc).

        Example:
            async for output in executer.execute_stream(["ls", "-la"]):
                if isinstance(output, str):
                    print(f"Got line: {output}")
                else:
                    print(f"Command finished with status: {output.status}")
        """
        self._logger.debug(f"Executing command (stream mode): {' '.join(cmd)}")
        env = env or {}
        self._validate_inputs(cmd, env, cwd)

        stdout_lines: List[str] = []
        stderr_lines: List[str] = []
        masked_stdout_lines: List[str] = []
        masked_stderr_lines: List[str] = []

        queue: asyncio.Queue = asyncio.Queue()
        async def reader(stream: asyncio.StreamReader, name: str):
            try:
                async for line in stream_lines(stream, self._logger, name, self._processor):
                    await queue.put((name, line))
            except Exception as e:
                await queue.put(("error", e))

        try:
            process = await asyncio.create_subprocess_exec(
                *cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                env={**os.environ, **env},
                cwd=cwd
            )
            stdout_task = asyncio.create_task(reader(process.stdout, "STDOUT"))
            stderr_task = asyncio.create_task(reader(process.stderr, "STDERR"))

            while True:
                try:
                    name, content = await queue.get()
                    if name == "error":
                        self._logger.error(f"Error processing stream: {content}", exc_info=True)
                        process.kill()
                        await process.wait()
                        raise CommandExecutionError(str(content), cmd=' '.join(cmd))

                    is_stdout = name == "STDOUT"
                    stream_type = "stdout" if is_stdout else "stderr"
                    masked_line = self._processor.mask(content) if mask and self._processor else content

                    if is_stdout:
                        stdout_lines.append(content)
                        if mask:
                            masked_stdout_lines.append(masked_line)
                    else:
                        stderr_lines.append(content)
                        if mask:
                            masked_stderr_lines.append(masked_line)

                    yield f"[{stream_type}] {masked_line if mask else content}"

                except asyncio.CancelledError:
                    self._logger.warning("Command execution was cancelled")
                    process.kill()
                    await process.wait()
                    raise
                except CommandExecutionError:
                    raise
                except Exception as e:
                    self._logger.error(f"Subprocess execution error: {e}", exc_info=True)
                    process.kill()
                    await process.wait()
                    raise CommandExecutionError(str(e), cmd=' '.join(cmd)) from e

                if stdout_task.done() and stderr_task.done() and queue.empty():
                    break

            status = await process.wait()
            yield ExecutionResult(
                status=status,
                stdout=''.join(stdout_lines),
                stderr=''.join(stderr_lines),
                masked_stdout=''.join(masked_stdout_lines) if mask else None,
                masked_stderr=''.join(masked_stderr_lines) if mask else None
            )

        except asyncio.CancelledError:
            self._logger.warning("Command execution was cancelled")
            process.kill()
            await process.wait()
            raise
        except SubprocessError:
            raise
        except CommandExecutionError:
            raise
        except Exception as e:
            self._logger.error(f"Subprocess execution error: {e}", exc_info=True)
            process.kill()
            await process.wait()
            raise CommandExecutionError(str(e), cmd=' '.join(cmd)) from e

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
    ) -> AsyncGenerator[Union[str, ExecutionResult], None]:
        """
        Execute a command with an isolated environment and yield stdout lines as they become available.

        Args:
            cmd (Sequence[str]): The command to execute as a sequence of strings.
            env (Optional[Dict[str, str]]): Environment variables to override for this execution.
            cwd (Optional[Path]): Working directory for the command.
            mask (bool): Whether to mask sensitive data in the output.

        Yields:
            Union[str, ExecutionResult]: Either a line of output (prefixed with stream type)
                                      or final execution result.

        Raises:
            SubprocessError: If the command fails with a non-zero exit code.
        """
        self._logger.debug("Starting IsolateExecuter.execute_stream")
        env = env or {}
        async with override_env(env):
            async for line in super().execute_stream(cmd, env=env, cwd=cwd, mask=mask):
                yield line

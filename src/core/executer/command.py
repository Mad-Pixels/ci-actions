"""Command executor implementations"""
from typing import Optional, Dict, Sequence
from contextlib import contextmanager
from pathlib import Path
import asyncio
import logging
import os

from .base import BaseExecuter, CommandExecuter, ExecutionResult
from .exceptions import SubprocessError, CommandExecutionError
from .masking import OutputMasker
from .validation import validate_env

logger = logging.getLogger(__name__)

@contextmanager
def backup_env(env: Optional[Dict[str, str]] = None):
    """
    Context manager for backing up and restoring environment variables
    
    Args:
        env: New environment variables to set
    """
    original_env = dict(os.environ)
    try:
        if env:
            sanitized_env = validate_env(env)
            os.environ.update(sanitized_env)
        yield
    finally:
        os.environ.clear()
        os.environ.update(original_env)

async def _read_stream(
    stream: asyncio.StreamReader,
    logger: logging.Logger,
    stream_name: str
) -> str:
    """
    Read from stream asynchronously with logging
    
    Args:
        stream: Stream to read from
        logger: Logger instance
        stream_name: Stream name for logging
        
    Returns:
        Read data as string
    """
    output = []
    try:
        while True:
            line = await stream.readline()
            if not line:
                break
            decoded_line = line.decode(errors='replace')
            logger.debug(f"[{stream_name}] {decoded_line.strip()}")
            output.append(decoded_line)
        return ''.join(output)
    except Exception as e:
        logger.error(f"Error reading from {stream_name} stream: {e}")
        raise CommandExecutionError(f"Stream reading error: {e}")

class SubprocessExecuter(BaseExecuter, CommandExecuter):
    """Subprocess-based command executor"""

    async def execute(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None,
        mask: bool = False,
    ) -> ExecutionResult:
        """
        Execute command
        
        Args:
            cmd: Command sequence
            env: Environment variables
            cwd: Working directory
            mask: Whether to mask output
            
        Returns:
            Execution result
            
        Raises:
            CommandExecutionError: On execution error
        """
        self._logger.debug(f"Executing command: {' '.join(cmd)}")
        self._validate_inputs(cmd, env, cwd)
        result = await self._run_command(cmd, env, cwd)
        
        if mask and self._output_processor:
            self._logger.debug("Masking output...")
            # Create new ExecutionResult with masked output
            return ExecutionResult(
                status=result.status,
                stdout=result.stdout,
                stderr=result.stderr,
                masked_stdout=self._output_processor.mask(result.stdout),
                masked_stderr=self._output_processor.mask(result.stderr)
            )
        return result

    async def _run_command(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None
    ) -> ExecutionResult:
        """
        Run command via subprocess
        
        Args:
            cmd: Command sequence
            env: Environment variables
            cwd: Working directory
            
        Returns:
            Execution result
            
        Raises:
            SubprocessError: On subprocess error
        """
        self._logger.debug(f"Creating subprocess: {' '.join(cmd)}")
        try:
            process = await asyncio.create_subprocess_exec(
                *cmd,
                stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.PIPE,
                env=env,
                cwd=cwd
            )

            # Read stdout and stderr in parallel
            stdout_task = asyncio.create_task(
                _read_stream(process.stdout, self._logger, "STDOUT")
            )
            stderr_task = asyncio.create_task(
                _read_stream(process.stderr, self._logger, "STDERR")
            )

            stdout, stderr = await asyncio.gather(stdout_task, stderr_task)
            await process.wait()

            if process.returncode != 0:
                raise SubprocessError(
                    cmd=' '.join(cmd),
                    returncode=process.returncode,
                    stderr=stderr
                )

            return ExecutionResult(
                status=process.returncode,
                stdout=stdout,
                stderr=stderr
            )
        except asyncio.CancelledError:
            self._logger.warning("Command execution was cancelled")
            raise
        except Exception as e:
            self._logger.error(f"Subprocess execution error: {e}", exc_info=True)
            raise

class IsolateExecuter(SubprocessExecuter):
    """Isolated environment command executor"""

    def __init__(self, output_processor: Optional[OutputMasker] = None):
        super().__init__(output_processor=output_processor)

    async def execute(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None,
        mask: bool = False,
    ) -> ExecutionResult:
        """
        Execute command in isolated environment
        
        Args:
            cmd: Command sequence
            env: Environment variables
            cwd: Working directory
            mask: Whether to mask output
            
        Returns:
            Execution result
        """
        self._logger.debug("Starting IsolateExecuter.execute")
        with backup_env(env):
            result = await super().execute(cmd, env, cwd, mask)
            return result
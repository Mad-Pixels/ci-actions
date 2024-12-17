from typing import Dict, Optional, Protocol, Sequence
from abc import ABC, abstractmethod
from dataclasses import dataclass
from pathlib import Path

import logging

@dataclass(frozen=True)
class ExecutionResult:
    """
    Represents the result of a command execution:
    - status: Exit code of the command (0 means success).
    - stdout: Standard output from the command.
    - stderr: Standard error output from the command.
    - masked_stdout: Optional masked version of stdout for sensitive data.
    - masked_stderr: Optional masked version of stderr for sensitive data.
    """
    status: int
    stdout: str
    stderr: str
    masked_stdout: Optional[str] = None
    masked_stderr: Optional[str] = None

    def __post_init__(self):
        if self.status < 0:
            raise ValueError("Status code cannot be negative")

class CommandExecuter(Protocol):
    """
    Protocol for executing commands:
    - execute: Executes the given command and returns ExecutionResult.
    
    Args:
        cmd: A sequence of command arguments to execute.
        env: Optional dictionary of environment variables.
        cwd: Optional working directory for the command execution.
        mask: Whether to mask sensitive output in stdout/stderr.
    """
    async def execute(
        self,
        cmd: Sequence[str],
        env: Dict[str, str]={},
        cwd: Optional[Path]=None,
        mask: bool = False,
    ) -> ExecutionResult:
        pass

class BaseExecuter(ABC):
    """
    Abstract base class for command executers. Handles common logic:
    - Input validation: Ensures commands, environment variables, and working directory are valid.
    - Logging: Provides a logger for child classes.

    Attributes:
        _processor: Optional processor for masking sensitive data in outputs.
        _logger: Logger instance for logging command execution details.
    """
    def __init__(self, processor=None):
        self._processor = processor
        self._logger = logging.Logger(f"executer.{self.__class__.__name__}")

    def _validate_inputs(
        self,
        cmd: Sequence[str],
        env: Dict[str, str],
        cwd: Optional[Path],
    ) -> None:
        """
        Validates the inputs for command execution.

        Args:
            cmd: A sequence of command arguments.
            env: A dictionary of environment variables.
            cwd: Optional working directory.
        
        Raises:
            Exception: If any input is invalid.
        """
        from .validater import validate_command, validate_env, validate_cwd

        try:
            validate_command(cmd)
            validate_env(env)
            validate_cwd(cwd)
        except Exception as e:
            self._logger.error(f"Input validation failed: {e}")
            raise
    
    @abstractmethod
    async def _run_command(
        self,
        cmd: Sequence[str],
        env: Dict[str, str],
        cwd: Optional[Path],
    ) -> ExecutionResult:
        """
        Abstract method to execute a command.

        Args:
            cmd: The command to execute.
            env: Environment variables to pass.
            cwd: Working directory for execution.

        Returns:
            ExecutionResult: The result of the executed command.
        """
        pass

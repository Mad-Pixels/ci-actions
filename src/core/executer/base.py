from typing import Dict, Optional, Protocol, Sequence, AsyncGenerator
from dataclasses import dataclass
from pathlib import Path
from abc import ABC

import logging

from .masker import OutputMasker

@dataclass(frozen=True)
class ExecutionResult:
    """
    Represents the result of a command execution.

    Attributes:
        status (int): Exit code of the command (0 means success).
        stdout (str): Standard output from the command.
        stderr (str): Standard error output from the command.
        masked_stdout (Optional[str]): Masked version of stdout for sensitive data.
        masked_stderr (Optional[str]): Masked version of stderr for sensitive data.
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
    Protocol for executing commands.

    Defines the `execute_stream` method which executes a command and yields output lines.
    """
    async def execute_stream(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None,
        mask: bool = False,
    ) -> AsyncGenerator[str, None]:
        pass

class BaseExecuter(ABC):
    """
    Abstract base class for command executers.

    Handles common functionalities such as input validation and logging.

    Attributes:
        _processor (Optional[OutputMasker]): Processor for masking sensitive data in outputs.
        _logger (logging.Logger): Logger instance for logging command execution details.
    """
    def __init__(self, processor: Optional['OutputMasker'] = None):
        self._processor = processor
        self._logger = logging.getLogger(f"executer.{self.__class__.__name__}")

    def _validate_inputs(
        self,
        cmd: Sequence[str],
        env: Dict[str, str],
        cwd: Optional[Path],
    ) -> None:
        """
        Validates the inputs for command execution.

        Args:
            cmd (Sequence[str]): Command arguments to execute.
            env (Dict[str, str]): Environment variables for the command.
            cwd (Optional[Path]): Working directory for command execution.
        
        Raises:
            CommandValidationError: If any input is invalid.
        """
        from .validate import validate_command, validate_env, validate_cwd

        try:
            validate_command(cmd)
            validate_env(env)
            validate_cwd(cwd)
        except Exception as e:
            self._logger.error(f"Input validation failed: {e}")
            raise

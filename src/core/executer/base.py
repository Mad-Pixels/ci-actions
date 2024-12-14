"""Base classes for command executor"""
from typing import Protocol, Optional, Dict, Sequence
from abc import ABC, abstractmethod
from dataclasses import dataclass
from pathlib import Path
import logging
import os

from .exceptions import CommandExecutionError
from .validation import validate_command, validate_env, validate_cwd

# Logging setup
LOG_LEVEL = os.getenv("LOG_LEVEL", "ERROR").upper()
logging.basicConfig(
    level=getattr(logging, LOG_LEVEL, logging.ERROR),
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)

@dataclass(frozen=True)
class ExecutionResult:
    """Immutable command execution result"""
    status: int
    stdout: str
    stderr: str
    masked_stdout: Optional[str] = None
    masked_stderr: Optional[str] = None
    
    def __post_init__(self):
        if self.status < 0:
            raise ValueError("Status code cannot be negative")

class CommandExecuter(Protocol):
    """Command executor protocol"""
    async def execute(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None,
        mask: bool = False,
    ) -> ExecutionResult:
        """Execute command with optional environment and working directory"""
        ...

class BaseExecuter(ABC):
    """Abstract base executor"""
    def __init__(self, output_processor=None):
        self._output_processor = output_processor
        # Initialize logger with class name
        self._logger = logging.getLogger(f"executer.{self.__class__.__name__}")

    def _validate_inputs(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None
    ) -> None:
        """
        Validate command execution inputs
        
        Args:
            cmd: Command sequence
            env: Environment variables
            cwd: Working directory
            
        Raises:
            CommandValidationError: If inputs are invalid
        """
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
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None
    ) -> ExecutionResult:
        """Run command implementation"""
        pass
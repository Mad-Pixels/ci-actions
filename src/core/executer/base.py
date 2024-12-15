from typing import Dict, Optional, Protocol, Sequence
from abc import ABC, abstractmethod
from dataclasses import dataclass
from pathlib import Path

import logging

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
        env: Dict[str, str]={},
        cwd: Optional[Path]=None,
        mask: bool = False,
    ) -> ExecutionResult:
        pass

class BaseExecuter(ABC):
    """Abstract base executer"""
    def __init__(self, processor=None):
        self._processor = processor
        self._logger = logging.Logger(f"executer.{self.__class__.__name__}")

    def _validate_inputs(
        self,
        cmd: Sequence[str],
        env: Dict[str, str],
        cwd: Optional[Path],
    ) -> None:
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
        cwd: Optional[Path]
    ) -> ExecutionResult:
        pass

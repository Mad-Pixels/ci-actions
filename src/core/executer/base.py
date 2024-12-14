from typing import Protocol, Optional, Dict, Sequence
from abc import ABC, abstractmethod
from dataclasses import dataclass
from pathlib import Path

import logging

logger = logging.getLogger(__name__)

@dataclass
class ExecutionResult:
    """Command execution result"""
    status: int
    stdout: str
    stderr: str
    masked_stdout: Optional[str] = None
    masked_stderr: Optional[str] = None

class CommandExecuter(Protocol):
    """Command protocol"""

    async def execute(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None,
        mask: bool = False,
    ) -> ExecutionResult:
        pass

class BaseExecuter(ABC):
    """Base executer object"""

    def __init__(self):
        self._output_processor = None
        self._env_manager = None

    @abstractmethod
    async def _run_command(
        self,
        cmd: Sequence[str],
        env: Optional[Dict[str, str]] = None,
        cwd: Optional[Path] = None
    ) -> ExecutionResult:
        pass

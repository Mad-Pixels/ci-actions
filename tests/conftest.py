from unittest.mock import AsyncMock

import pytest

from src.core.executer.executer import SubprocessExecuter, IsolateExecuter
from src.core.executer.base import ExecutionResult
from src.core.executer.masker import OutputMasker

@pytest.fixture
def executer():
    """Фикстура для создания экземпляра SubprocessExecuter"""
    return SubprocessExecuter()

@pytest.fixture
def isolate_executer():
    """Фикстура для создания экземпляра IsolateExecuter"""
    return IsolateExecuter()

@pytest.fixture
def mock_executer() -> IsolateExecuter:
    """Возвращает мокнутый IsolateExecuter"""
    executer = AsyncMock(spec=IsolateExecuter)
    executer.execute.return_value = ExecutionResult(
        status=0,
        stdout="default stdout",
        stderr="default stderr"
    )
    return executer

@pytest.fixture
def masker():
    """Фикстура для создания экземпляра OutputMasker."""
    return OutputMasker()

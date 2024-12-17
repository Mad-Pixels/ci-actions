from unittest.mock import AsyncMock, Mock

import pytest

from core.executer.executer import SubprocessExecuter, IsolateExecuter
from core.executer.base import ExecutionResult
from core.executer.masker import OutputMasker


@pytest.fixture
def executer() -> SubprocessExecuter:
    """
    Fixture to create an instance of SubprocessExecuter.
    """
    return SubprocessExecuter()


@pytest.fixture
def isolate_executer() -> IsolateExecuter:
    """
    Fixture to create an instance of IsolateExecuter.
    """
    return IsolateExecuter()


@pytest.fixture
def mock_executer() -> IsolateExecuter:
    """
    Fixture to return a mocked IsolateExecuter.

    The mocked IsolateExecuter replaces the `execute_stream` method with a default
    AsyncMock that yields predefined output lines. This allows testing the Terraform
    commands without actually executing real system commands.

    Returns:
        A mocked instance of IsolateExecuter.
    """
    async def async_iter(self):
        return self

    executer = AsyncMock(spec=IsolateExecuter)
    executer.execute_stream.return_value.__aiter__ = async_iter
    return executer

@pytest.fixture
def masker() -> OutputMasker:
    """
    Fixture to create an instance of OutputMasker.
    """
    return OutputMasker()

@pytest.fixture
def async_gen_factory():
    """
    Fixture для создания асинхронных генераторов с заданными строками вывода.
    """
    def _factory(lines):
        async def _gen(*args, **kwargs):
            for line in lines:
                yield line
        return _gen
    return _factory


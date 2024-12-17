from unittest.mock import AsyncMock

import pytest

from src.core.executer.executer import SubprocessExecuter, IsolateExecuter
from src.core.executer.base import ExecutionResult
from src.core.executer.masker import OutputMasker

@pytest.fixture
def executer() -> SubprocessExecuter:
    """
    Fixture to create an instance of SubprocessExecuter.

    This fixture is used to test the behavior of the SubprocessExecuter
    without mocking its methods.
    """
    return SubprocessExecuter()

@pytest.fixture
def isolate_executer() -> IsolateExecuter:
    """
    Fixture to create an instance of IsolateExecuter.

    IsolateExecuter adds isolated environment handling around SubprocessExecuter.
    This fixture provides a real instance for testing purposes.
    """
    return IsolateExecuter()

@pytest.fixture
def mock_executer() -> IsolateExecuter:
    """
    Fixture to return a mocked IsolateExecuter.

    The mocked IsolateExecuter replaces the `execute` method with a default
    AsyncMock that returns a successful ExecutionResult by default.
    This allows testing the Terraform commands without actually executing
    real system commands.

    Returns:
        A mocked instance of IsolateExecuter.
    """
    executer = AsyncMock(spec=IsolateExecuter)
    executer.execute.return_value = ExecutionResult(
        status=0,
        stdout="default stdout",
        stderr="default stderr"
    )
    return executer

@pytest.fixture
def masker() -> OutputMasker:
    """
    Fixture to create an instance of OutputMasker.

    OutputMasker is used to mask sensitive data in command outputs.
    This fixture provides a reusable instance for tests that involve masking.

    Returns:
        An instance of OutputMasker.
    """
    return OutputMasker()

from unittest.mock import AsyncMock, MagicMock, patch
import pytest
import asyncio
from pathlib import Path

from core.executer.executer import SubprocessExecuter, IsolateExecuter
from core.executer.exceptions import SubprocessError
from core.executer.base import BaseExecuter, CommandExecuter, ExecutionResult
from core.executer.masker import OutputMasker

pytestmark = pytest.mark.asyncio

@pytest.fixture
def executer():
    return SubprocessExecuter()

@pytest.fixture
def isolate_executer():
    return IsolateExecuter()


async def test_subprocess_executer_success(executer, mocker):
    """Test успешного выполнения команды в SubprocessExecuter."""
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Command executed successfully\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Some stderr output\n", b""]
    
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    final_result = None
    
    async for result in executer.execute_stream(["echo", "hello"], env={}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)

    assert len(output) == 2
    # Сортируем вывод чтобы гарантировать порядок проверки
    output.sort()
    assert output == [
        "[stderr] Some stderr output\n",
        "[stdout] Command executed successfully\n"
    ]
    assert final_result.status == 0
    assert "Command executed successfully" in final_result.stdout
    assert "Some stderr output" in final_result.stderr

async def test_subprocess_executer_error(executer, mocker):
    """Test обработки ошибки при выполнении команды в SubprocessExecuter."""
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Some stdout\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Some error occurred\n", b""]
    
    mock_proc.wait = AsyncMock(return_value=1)
    mock_proc.returncode = 1

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    final_result = None

    async for result in executer.execute_stream(["ls", "/nonexistent"], env={}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)

    assert len(output) == 2
    output.sort()
    assert output == [
        "[stderr] Some error occurred\n",
        "[stdout] Some stdout\n"
    ]
    assert final_result.status == 1
    assert "Some stdout" in final_result.stdout
    assert "Some error occurred" in final_result.stderr


async def test_subprocess_executer_masking(executer, mocker):
    """
    Test маскировки чувствительного вывода в SubprocessExecuter.
    """
    mock_processor = mocker.Mock()
    mock_processor.mask = lambda x: x.replace("sensitive", "******")
    executer._processor = mock_processor

    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"sensitive data\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"", b""]
    
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    async for result in executer.execute_stream(["echo", "sensitive"], env={}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)

    assert len(output) == 1
    assert output[0] == "[stdout] ****** data\n"
    assert final_result.status == 0
    assert "sensitive data" in final_result.stdout
    assert final_result.masked_stdout == "****** data\n"

async def test_isolate_executer_success(isolate_executer, mocker):
    """
    Test успешного выполнения команды в IsolateExecuter.
    """
    mock_override_env = mocker.patch("core.executer.executer.override_env", autospec=True)

    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Success\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"", b""]
    
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    async for result in isolate_executer.execute_stream(["echo", "test"], env={"KEY": "VALUE"}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)

    assert len(output) == 1
    assert output[0] == "[stdout] Success\n"
    assert final_result.status == 0
    assert "Success" in final_result.stdout
    mock_override_env.assert_called_once_with({"KEY": "VALUE"})

async def test_isolate_executer_override_env_called(isolate_executer, mocker):
    """
    Test вызова override_env в IsolateExecuter.
    """
    with patch("core.executer.executer.override_env", autospec=True) as mock_override_env:
        mock_proc = MagicMock()
        mock_proc.stdout = AsyncMock()
        mock_proc.stderr = AsyncMock()
        
        mock_proc.stdout.readline = AsyncMock()
        mock_proc.stdout.readline.side_effect = [b"Mocked output\n", b""]
        mock_proc.stderr.readline = AsyncMock()
        mock_proc.stderr.readline.side_effect = [b"", b""]
        
        mock_proc.wait = AsyncMock(return_value=0)
        mock_proc.returncode = 0

        mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

        output = []
        async for result in isolate_executer.execute_stream(["echo", "test"], env={"KEY": "VALUE"}, cwd=None, mask=True):
            if isinstance(result, ExecutionResult):
                final_result = result
            else:
                output.append(result)

        assert len(output) == 1
        assert output[0] == "[stdout] Mocked output\n"
        assert final_result.status == 0
        mock_override_env.assert_called_once_with({"KEY": "VALUE"})

async def test_subprocess_executer_execution_result_success(executer, mocker):
    """Test корректности ExecutionResult при успешном выполнении."""
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"First line\n", b"Second line\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Error info\n", b""]
    
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    final_result = None
    async for result in executer.execute_stream(["test", "cmd"], env={}, cwd=None, mask=False):  # Изменили на False
        if isinstance(result, ExecutionResult):
            final_result = result

    assert final_result is not None
    assert final_result.status == 0
    assert final_result.stdout == "First line\nSecond line\n"
    assert final_result.stderr == "Error info\n"
    assert final_result.masked_stdout is None
    assert final_result.masked_stderr is None

async def test_subprocess_executer_execution_result_with_masking(executer, mocker):
    """Test корректности ExecutionResult с маскированием."""
    # Настраиваем процессор маскировки
    mock_processor = mocker.Mock()
    mock_processor.mask = lambda x: x.replace("secret", "******")
    executer._processor = mock_processor

    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Contains secret data\n", b"Regular line\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Error with secret\n", b""]
    
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    final_result = None
    async for result in executer.execute_stream(["test", "cmd"], env={}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result

    assert final_result is not None
    assert final_result.status == 0
    # Проверяем оригинальные данные
    assert final_result.stdout == "Contains secret data\nRegular line\n"
    assert final_result.stderr == "Error with secret\n"
    # Проверяем маскированные данные
    assert final_result.masked_stdout == "Contains ****** data\nRegular line\n"
    assert final_result.masked_stderr == "Error with ******\n"

async def test_subprocess_executer_execution_result_error(executer, mocker):
    """Test корректности ExecutionResult при ошибке."""
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Partial output\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Critical error\n", b""]
    
    mock_proc.wait = AsyncMock(return_value=1)
    mock_proc.returncode = 1

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    final_result = None
    async for result in executer.execute_stream(["failing", "cmd"], env={}, cwd=None, mask=False):  # Изменили на False
        if isinstance(result, ExecutionResult):
            final_result = result

    assert final_result is not None
    assert final_result.status == 1
    assert final_result.stdout == "Partial output\n"  
    assert final_result.stderr == "Critical error\n"
    assert final_result.masked_stdout is None
    assert final_result.masked_stderr is None
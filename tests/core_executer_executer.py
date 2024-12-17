from unittest.mock import AsyncMock, patch

import pytest

from src.core.executer.executer import SubprocessExecuter 
from src.core.executer.exceptions import SubprocessError
from src.core.executer.base import ExecutionResult
from src.core.executer.masker import OutputMasker

pytestmark = pytest.mark.asyncio

async def test_subprocess_executer_success(executer, mocker):
    """Тест успешного выполнения команды в SubprocessExecuter"""
    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    mock_process.stdout.readline = AsyncMock(side_effect=[b"Command executed successfully\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"", b""])
    mock_process.returncode = 0

    result = await executer.execute(["echo", "hello"], env={}, cwd=None)
    assert result.status == 0
    assert "Command executed successfully" in result.stdout
    assert result.stderr == ""

async def test_subprocess_executer_error(executer, mocker):
    """Тест ошибки выполнения команды в SubprocessExecuter"""
    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    mock_process.stdout.readline = AsyncMock(side_effect=[b"Some stdout\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"Some error occurred\n", b""])
    mock_process.returncode = 1

    with pytest.raises(SubprocessError) as exc_info:
        await executer.execute(["ls", "/nonexistent"], env={}, cwd=None)

    exc = exc_info.value
    assert exc.returncode == 1
    assert "Some error occurred" in exc.stderr
    assert "Some stdout" in exc.stdout

async def test_subprocess_executer_masking(executer, mocker):
    """Тест маскирования вывода"""
    mock_processor = mocker.Mock()
    mock_processor.mask.side_effect = lambda x: x.replace("sensitive", "******")
    executer._processor = mock_processor

    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    mock_process.stdout.readline = AsyncMock(side_effect=[b"sensitive data\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"error with sensitive info\n", b""])
    mock_process.returncode = 0

    result = await executer.execute(["echo", "sensitive"], mask=True)
    assert result.masked_stdout == "****** data"
    assert result.masked_stderr == "error with ****** info"

async def test_isolate_executer_success(isolate_executer, mocker):
    """Тест успешного выполнения команды в IsolateExecuter"""
    mocker.patch("src.core.executer.executer.override_env", autospec=True)

    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    mock_process.stdout.readline = AsyncMock(side_effect=[b"Success\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"", b""])
    mock_process.returncode = 0

    result = await isolate_executer.execute(["echo", "test"], env={"KEY": "VALUE"})
    assert result.status == 0
    assert "Success" in result.stdout

async def test_isolate_executer_override_env_called(isolate_executer, mocker):
    """Тест, что override_env вызывается в IsolateExecuter"""
    with patch("src.core.executer.executer.override_env", autospec=True) as mock_override_env:
        mock_execute = mocker.patch.object(
            SubprocessExecuter, "execute", new=AsyncMock()
        )

        await isolate_executer.execute(["echo", "test"], env={"KEY": "VALUE"})

        mock_override_env.assert_called_once_with({"KEY": "VALUE"})
        mock_execute.assert_awaited_once_with(["echo", "test"], {"KEY": "VALUE"}, None, False)

@pytest.mark.asyncio
async def test_execute_with_env_masking():
    masker = OutputMasker()
    masker.sensitive("secret-value")

    executer = SubprocessExecuter(processor=masker)
    executer._run_command = AsyncMock(return_value=ExecutionResult(0, "stdout", "stderr"))

    env = {"VAR1": "value1", "SECRET_VAR": "secret-value"}
    result = await executer.execute(
        cmd=["echo", "test"],
        env=env,
        mask=True
    )

    assert result.masked_stdout is not None
    assert result.masked_stderr is not None
    assert "******" in masker.mask_env(env).values()

@pytest.mark.asyncio
async def test_isolate_executer_env_masking(isolate_executer, mocker):
    """Тест маскирования переменных окружения в IsolateExecuter"""
    # Используем настоящий OutputMasker
    masker = OutputMasker()
    masker.sensitive("sensitive_value")

    isolate_executer._processor = masker

    # Мок процесса и вызовов readline
    mock_process = AsyncMock()
    mock_process.stdout.readline = AsyncMock(side_effect=[b"Output with sensitive_value\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"Error sensitive_value\n", b""])
    mock_process.returncode = 0

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    env = {"VAR1": "value1", "SECRET_VAR": "sensitive_value"}
    result = await isolate_executer.execute(["echo", "test"], env=env, mask=True)

    # Проверка маскирования
    assert result.masked_stdout.strip() == "Output with ******"
    assert result.masked_stderr.strip() == "Error ******"
    assert result.stdout.strip() == "Output with sensitive_value"
    assert result.stderr.strip() == "Error sensitive_value"

@pytest.mark.asyncio
async def test_sensitive_env_masking(mocker):
    """Тест маскирования содержимого чувствительных переменных окружения в stdout"""
    masker = OutputMasker()
    masker.sensitive("secret-value")

    executer = SubprocessExecuter(processor=masker)
    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    # Моделируем вывод команды с чувствительными данными
    mock_process.stdout.readline = AsyncMock(side_effect=[b"Value is secret-value\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"Error: secret-value not found\n", b""])
    mock_process.returncode = 0

    result = await executer.execute(["echo", "test"], mask=True)

    # Проверяем, что чувствительные данные заменены на "******"
    assert result.masked_stdout.strip() == "Value is ******"
    assert result.masked_stderr.strip() == "Error: ****** not found"

    # Оригинальный stdout/stderr остаётся неизменным
    assert result.stdout.strip() == "Value is secret-value"
    assert result.stderr.strip() == "Error: secret-value not found"

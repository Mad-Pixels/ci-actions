from unittest.mock import AsyncMock, patch
from pathlib import Path

import pytest
import asyncio

from core.executer.executer import SubprocessExecuter, IsolateExecuter
from core.executer.exceptions import SubprocessError
from core.executer.base import ExecutionResult

@pytest.fixture
def executer():
    """Фикстура для создания экземпляра SubprocessExecuter"""
    return SubprocessExecuter()

@pytest.fixture
def isolate_executer():
    """Фикстура для создания экземпляра IsolateExecuter"""
    return IsolateExecuter()

@pytest.mark.asyncio
async def test_subprocess_executer_success(executer, mocker):
    """Тест успешного выполнения команды в SubprocessExecuter"""
    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    # Mock the stdout and stderr behavior
    mock_process.stdout.readline = AsyncMock(side_effect=[b"Command executed successfully\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"", b""])

    # Set return code
    mock_process.returncode = 0

    result = await executer.execute(["echo", "hello"], env={}, cwd=None)
    assert result.status == 0
    assert "Command executed successfully" in result.stdout
    assert result.stderr == ""

@pytest.mark.asyncio
async def test_subprocess_executer_error(executer, mocker):
    """Тест ошибки выполнения команды в SubprocessExecuter"""
    # Создаем объект mock_process
    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    # Настройка для stdout и stderr
    mock_process.stdout.readline = AsyncMock(side_effect=[b"Some stdout\n", b""])  # Симулируем вывод в stdout
    mock_process.stderr.readline = AsyncMock(side_effect=[b"Some error occurred\n", b""])  # Симулируем вывод в stderr

    # Устанавливаем код возврата
    mock_process.returncode = 1

    # Проверяем, что SubprocessError вызывается
    with pytest.raises(SubprocessError) as exc_info:
        await executer.execute(["ls", "/nonexistent"], env={}, cwd=None)

    exc = exc_info.value
    assert exc.returncode == 1
    assert "Some error occurred" in exc.stderr
    assert "Some stdout" in exc.stdout

@pytest.mark.asyncio
async def test_subprocess_executer_masking(executer, mocker):
    """Тест маскирования вывода"""
    # Mock процессора для маскирования
    mock_processor = mocker.Mock()
    mock_processor.mask.side_effect = lambda x: x.replace("sensitive", "******")
    executer._processor = mock_processor

    # Mock процесса выполнения
    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    # Настройка вывода stdout и stderr
    mock_process.stdout.readline = AsyncMock(side_effect=[b"sensitive data\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"error with sensitive info\n", b""])

    # Установка кода возврата
    mock_process.returncode = 0

    # Выполнение команды
    result = await executer.execute(["echo", "sensitive"], mask=True)

    # Проверка результата
    assert result.masked_stdout == "****** data"
    assert result.masked_stderr == "error with ****** info"

@pytest.mark.asyncio
async def test_isolate_executer_success(isolate_executer, mocker):
    """Тест успешного выполнения команды в IsolateExecuter"""
    # Мокируем override_env
    mocker.patch("core.executer.utils.override_env", autospec=True)

    # Мокируем create_subprocess_exec
    mock_process = AsyncMock()
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_process)

    # Настраиваем stdout и stderr
    mock_process.stdout.readline = AsyncMock(side_effect=[b"Success\n", b""])
    mock_process.stderr.readline = AsyncMock(side_effect=[b"", b""])

    # Устанавливаем код возврата
    mock_process.returncode = 0

    # Выполнение команды
    result = await isolate_executer.execute(["echo", "test"], env={"KEY": "VALUE"})

    # Проверка результата
    assert result.status == 0
    assert "Success" in result.stdout

@pytest.mark.asyncio
async def test_isolate_executer_override_env_called(isolate_executer, mocker):
    """Тест, что override_env вызывается в IsolateExecuter"""
    # Патчим override_env в модуле core.executer.executer
    with patch("core.executer.executer.override_env", autospec=True) as mock_override_env:
        # Мокируем execute в SubprocessExecuter
        mock_execute = mocker.patch.object(
            SubprocessExecuter, "execute", new=AsyncMock()
        )

        # Выполняем тестируемый метод
        await isolate_executer.execute(["echo", "test"], env={"KEY": "VALUE"})

        # Проверяем, что override_env вызывался с правильными аргументами
        mock_override_env.assert_called_once_with({"KEY": "VALUE"})

        # Проверяем, что метод execute родительского класса вызывался
        mock_execute.assert_awaited_once_with(["echo", "test"], {"KEY": "VALUE"}, None, False)
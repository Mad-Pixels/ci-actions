from unittest.mock import AsyncMock, MagicMock, patch
import pytest
import asyncio
from pathlib import Path

from core.executer.executer import SubprocessExecuter, IsolateExecuter
from core.executer.exceptions import SubprocessError
from core.executer.masker import OutputMasker

pytestmark = pytest.mark.asyncio

@pytest.fixture
def executer():
    return SubprocessExecuter()

@pytest.fixture
def isolate_executer():
    return IsolateExecuter()

async def test_subprocess_executer_success(executer, mocker):
    """
    Test успешного выполнения команды в SubprocessExecuter.
    """
    # Создаём мок процесса
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Настраиваем stdout
    mock_proc.stdout.readline = AsyncMock(side_effect=[b"Command executed successfully\n", b""])
    
    # Настраиваем stderr с учетом асинхронности
    stderr_data = b""
    mock_proc.stderr.read = AsyncMock(return_value=stderr_data)
    
    # Устанавливаем returncode
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    # Мокируем создание процесса
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    # Выполняем команду
    output = []
    async for line in executer.execute_stream(["echo", "hello"], env={}, cwd=None, mask=True):
        output.append(line)

    assert output == ["Command executed successfully\n"]
    assert mock_proc.wait.await_count == 1
    assert mock_proc.stderr.read.await_count == 1

async def test_subprocess_executer_error(executer, mocker):
    """
    Test обработки ошибки при выполнении команды в SubprocessExecuter.
    """
    # Создаём мок процесса
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Настраиваем stdout и stderr
    mock_proc.stdout.readline = AsyncMock(side_effect=[b"Some stdout\n", b""])
    stderr_data = b"Some error occurred\n"
    mock_proc.stderr.read = AsyncMock(return_value=stderr_data)
    
    # Устанавливаем returncode
    mock_proc.wait = AsyncMock(return_value=1)
    mock_proc.returncode = 1

    # Мокируем создание процесса
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    # Проверяем, что вызывается исключение
    with pytest.raises(SubprocessError) as exc_info:
        async for line in executer.execute_stream(["ls", "/nonexistent"], env={}, cwd=None, mask=True):
            pass

    error = exc_info.value
    assert error.returncode == 1
    assert "Some error occurred" in error.stderr

async def test_subprocess_executer_masking(executer, mocker):
    """
    Test маскировки чувствительного вывода в SubprocessExecuter.
    """
    # Настраиваем процессор маскировки
    mock_processor = mocker.Mock()
    mock_processor.mask.side_effect = lambda x: x.replace("sensitive", "******")
    executer._processor = mock_processor

    # Создаём мок процесса
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Настраиваем stdout и stderr
    mock_proc.stdout.readline = AsyncMock(side_effect=[b"sensitive data\n", b""])
    stderr_data = b""
    mock_proc.stderr.read = AsyncMock(return_value=stderr_data)
    
    # Устанавливаем returncode
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    # Мокируем создание процесса
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    # Выполняем команду
    output = []
    async for line in executer.execute_stream(["echo", "sensitive"], env={}, cwd=None, mask=True):
        output.append(mock_processor.mask(line))

    assert output == ["****** data\n"]
    assert mock_proc.wait.await_count == 1
    assert mock_proc.stderr.read.await_count == 1

async def test_isolate_executer_success(isolate_executer, mocker):
    """
    Test успешного выполнения команды в IsolateExecuter.
    """
    # Патчим override_env
    mock_override_env = mocker.patch("core.executer.executer.override_env", autospec=True)

    # Создаём мок процесса
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Настраиваем stdout и stderr
    mock_proc.stdout.readline = AsyncMock(side_effect=[b"Success\n", b""])
    stderr_data = b""
    mock_proc.stderr.read = AsyncMock(return_value=stderr_data)
    
    # Устанавливаем returncode
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    # Мокируем создание процесса
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    # Выполняем команду
    output = []
    async for line in isolate_executer.execute_stream(["echo", "test"], env={"KEY": "VALUE"}, cwd=None, mask=True):
        output.append(line)

    assert output == ["Success\n"]
    mock_override_env.assert_called_once_with({"KEY": "VALUE"})
    assert mock_proc.wait.await_count == 1
    assert mock_proc.stderr.read.await_count == 1

async def test_isolate_executer_override_env_called(isolate_executer, mocker):
    """
    Test вызова override_env в IsolateExecuter.
    """
    # Патчим override_env
    with patch("core.executer.executer.override_env", autospec=True) as mock_override_env:
        # Создаём мок процесса
        mock_proc = MagicMock()
        mock_proc.stdout = AsyncMock()
        mock_proc.stderr = AsyncMock()
        
        # Настраиваем stdout и stderr
        mock_proc.stdout.readline = AsyncMock(side_effect=[b"Mocked output\n", b""])
        stderr_data = b""
        mock_proc.stderr.read = AsyncMock(return_value=stderr_data)
        
        # Устанавливаем returncode
        mock_proc.wait = AsyncMock(return_value=0)
        mock_proc.returncode = 0

        # Мокируем создание процесса
        mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

        # Выполняем команду
        async for line in isolate_executer.execute_stream(["echo", "test"], env={"KEY": "VALUE"}, cwd=None, mask=True):
            pass

        mock_override_env.assert_called_once_with({"KEY": "VALUE"})
        assert mock_proc.wait.await_count == 1
        assert mock_proc.stderr.read.await_count == 1
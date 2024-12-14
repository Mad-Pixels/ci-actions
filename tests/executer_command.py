"""Tests for command executors"""
import pytest
import asyncio
import os
from pathlib import Path
from unittest.mock import patch, AsyncMock, MagicMock

from core.executer.command import (
    SubprocessExecuter,
    IsolateExecuter,
    _read_stream,
    backup_env
)
from core.executer.exceptions import SubprocessError
from core.executer.masking import OutputMasker

@pytest.fixture
def subprocess_executer():
    """Create subprocess executer instance"""
    return SubprocessExecuter()

@pytest.fixture
def output_masker():
    """Create configured output masker"""
    masker = OutputMasker()
    masker.add_pattern(r"\d{4}-\d{2}-\d{2}")  # Date pattern
    masker.add_sensitive_value("SECRET_VALUE")
    return masker

@pytest.fixture
def isolate_executer(output_masker):
    """Create isolated executer with masking"""
    return IsolateExecuter(output_processor=output_masker)

@pytest.mark.asyncio
async def test_subprocess_executer_echo(subprocess_executer):
    """Test basic echo command execution"""
    result = await subprocess_executer.execute(["echo", "test"])
    assert result.status == 0
    assert "test" in result.stdout
    assert not result.stderr

@pytest.mark.asyncio
async def test_subprocess_executer_error(subprocess_executer):
    """Test handling of command execution error"""
    with pytest.raises(SubprocessError) as exc_info:
        await subprocess_executer.execute(["ls", "/nonexistent"])
    assert "No such file or directory" in str(exc_info.value)

@pytest.mark.asyncio
async def test_isolate_executer_with_env(isolate_executer):
    """Test isolated execution with custom environment"""
    env = {"TEST_VAR": "test_value"}
    result = await isolate_executer.execute(
        ["printenv", "TEST_VAR"],
        env=env
    )
    assert result.status == 0
    assert "test_value" in result.stdout

@pytest.mark.asyncio
async def test_isolate_executer_masking(isolate_executer):
    """Test output masking in isolated executor"""
    result = await isolate_executer.execute(
        ["echo", "SECRET_VALUE on 2024-01-01"],
        mask=True
    )
    assert "SECRET_VALUE" not in result.masked_stdout
    assert "2024-01-01" not in result.masked_stdout
    assert "*********** on *****" in result.masked_stdout

@pytest.mark.asyncio
@patch("asyncio.create_subprocess_exec")
async def test_subprocess_mock(mock_subprocess, subprocess_executer):
    """Test subprocess execution with mocked process"""
    mock_process = AsyncMock()
    mock_process.returncode = 0
    mock_process.stdout.readline.side_effect = [
        b"output line 1\n",
        b"output line 2\n",
        b""
    ]
    mock_process.stderr.readline.side_effect = [b""]
    mock_subprocess.return_value = mock_process

    result = await subprocess_executer.execute(["test", "command"])
    
    assert result.status == 0
    assert "output line 1" in result.stdout
    assert "output line 2" in result.stdout

def test_backup_env():
    """Test environment backup context manager"""
    original_path = os.environ.get("PATH")
    test_env = {"PATH": "/test/path"}
    
    with backup_env(test_env):
        assert os.environ["PATH"] == "/test/path"
    
    assert os.environ["PATH"] == original_path

@pytest.mark.asyncio
async def test_read_stream():
    """Test stream reading"""
    mock_stream = AsyncMock()
    mock_stream.readline.side_effect = [
        b"line1\n",
        b"line2\n",
        b""
    ]
    mock_logger = MagicMock()

    output = await _read_stream(mock_stream, mock_logger, "TEST")
    
    assert output == "line1\nline2\n"
    assert mock_logger.debug.call_count == 2

@pytest.mark.asyncio
async def test_execution_cancellation(subprocess_executer):
    """Test handling of cancelled execution"""
    async def cancel_soon():
        await asyncio.sleep(0.1)
        raise asyncio.CancelledError()

    with pytest.raises(asyncio.CancelledError):
        await asyncio.gather(
            subprocess_executer.execute(["sleep", "1"]),
            cancel_soon()
        )

@pytest.mark.asyncio
async def test_working_directory(subprocess_executer, tmp_path):
    """Test command execution in specific working directory"""
    test_dir = tmp_path / "test_dir"
    test_dir.mkdir()
    
    result = await subprocess_executer.execute(
        ["pwd"],
        cwd=test_dir
    )
    
    assert str(test_dir) in result.stdout
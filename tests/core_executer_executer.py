from unittest.mock import AsyncMock, MagicMock, patch
from pathlib import Path

import pytest

from core.executer.exceptions import CommandValidationError
from core.executer.base import ExecutionResult

pytestmark = pytest.mark.asyncio

async def test_subprocess_executer_success(executer, mocker):
    """Test successful execution of a command using SubprocessExecuter."""
    # Mock the subprocess.Popen object
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Simulate stdout and stderr outputs
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Command executed successfully\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Some stderr output\n", b""]
    
    # Simulate process exit code
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    # Patch asyncio.create_subprocess_exec to return the mocked process
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    final_result = None
    
    # Execute the command and collect outputs
    async for result in executer.execute_stream(["echo", "hello"], env={}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)

    # Assert that two lines were yielded: one from stdout and one from stderr
    assert len(output) == 2
    # Sort the output to ensure consistent order for assertions
    output.sort()
    assert output == [
        "[stderr] Some stderr output\n",
        "[stdout] Command executed successfully\n"
    ]
    # Verify the final ExecutionResult
    assert final_result.status == 0
    assert "Command executed successfully" in final_result.stdout
    assert "Some stderr output" in final_result.stderr


async def test_subprocess_executer_error(executer, mocker):
    """Test handling of command execution errors in SubprocessExecuter."""
    # Mock the subprocess.Popen object
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Simulate stdout and stderr outputs
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Some stdout\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Some error occurred\n", b""]
    
    # Simulate process exit code indicating an error
    mock_proc.wait = AsyncMock(return_value=1)
    mock_proc.returncode = 1

    # Patch asyncio.create_subprocess_exec to return the mocked process
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    final_result = None

    # Execute the failing command and collect outputs
    async for result in executer.execute_stream(["ls", "/nonexistent"], env={}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)

    # Assert that two lines were yielded: one from stdout and one from stderr
    assert len(output) == 2
    output.sort()
    assert output == [
        "[stderr] Some error occurred\n",
        "[stdout] Some stdout\n"
    ]
    # Verify the final ExecutionResult with error status
    assert final_result.status == 1
    assert "Some stdout" in final_result.stdout
    assert "Some error occurred" in final_result.stderr


async def test_subprocess_executer_masking(executer, mocker):
    """
    Test masking of sensitive output data in SubprocessExecuter.
    
    Ensures that sensitive information is properly masked in the output.
    """
    # Mock the OutputMasker processor to mask "sensitive" as "******"
    mock_processor = mocker.Mock()
    mock_processor.mask = lambda x: x.replace("sensitive", "******")
    executer._processor = mock_processor

    # Mock the subprocess.Popen object
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Simulate masked stdout and no stderr
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"sensitive data\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"", b""]
    
    # Simulate successful process exit
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    # Patch asyncio.create_subprocess_exec to return the mocked process
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    async for result in executer.execute_stream(["echo", "sensitive"], env={}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)

    # Assert that the masked output was yielded correctly
    assert len(output) == 1
    assert output[0] == "[stdout] ****** data\n"
    # Verify the ExecutionResult with masked stdout
    assert final_result.status == 0
    assert "sensitive data" in final_result.stdout
    assert final_result.masked_stdout == "****** data\n"


async def test_isolate_executer_success(isolate_executer, mocker):
    """
    Test successful execution of a command using IsolateExecuter.
    
    Ensures that environment variables are correctly overridden during execution.
    """
    # Mock the override_env context manager
    mock_override_env = mocker.patch("core.executer.executer.override_env", autospec=True)

    # Mock the subprocess.Popen object
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Simulate stdout and no stderr
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Success\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"", b""]
    
    # Simulate successful process exit
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    # Patch asyncio.create_subprocess_exec to return the mocked process
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    async for result in isolate_executer.execute_stream(["echo", "test"], env={"KEY": "VALUE"}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)

    # Assert that the output was yielded correctly
    assert len(output) == 1
    assert output[0] == "[stdout] Success\n"
    # Verify the ExecutionResult
    assert final_result.status == 0
    assert "Success" in final_result.stdout
    # Ensure override_env was called with the correct environment variables
    mock_override_env.assert_called_once_with({"KEY": "VALUE"})


async def test_isolate_executer_override_env_called(isolate_executer, mocker):
    """
    Test that override_env is called correctly in IsolateExecuter.
    
    Ensures that the environment overriding mechanism is invoked during command execution.
    """
    with patch("core.executer.executer.override_env", autospec=True) as mock_override_env:
        # Mock the subprocess.Popen object
        mock_proc = MagicMock()
        mock_proc.stdout = AsyncMock()
        mock_proc.stderr = AsyncMock()
        
        # Simulate stdout and no stderr
        mock_proc.stdout.readline = AsyncMock()
        mock_proc.stdout.readline.side_effect = [b"Mocked output\n", b""]
        mock_proc.stderr.readline = AsyncMock()
        mock_proc.stderr.readline.side_effect = [b"", b""]
        
        # Simulate successful process exit
        mock_proc.wait = AsyncMock(return_value=0)
        mock_proc.returncode = 0

        # Patch asyncio.create_subprocess_exec to return the mocked process
        mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

        output = []
        async for result in isolate_executer.execute_stream(["echo", "test"], env={"KEY": "VALUE"}, cwd=None, mask=True):
            if isinstance(result, ExecutionResult):
                final_result = result
            else:
                output.append(result)

        # Assert that the output was yielded correctly
        assert len(output) == 1
        assert output[0] == "[stdout] Mocked output\n"
        # Verify the ExecutionResult
        assert final_result.status == 0
        # Ensure override_env was called with the correct environment variables
        mock_override_env.assert_called_once_with({"KEY": "VALUE"})


async def test_subprocess_executer_execution_result_success(executer, mocker):
    """Test the correctness of ExecutionResult on successful execution."""
    # Mock the subprocess.Popen object
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Simulate multiple lines in stdout and stderr
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"First line\n", b"Second line\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Error info\n", b""]
    
    # Simulate successful process exit
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    # Patch asyncio.create_subprocess_exec to return the mocked process
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    final_result = None
    # Execute the command and collect the ExecutionResult
    async for result in executer.execute_stream(["test", "cmd"], env={}, cwd=None, mask=False):
        if isinstance(result, ExecutionResult):
            final_result = result

    # Verify the ExecutionResult contents
    assert final_result is not None
    assert final_result.status == 0
    assert final_result.stdout == "First line\nSecond line\n"
    assert final_result.stderr == "Error info\n"
    assert final_result.masked_stdout is None
    assert final_result.masked_stderr is None


async def test_subprocess_executer_execution_result_with_masking(executer, mocker):
    """Test the correctness of ExecutionResult with masking enabled."""
    # Setup the OutputMasker to mask the word "secret"
    mock_processor = mocker.Mock()
    mock_processor.mask = lambda x: x.replace("secret", "******")
    executer._processor = mock_processor

    # Mock the subprocess.Popen object
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Simulate masked stdout and stderr
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Contains secret data\n", b"Regular line\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Error with secret\n", b""]
    
    # Simulate successful process exit
    mock_proc.wait = AsyncMock(return_value=0)
    mock_proc.returncode = 0

    # Patch asyncio.create_subprocess_exec to return the mocked process
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    final_result = None
    # Execute the command and collect the ExecutionResult
    async for result in executer.execute_stream(["test", "cmd"], env={}, cwd=None, mask=True):
        if isinstance(result, ExecutionResult):
            final_result = result

    # Verify the ExecutionResult with masked outputs
    assert final_result is not None
    assert final_result.status == 0
    assert final_result.stdout == "Contains secret data\nRegular line\n"
    assert final_result.stderr == "Error with secret\n"
    assert final_result.masked_stdout == "Contains ****** data\nRegular line\n"
    assert final_result.masked_stderr == "Error with ******\n"


async def test_subprocess_executer_execution_result_error(executer, mocker):
    """Test the correctness of ExecutionResult when an error occurs."""
    # Mock the subprocess.Popen object
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    
    # Simulate partial stdout and stderr outputs
    mock_proc.stdout.readline = AsyncMock()
    mock_proc.stdout.readline.side_effect = [b"Partial output\n", b""]
    mock_proc.stderr.readline = AsyncMock()
    mock_proc.stderr.readline.side_effect = [b"Critical error\n", b""]
    
    # Simulate process exit code indicating an error
    mock_proc.wait = AsyncMock(return_value=1)
    mock_proc.returncode = 1

    # Patch asyncio.create_subprocess_exec to return the mocked process
    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    final_result = None
    # Execute the failing command and collect the ExecutionResult
    async for result in executer.execute_stream(["failing", "cmd"], env={}, cwd=None, mask=False):
        if isinstance(result, ExecutionResult):
            final_result = result

    # Verify the ExecutionResult with error status
    assert final_result is not None
    assert final_result.status == 1
    assert final_result.stdout == "Partial output\n"  
    assert final_result.stderr == "Critical error\n"
    assert final_result.masked_stdout is None
    assert final_result.masked_stderr is None

async def test_subprocess_executer_validation_error(executer, mocker):
    """Test that CommandValidationError is raised for invalid command inputs."""
    with pytest.raises(CommandValidationError):
        # Empty command list
        async for _ in executer.execute_stream([], mask=False):
            pass

    with pytest.raises(CommandValidationError):
        # Non-string command arguments
        async for _ in executer.execute_stream(["ls", 123], mask=False):
            pass

    with pytest.raises(CommandValidationError):
        # Non-existent working directory
        async for _ in executer.execute_stream(["ls"], cwd=Path("/nonexistent"), mask=False):
            pass

@pytest.mark.parametrize("cmd, returncode, stdout_side, stderr_side, expected_status", [
    (["echo", "success"], 0, [b"Success\n", b""], [b"", b""] , 0),
    (["false"], 1, [b"", b""], [b"Command failed\n", b""], 1),
])
async def test_subprocess_executer_various_cases(executer, mocker, cmd, returncode, stdout_side, stderr_side, expected_status):
    """Test various command execution scenarios."""
    mock_proc = MagicMock()
    mock_proc.stdout = AsyncMock()
    mock_proc.stderr = AsyncMock()
    mock_proc.stdout.readline = AsyncMock(side_effect=stdout_side)
    mock_proc.stderr.readline = AsyncMock(side_effect=stderr_side)
    mock_proc.wait = AsyncMock(return_value=returncode)
    mock_proc.returncode = returncode

    mocker.patch("asyncio.create_subprocess_exec", return_value=mock_proc)

    output = []
    async for result in executer.execute_stream(cmd, mask=False):
        if isinstance(result, ExecutionResult):
            final_result = result
        else:
            output.append(result)
    
    assert final_result.status == expected_status

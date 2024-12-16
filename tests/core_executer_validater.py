from pathlib import Path

import pytest

from src.core.executer.validater import validate_command, validate_env, validate_cwd
from src.core.executer.exceptions import CommandValidationError

def test_validate_command_valid():
    validate_command(["echo", "Hello, World!"])

def test_validate_command_empty():
    with pytest.raises(CommandValidationError, match="Empty command sequence"):
        validate_command([])

def test_validate_command_invalid_type():
    with pytest.raises(CommandValidationError, match="Command must be a sequence"):
        validate_command("ls -la")

def test_validate_command_invalid_argument():
    with pytest.raises(CommandValidationError, match="Invalid command argument"):
        validate_command(["echo", "Hello; World!"])

def test_validate_command_non_string_argument():
    with pytest.raises(CommandValidationError, match="Command arg must be sting"):
        validate_command(["echo", 123])

def test_validate_env_valid():
    env = {"KEY": "value", "VAR": "test"}
    sanitized = validate_env(env)
    assert sanitized == env

def test_validate_env_strip_whitespace():
    env = {" KEY ": " value ", " VAR ": " test "}
    sanitized = validate_env(env)
    assert sanitized == {"KEY": "value", "VAR": "test"}

def test_validate_env_invalid_key_type():
    env = {123: "value"}
    with pytest.raises(CommandValidationError, match="Environment variables must be strings"):
        validate_env(env)

def test_validate_env_invalid_value_type():
    env = {"KEY": 123}
    with pytest.raises(CommandValidationError, match="Environment variables must be strings"):
        validate_env(env)

def test_validate_env_empty():
    sanitized = validate_env({})
    assert sanitized == {}

def test_validate_cwd_valid(tmp_path):
    assert validate_cwd(tmp_path) == tmp_path

def test_validate_cwd_none():
    assert validate_cwd(None) is None

def test_validate_cwd_invalid_path_type():
    cwd = validate_cwd(str(Path.cwd()))
    assert cwd == Path.cwd()

def test_validate_cwd_nonexistent():
    with pytest.raises(CommandValidationError, match="Working directory does not exist"):
        validate_cwd(Path("/nonexistent/directory"))

def test_validate_cwd_not_a_directory(tmp_path):
    file_path = tmp_path / "test_file"
    file_path.write_text("test")
    with pytest.raises(CommandValidationError, match="Path is not a directory"):
        validate_cwd(file_path)

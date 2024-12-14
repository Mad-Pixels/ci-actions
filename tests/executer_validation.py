"""Tests for validation module"""
import pytest
from pathlib import Path
from typing import Dict

from core.executer.validation import validate_command, validate_env, validate_cwd
from core.executer.exceptions import CommandValidationError

@pytest.mark.parametrize("cmd,should_raise", [
    (["ls", "-l"], False),
    ([], True),
    (None, True),
    ([123, "test"], True),
    (["echo", "hello; rm -rf /"], True),
    (["echo", "test && ls"], True),
    (["git", "commit", "-m", "test"], False),
])
def test_validate_command(cmd, should_raise):
    """Test command validation with various inputs"""
    if should_raise:
        with pytest.raises(CommandValidationError):
            validate_command(cmd)
    else:
        validate_command(cmd)

@pytest.mark.parametrize("env,expected", [
    (None, {}),
    ({}, {}),
    ({"PATH": "/usr/bin"}, {"PATH": "/usr/bin"}),
    ({"KEY": "  value  "}, {"KEY": "value"}),
])
def test_validate_env_valid(env: Dict[str, str], expected: Dict[str, str]):
    """Test environment validation with valid inputs"""
    result = validate_env(env)
    assert result == expected

@pytest.mark.parametrize("env", [
    {"test": 123},  # Non-string value
    {123: "test"},  # Non-string key
    {None: "test"},  # Invalid key type
])
def test_validate_env_invalid(env):
    """Test environment validation with invalid inputs"""
    with pytest.raises(CommandValidationError):
        validate_env(env)

def test_validate_cwd_none():
    """Test working directory validation with None input"""
    assert validate_cwd(None) is None

def test_validate_cwd_valid(tmp_path):
    """Test working directory validation with valid directory"""
    result = validate_cwd(tmp_path)
    assert result == tmp_path

def test_validate_cwd_nonexistent(tmp_path):
    """Test working directory validation with non-existent path"""
    nonexistent = tmp_path / "nonexistent"
    with pytest.raises(CommandValidationError):
        validate_cwd(nonexistent)

def test_validate_cwd_file(tmp_path):
    """Test working directory validation with file instead of directory"""
    test_file = tmp_path / "test.txt"
    test_file.write_text("test")
    with pytest.raises(CommandValidationError):
        validate_cwd(test_file)
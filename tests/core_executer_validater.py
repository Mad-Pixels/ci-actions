from pathlib import Path

import pytest

from core.executer.validater import validate_command, validate_env, validate_cwd
from core.executer.exceptions import CommandValidationError

def test_validate_command_valid():
    """Проверка валидной команды"""
    validate_command(["echo", "Hello, World!"])  # Не должно вызывать исключений


def test_validate_command_empty():
    """Проверка на пустую команду"""
    with pytest.raises(CommandValidationError, match="Empty command sequence"):
        validate_command([])


def test_validate_command_invalid_type():
    """Проверка, что команда должна быть последовательностью"""
    with pytest.raises(CommandValidationError, match="Command must be a sequence"):
        validate_command("ls -la")


def test_validate_command_invalid_argument():
    """Проверка на недопустимые символы в аргументе"""
    with pytest.raises(CommandValidationError, match="Invalid command argument"):
        validate_command(["echo", "Hello; World!"])


def test_validate_command_non_string_argument():
    """Проверка, что все аргументы команды должны быть строками"""
    with pytest.raises(CommandValidationError, match="Command arg must be sting"):
        validate_command(["echo", 123])


# Тесты для validate_env
def test_validate_env_valid():
    """Проверка валидного окружения"""
    env = {"KEY": "value", "VAR": "test"}
    sanitized = validate_env(env)
    assert sanitized == env


def test_validate_env_strip_whitespace():
    """Проверка, что пробелы в ключах и значениях убираются"""
    env = {" KEY ": " value ", " VAR ": " test "}
    sanitized = validate_env(env)
    assert sanitized == {"KEY": "value", "VAR": "test"}


def test_validate_env_invalid_key_type():
    """Проверка, что ключи и значения окружения должны быть строками"""
    env = {123: "value"}
    with pytest.raises(CommandValidationError, match="Environment variables must be strings"):
        validate_env(env)


def test_validate_env_invalid_value_type():
    """Проверка, что значения окружения должны быть строками"""
    env = {"KEY": 123}
    with pytest.raises(CommandValidationError, match="Environment variables must be strings"):
        validate_env(env)


def test_validate_env_empty():
    """Проверка на пустое окружение"""
    sanitized = validate_env({})
    assert sanitized == {}


# Тесты для validate_cwd
def test_validate_cwd_valid(tmp_path):
    """Проверка валидной рабочей директории"""
    assert validate_cwd(tmp_path) == tmp_path


def test_validate_cwd_none():
    """Проверка, если рабочая директория не указана"""
    assert validate_cwd(None) is None


def test_validate_cwd_invalid_path_type():
    """Проверка, что путь преобразуется в Path, если передан строкой"""
    cwd = validate_cwd(str(Path.cwd()))
    assert cwd == Path.cwd()


def test_validate_cwd_nonexistent():
    """Проверка, что директория существует"""
    with pytest.raises(CommandValidationError, match="Working directory does not exist"):
        validate_cwd(Path("/nonexistent/directory"))


def test_validate_cwd_not_a_directory(tmp_path):
    """Проверка, что путь не является файлом"""
    file_path = tmp_path / "test_file"
    file_path.write_text("test")
    with pytest.raises(CommandValidationError, match="Path is not a directory"):
        validate_cwd(file_path)

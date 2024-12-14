from typing import TypedDict, Dict, Optional, Literal, Union
from typing_extensions import NotRequired

EnvironmentType = Literal["local", "github"]

class EnvironmentConfig(TypedDict):
    """Base configuration for environment"""
    type: EnvironmentType
    mask_secrets: NotRequired[bool]  # Маскировать ли секреты в логах
    inherit_env: NotRequired[bool]   # Наследовать ли текущие env переменные

class Secret(TypedDict):
    """Secret configuration"""
    name: str  # Имя секрета
    env_var: str  # В какую env переменную мапить
    required: NotRequired[bool]  # Обязательный ли секрет
    default: NotRequired[str]  # Значение по умолчанию

class Variable(TypedDict):
    """Environment variable configuration"""
    name: str  # Имя переменной
    env_var: str  # В какую env переменную мапить
    required: NotRequired[bool]  # Обязательная ли переменная
    default: NotRequired[str]  # Значение по умолчанию

class GitHubEnvironmentConfig(EnvironmentConfig):
    """GitHub specific environment configuration"""
    type: Literal["github"]
    workspace: NotRequired[str]  # GitHub workspace
    token_var: NotRequired[str]  # Имя переменной с GitHub token
    
class LocalEnvironmentConfig(EnvironmentConfig):
    """Local environment configuration"""
    type: Literal["local"]
    env_file: NotRequired[str]  # Путь к .env файлу
    
EnvConfig = Union[GitHubEnvironmentConfig, LocalEnvironmentConfig]

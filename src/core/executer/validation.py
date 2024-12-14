from typing import Sequence, Dict, Optional
from pathlib import Path

from .exceptions import CommandValidationError

def validate_command(cmd: Sequence[str]) -> None:
    """
    Validate command sequence
    
    Args:
        cmd: Command sequence as strings
        
    Raises:
        CommandValidationError: If command is invalid
    """
    if not cmd:
        raise CommandValidationError("Empty command sequence")
    
    if not isinstance(cmd, Sequence):
        raise CommandValidationError(
            f"Command must be a sequence, got {type(cmd)}"
        )

    for arg in cmd:
        if not isinstance(arg, str):
            raise CommandValidationError(
                f"Command arguments must be strings, got {type(arg)}"
            )
        
        # Basic injection prevention
        if any(char in arg for char in ['&', '|', ';', '`', '$', '\\']):
            raise CommandValidationError(
                f"Invalid command argument: {arg}"
            )

def validate_env(env: Optional[Dict[str, str]]) -> Dict[str, str]:
    """
    Validate and sanitize environment variables
    
    Args:
        env: Environment variables dictionary
        
    Returns:
        Sanitized environment dictionary
    """
    if not env:
        return {}
        
    sanitized = {}
    for key, value in env.items():
        if not isinstance(key, str) or not isinstance(value, str):
            raise CommandValidationError(
                "Environment variables must be strings"
            )
            
        sanitized[key.strip()] = value.strip()
        
    return sanitized

def validate_cwd(cwd: Optional[Path]) -> Optional[Path]:
    """
    Validate working directory
    
    Args:
        cwd: Working directory path
        
    Returns:
        Validated directory path
        
    Raises:
        CommandValidationError: If directory is invalid
    """
    if not cwd:
        return None
        
    if not isinstance(cwd, Path):
        cwd = Path(cwd)
        
    if not cwd.exists():
        raise CommandValidationError(f"Working directory does not exist: {cwd}")
    
    if not cwd.is_dir():
        raise CommandValidationError(f"Path is not a directory: {cwd}")
        
    return cwd
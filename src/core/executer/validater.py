from typing import Dict, Optional, Sequence
from pathlib import Path

from .exceptions import CommandValidationError

def validate_command(cmd: Sequence[str]) -> None:
    """Validate command sequence"""
    if not cmd:
        raise CommandValidationError("Empty command sequence")
    if not isinstance(cmd, Sequence) or isinstance(cmd, str):
        raise CommandValidationError(f"Command must be a sequence, got {type(cmd)}")
    for arg in cmd:
        if not isinstance(arg, str):
            raise CommandValidationError(f"Command arg must be sting, got {type(arg)}")
        if any(char in arg for char in ['&', '|', ';', '`', '$', '\\']):
            raise CommandValidationError(f"Invalid command argument: {arg}")
        
def validate_env(env: Optional[Dict[str, str]]) -> Dict[str, str]:
    """Validate and sanitize environments variables"""
    if not env:
        return {}
    
    sanitized = {}
    for k, v in env.items():
        if not isinstance(k, str) or not isinstance(v, str):
            raise CommandValidationError("Environment variables must be strings")
        sanitized[k.strip()] = v.strip()
    return sanitized

def validate_cwd(cwd: Optional[Path]) -> Optional[Path]:
    """Validate working directory"""
    if not cwd:
        return None
    if not isinstance(cwd, Path):
        cwd = Path(cwd)
    if not cwd.exists():
        raise CommandValidationError(f"Working directory does not exist: {cwd}")
    if not cwd.is_dir():
        raise CommandValidationError(f"Path is not a directory: {cwd}")
    return cwd

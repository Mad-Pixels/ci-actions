from typing import Optional

class CommandExecutionError(Exception):
    """Base exception for command execution errors"""
    def __init__(self, message: str, cmd: Optional[str]=None):
        self.cmd = cmd
        super().__init__(message)

class SubprocessError(CommandExecutionError):
    """Subprocess execution error"""
    def __init__(self, cmd: str, returncode: int, stderr: str, stdout: str):
        self.returncode = returncode
        self.stderr = stderr
        self.stdout = stdout

        message = f"Command '{cmd}' failed with return code {returncode}: {stderr}"
        super().__init__(message, cmd)

class CommandValidationError(Exception):
    """Invalid command parameters exception"""
    pass

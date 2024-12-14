from typing import Dict
from contextlib import contextmanager
import os

class EnvironmentManager:
    """Environment manager"""

    @contextmanager
    def temporary_env(self, env_vars: Dict[str, str]):
        """Setup envs"""
        original_env = dict(os.environ)

        try:
            os.environ.update(env_vars)
            yield
        finally:
            os.environ.clear()
            os.environ.update(original_env)

    def sanitize_env(self, env_vars: Dict[str, str]) -> Dict[str, str]:
        """Cleanup temp environments"""
        sanitized = {}
        for key, value in env_vars.items():
            if isinstance(value, str):
                sanitized[key] = value.strip()
        return sanitized

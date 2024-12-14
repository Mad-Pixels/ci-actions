import os
from typing import Optional, Dict
from pathlib import Path

from dotenv import load_dotenv

from .base import EnvironmentBase
from .types import EnvironmentType, LocalEnvironmentConfig

class LocalEnvironment(EnvironmentBase):
    """Local environment implementation"""
    
    def __init__(self, config: LocalEnvironmentConfig, *args, **kwargs):
        super().__init__(config, *args, **kwargs)
        self._loaded_env: Dict[str, str] = {}
        
    @property
    def environment_type(self) -> EnvironmentType:
        return "local"
        
    async def load(self) -> None:
        """Load local environment configuration"""
        env_file = self.config.get("env_file")
        if env_file:
            env_path = Path(env_file)
            if env_path.exists():
                load_dotenv(env_path)
                
        self._loaded_env = dict(os.environ)
        
    async def get_secret(self, name: str) -> Optional[str]:
        """Get local secret"""
        return self._loaded_env.get(name)
        
    async def get_variable(self, name: str) -> Optional[str]:
        """Get local variable"""
        return self._loaded_env.get(name)
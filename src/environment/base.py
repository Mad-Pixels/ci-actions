from typing import Dict, Optional, List
from abc import ABC, abstractmethod

import os
import logging

from .types import EnvironmentType, Secret, Variable, EnvironmentConfig

logger = logging.getLogger(__name__)

class EnvironmentError(Exception):
    """Base class for environment errors"""
    pass

class EnvironmentBase(ABC):
    """Base class for environments"""
    
    def __init__(
        self,
        config: EnvironmentConfig,
        secrets: Optional[List[Secret]] = None,
        variables: Optional[List[Variable]] = None
    ):
        self.config = config
        self.secrets = secrets or []
        self.variables = variables or []
        self._env: Dict[str, str] = {}
        self._masked_values: List[str] = []
        
    @property
    @abstractmethod
    def environment_type(self) -> EnvironmentType:
        """Return environment type"""
        pass
        
    @abstractmethod
    async def load(self) -> None:
        """Load environment configuration"""
        pass
        
    @abstractmethod
    async def get_secret(self, name: str) -> Optional[str]:
        """Get secret by name"""
        pass
        
    @abstractmethod
    async def get_variable(self, name: str) -> Optional[str]:
        """Get variable by name"""
        pass

    async def prepare(self) -> Dict[str, str]:
        """Prepare environment variables"""
        await self.load()
        
        if self.config.get("inherit_env", False):
            self._env.update(os.environ)
            
        for secret in self.secrets:
            value = await self.get_secret(secret["name"])
            if value is None:
                if secret.get("required", False):
                    raise EnvironmentError(f"Required secret {secret['name']} not found")
                value = secret.get("default", "")
            
            if value and self.config.get("mask_secrets", True):
                self._masked_values.append(value)
                
            self._env[secret["env_var"]] = value
            
        for var in self.variables:
            value = await self.get_variable(var["name"])
            if value is None:
                if var.get("required", False):
                    raise EnvironmentError(f"Required variable {var['name']} not found")
                value = var.get("default", "")
                
            self._env[var["env_var"]] = value
        return self._env
        
    def mask_output(self, output: str) -> str:
        """Mask sensitive values in output"""
        if not self.config.get("mask_secrets", True):
            return output
            
        masked = output
        for value in self._masked_values:
            if value:
                masked = masked.replace(value, "***")
        return masked

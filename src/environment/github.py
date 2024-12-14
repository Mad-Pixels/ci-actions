from typing import Optional, Dict

import os

from .base import EnvironmentBase, EnvironmentError
from .types import EnvironmentType, GitHubEnvironmentConfig

class GitHubEnvironment(EnvironmentBase):
    """GitHub Actions environment implementation"""
    
    def __init__(self, config: GitHubEnvironmentConfig, *args, **kwargs):
        super().__init__(config, *args, **kwargs)
        self._secrets: Dict[str, str] = {}
        self._variables: Dict[str, str] = {}
        
    @property
    def environment_type(self) -> EnvironmentType:
        return "github"
        
    async def load(self) -> None:
        """Load GitHub environment configuration"""
        if "GITHUB_ACTIONS" not in os.environ:
            raise EnvironmentError("Not in GitHub Actions environment")
            
        for key, value in os.environ.items():
            if key.startswith("GITHUB_"):
                self._variables[key] = value
                
        workspace = self.config.get("workspace")
        if workspace and workspace != os.environ.get("GITHUB_WORKSPACE"):
            raise EnvironmentError(f"Wrong GitHub workspace: {workspace}")
            
        token_var = self.config.get("token_var", "GITHUB_TOKEN")
        if token_var not in os.environ:
            raise EnvironmentError(f"GitHub token not found in {token_var}")
            
    async def get_secret(self, name: str) -> Optional[str]:
        """Get GitHub secret"""
        return os.environ.get(name)
        
    async def get_variable(self, name: str) -> Optional[str]:
        """Get GitHub variable"""
        if name in self._variables:
            return self._variables[name]
        return os.environ.get(name)

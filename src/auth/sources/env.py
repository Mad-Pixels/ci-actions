from typing import Dict

import os

from ..base import CredentialsSourceBase

class EnvCredentialsSource(CredentialsSourceBase):
    """Environment variables credentials source"""
    
    def __init__(self, prefix: str = ""):
        """
        Initialize source
        
        Args:
            prefix: Optional prefix for env variables
        """
        self.prefix = prefix
        
    async def get_credentials(self) -> Dict[str, str]:
        """Get credentials from environment variables"""
        credentials = {}
        
        for key, value in os.environ.items():
            if self.prefix and not key.startswith(self.prefix):
                continue

            clean_key = key[len(self.prefix):] if self.prefix else key
            credentials[clean_key.lower()] = value
            
        return credentials

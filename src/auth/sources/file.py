from typing import Dict, Optional
from pathlib import Path

import json

from ..base import CredentialsSourceBase

class FileCredentialsSource(CredentialsSourceBase):
    """File-based credentials source"""
    
    def __init__(self, file_path: str, json_key: Optional[str] = None):
        """
        Initialize source
        
        Args:
            file_path: Path to credentials file
            json_key: Optional JSON key to extract credentials from
        """
        self.file_path = Path(file_path)
        self.json_key = json_key
        
    async def get_credentials(self) -> Dict[str, str]:
        """Get credentials from file"""
        if not self.file_path.exists():
            raise FileNotFoundError(f"Credentials file not found: {self.file_path}")
            
        with open(self.file_path) as f:
            content = json.load(f)
            
        if self.json_key:
            if self.json_key not in content:
                raise KeyError(f"Key {self.json_key} not found in credentials file")
            return content[self.json_key]
            
        return content

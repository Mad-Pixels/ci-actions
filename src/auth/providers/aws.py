from typing import Dict

from ..base import CredentialsProviderBase
from ..types import AWSCredentials, ProviderType

class AWSCredentialsProvider(CredentialsProviderBase[AWSCredentials]):
    """AWS credentials provider"""
    
    @property
    def provider_type(self) -> ProviderType:
        return "aws"
        
    def _map_credentials(self, raw: Dict[str, str]) -> AWSCredentials:
        """Map raw credentials to AWS format"""
        creds: AWSCredentials = {
            "aws_access_key_id": raw.get("aws_access_key_id", ""),
            "aws_secret_access_key": raw.get("aws_secret_access_key", ""),
        }
        
        if "aws_session_token" in raw:
            creds["aws_session_token"] = raw["aws_session_token"]
        if "region" in raw:
            creds["region"] = raw["region"]
        if "profile" in raw:
            creds["profile"] = raw["profile"] 
        return creds

    async def get_credentials(self) -> AWSCredentials:
        """Get AWS credentials"""
        raw_creds = await self._source.get_credentials()
        return self._map_credentials(raw_creds)
        
    def validate_credentials(self, credentials: AWSCredentials) -> bool:
        """Validate AWS credentials format"""
        return bool(
            credentials.get("aws_access_key_id") and 
            credentials.get("aws_secret_access_key")
        )

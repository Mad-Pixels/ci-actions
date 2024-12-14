from typing import Dict

from ..base import CredentialsProviderBase
from ..types import GitHubCredentials, ProviderType

class GitHubCredentialsProvider(CredentialsProviderBase[GitHubCredentials]):
    """GitHub credentials provider"""
    
    @property
    def provider_type(self) -> ProviderType:
        return "github"

    def _map_credentials(self, raw: Dict[str, str]) -> GitHubCredentials:
        """Map raw credentials to GitHub format"""
        creds: GitHubCredentials = {
            "token": raw.get("github_token", "")
        }

        if "github_api_url" in raw:
            creds["api_url"] = raw["github_api_url"]
        return creds

    async def get_credentials(self) -> GitHubCredentials:
        """Get GitHub credentials"""
        raw_creds = await self._source.get_credentials()
        return self._map_credentials(raw_creds)

    def validate_credentials(self, credentials: GitHubCredentials) -> bool:
        """Validate GitHub credentials format"""
        return bool(credentials.get("token"))

from typing import Optional, Dict, Generic, TypeVar
from abc import ABC, abstractmethod

from .types import Credentials, ProviderType, CredentialsSource

T = TypeVar('T', bound=Credentials)

class CredentialsSourceBase(ABC):
    """Base class for credential sources"""
    
    @abstractmethod
    async def get_credentials(self) -> Dict[str, str]:
        """Get raw credentials from source"""
        pass

class CredentialsProviderBase(Generic[T], ABC):
    """Base class for credential providers"""
    
    def __init__(self, source: CredentialsSourceBase):
        self._source = source
        
    @property
    @abstractmethod
    def provider_type(self) -> ProviderType:
        """Return provider type"""
        pass
        
    @abstractmethod
    async def get_credentials(self) -> T:
        """Get typed credentials"""
        pass
        
    @abstractmethod
    def validate_credentials(self, credentials: T) -> bool:
        """Validate credentials format"""
        pass

class CredentialsManager:
    """Manager for multiple credential providers"""
    
    def __init__(self, providers: list[CredentialsProviderBase]):
        self._providers = {p.provider_type: p for p in providers}
        
    def get_provider(self, provider_type: ProviderType) -> Optional[CredentialsProviderBase]:
        """Get provider by type"""
        return self._providers.get(provider_type)
        
    async def get_credentials(self, provider_type: ProviderType) -> Optional[Credentials]:
        """Get credentials for specific provider"""
        provider = self.get_provider(provider_type)
        if provider:
            return await provider.get_credentials()
        return None

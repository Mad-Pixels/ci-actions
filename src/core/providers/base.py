from typing import Dict, Optional
from abc import ABC, abstractmethod

class BaseProvider(ABC):
    """Abstract base provider for credentials or auth data."""

    @abstractmethod
    def get_environment(self) -> Dict[str, str]:
        """
        Return environment variables required by the provider.
        For example: {"AWS_ACCESS_KEY_ID": "...", "AWS_SECRET_ACCESS_KEY": "..."}.
        """
        pass

    @abstractmethod
    def get_sensitive(self) -> Dict[str, str]:
        """
        Return sensitive values that must be masked.
        For example: same as environment or subset of it.
        """
        pass

    @abstractmethod
    def validate(self) -> None:
        """
        Optional: Validate that credentials are present and correct.
        Could raise an exception if invalid.
        """
        pass

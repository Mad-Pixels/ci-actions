from typing import Dict, Optional
from abc import ABC, abstractmethod

class BaseProvider(ABC):
    """
    Abstract base class for providers that supply credentials or authorization data.

    Providers inheriting from this class are responsible for:
    - Defining environment variables needed for external services.
    - Specifying sensitive data to be masked for logging or output.
    - Optionally validating the presence or correctness of credentials.

    This class ensures that all concrete providers implement a standard interface.
    """

    @abstractmethod
    def get_environment(self) -> Dict[str, str]:
        """
        Retrieve environment variables required by the provider.

        Returns:
            Dict[str, str]: A dictionary of environment variables required 
                            for provider-specific authentication.
        
        Example:
            For AWS credentials:
            {
                "AWS_ACCESS_KEY_ID": "example-key",
                "AWS_SECRET_ACCESS_KEY": "example-secret"
            }
        """
        pass

    @abstractmethod
    def get_sensitive(self) -> Dict[str, str]:
        """
        Retrieve sensitive values that need to be masked in logs or outputs.

        Returns:
            Dict[str, str]: A dictionary containing sensitive values, 
                            typically identical to or a subset of the environment variables.

        Example:
            {
                "AWS_SECRET_ACCESS_KEY": "example-secret"
            }
        """
        pass

    @abstractmethod
    def validate(self) -> None:
        """
        Validate the presence and correctness of credentials.

        This method can raise exceptions if validation fails, ensuring that the provider's 
        credentials meet the required conditions before use.

        Raises:
            ValueError: If required credentials are missing or invalid.

        Example:
            For AWS credentials, this could check that both keys are non-empty:
            - AWS_ACCESS_KEY_ID
            - AWS_SECRET_ACCESS_KEY
        """
        pass

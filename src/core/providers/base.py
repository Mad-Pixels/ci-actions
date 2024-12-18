from typing import Dict, List, Tuple
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
        """
        pass

    @abstractmethod
    def get_sensitive(self) -> Dict[str, str]:
        """
        Retrieve sensitive values that need to be masked in logs or outputs.

        Returns:
            Dict[str, str]: A dictionary containing sensitive values, 
                            typically identical to or a subset of the environment variables.
        """
        pass

    @abstractmethod
    def validate(self) -> None:
        """
        Validate the presence and correctness of credentials.

        Raises:
            ValueError: If required credentials are missing or invalid.
        """
        pass

    def get_predefined_masked_objects(self) -> List[str]:
        """
        Retrieve a list of predefined sensitive objects that should be masked.

        Returns:
            List[str]: A list of strings representing sensitive objects to be masked.
        """
        return []

    def _generate_env_and_sensitive(self, credentials: Dict[str, str]) -> Tuple[Dict[str, str], Dict[str, str]]:
        """
        Generates environment and sensitive dictionaries based on provided credentials.

        Args:
            credentials (Dict[str, str]): A dictionary of credentials.

        Returns:
            Tuple[Dict[str, str], Dict[str, str]]: A tuple containing environment variables and sensitive data.
        """
        environment = credentials.copy()
        sensitive = credentials.copy()
        return environment, sensitive

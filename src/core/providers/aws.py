from typing import Dict, Optional

from .base import BaseProvider

class AWSProvider(BaseProvider):
    """
    AWS Provider for managing AWS credentials.

    This provider supplies AWS environment variables and sensitive data for 
    authentication with AWS services.

    It requires AWS credentials, specifically:
    - AWS_ACCESS_KEY_ID
    - AWS_SECRET_ACCESS_KEY

    Optionally, it supports:
    - AWS_SESSION_TOKEN
    """

    def __init__(
        self,
        access_key_id: str,
        secret_access_key: str,
        session_token: Optional[str]=None
    ):
        """
        Initialize the AWSProvider with credentials.

        Args:
            access_key_id (str): AWS access key ID.
            secret_access_key (str): AWS secret access key.
            session_token (Optional[str]): Optional session token for temporary credentials.
        """
        self.credentials = {
            "AWS_ACCESS_KEY_ID": access_key_id,
            "AWS_SECRET_ACCESS_KEY": secret_access_key
        }
        if session_token:
            self.credentials["AWS_SESSION_TOKEN"] = session_token

    def get_environment(self) -> Dict[str, str]:
        """
        Return environment variables for AWS credentials.

        Returns:
            Dict[str, str]: A dictionary with AWS credentials for the environment.
        """
        environment, _ = self._generate_env_and_sensitive(self.credentials)
        return environment
    
    def get_sensitive(self) -> Dict[str, str]:
        """
        Return sensitive values for masking.

        Returns:
            Dict[str, str]: A dictionary with sensitive AWS credentials.
        """
        _, sensitive = self._generate_env_and_sensitive(self.credentials)
        return sensitive
    
    def validate(self) -> None:
        """
        Validate that required AWS credentials are provided.

        Raises:
            ValueError: If AWS credentials are incomplete or missing.
        """
        if not self.credentials.get("AWS_ACCESS_KEY_ID") or not self.credentials.get("AWS_SECRET_ACCESS_KEY"):
            raise ValueError("AWS credentials are incomplete.")

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
        self.access_key_id = access_key_id
        self.secret_access_key = secret_access_key
        self.session_token = session_token

    def get_environment(self) -> Dict[str, str]:
        """
        Return environment variables for AWS credentials.

        Returns:
            Dict[str, str]: A dictionary with AWS credentials for the environment.
        """
        env = {
            "AWS_ACCESS_KEY_ID": self.access_key_id,
            "AWS_SECRET_ACCESS_KEY": self.secret_access_key
        }
        if self.session_token:
            env["AWS_SESSION_TOKEN"] = self.session_token
        return env
    
    def get_sensitive(self) -> Dict[str, str]:
        """
        Return sensitive values for masking.

        Returns:
            Dict[str, str]: A dictionary with sensitive AWS credentials.
        """
        sens = {
            "AWS_ACCESS_KEY_ID": self.access_key_id,
            "AWS_SECRET_ACCESS_KEY": self.secret_access_key
        }
        if self.session_token:
            sens["AWS_SESSION_TOKEN"] = self.session_token
        return sens
    
    def validate(self) -> None:
        """
        Validate that required AWS credentials are provided.

        Raises:
            ValueError: If AWS credentials are incomplete or missing.
        """
        if not self.access_key_id or not self.secret_access_key:
            raise ValueError("AWS credentials are incomplete.")

from typing import Dict, Optional

from .base import BaseProvider

class AWSProvider(BaseProvider):
    """
    Example AWS Provider that expects AWS credentials.
    AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, optional AWS_SESSION_TOKEN.
    """

    def __init__(
        self,
        access_key_id: str,
        secret_access_key: str,
        session_token: Optional[str]=None
    ):
        self.access_key_id = access_key_id
        self.secret_access_key = secret_access_key
        self.session_token = session_token

    def get_environment(self) -> Dict[str, str]:
        env = {
            "AWS_ACCESS_KEY_ID": self.access_key_id,
            "AWS_SECRET_ACCESS_KEY": self.secret_access_key
        }
        if self.session_token:
            env["AWS_SESSION_TOKEN"] = self.session_token
        return env
    
    def get_sensitive(self) -> Dict[str, str]:
        sens = {
            "AWS_ACCESS_KEY_ID": self.access_key_id,
            "AWS_SECRET_ACCESS_KEY": self.secret_access_key
        }
        if self.session_token:
            sens["AWS_SESSION_TOKEN"] = self.session_token
        return sens
    
    def validate(self) -> None:
        if not self.access_key_id or not self.secret_access_key:
            raise ValueError("AWS credentials are incomplete.")

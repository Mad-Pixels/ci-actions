from typing import Optional
from abc import ABC

import aioboto3
import logging

from ..types.aws import AWSCredentials

logger = logging.getLogger(__name__)

class AWSBase(ABC):
    """Base class for AWS services"""

    def __init__(
        self,
        credentials: Optional[AWSCredentials] = None,
        session: Optional[aioboto3.Session] = None
    ):
        self._credentials = credentials
        self._session = session or self._create_session()

    def _create_session(self) -> aioboto3.Session:
        """Creates aioboto3 session with provided credentials"""
        if self._credentials:
            return aioboto3.Session(
                aws_access_key_id=self._credentials.aws_access_key_id,
                aws_secret_access_key=self._credentials.aws_secret_access_key,
                aws_session_token=self._credentials.aws_session_token,
                region_name=self._credentials.region_name,
                profile_name=self._credentials.profile_name
            )
        return aioboto3.Session()

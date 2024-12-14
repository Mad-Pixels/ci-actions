from typing import TypedDict, Optional, Literal, Union
from typing_extensions import NotRequired

ProviderType = Literal["aws", "github"]

class AWSCredentials(TypedDict):
    """AWS credentials"""
    aws_access_key_id: str
    aws_secret_access_key: str
    aws_session_token: NotRequired[str]
    region: NotRequired[str]
    profile: NotRequired[str]

class GitHubCredentials(TypedDict):
    """GitHub credentials"""
    token: str
    api_url: NotRequired[str]

Credentials = Union[AWSCredentials, GitHubCredentials]

class CredentialsSource(TypedDict):
    """Base credentials source configuration"""
    type: Literal["env", "file"]
    env_prefix: NotRequired[str]
    file_path: NotRequired[str]
    json_key: NotRequired[str]

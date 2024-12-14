from typing import TypedDict, Optional, Literal, Union
from dataclasses import dataclass
from enum import Enum

from typing_extensions import NotRequired

AWSRegion = Literal[
    "us-east-1", "eu-central-1"
]

AWSTag = TypedDict("AWSTag", {
    "Key": str,
    "Value": str
})

AWSTagList = list[AWSTag]

@dataclass
class AWSCredentials:
    """AWS credentials configuration"""
    aws_access_key_id: Optional[str] = None
    aws_secret_access_key: Optional[str] = None
    aws_session_token: Optional[str] = None
    region_name: Optional[str] = None
    profile_name: Optional[str] = None

class Architecture(str, Enum):
    """Supported arch"""
    X86_64 = "x86_64"
    ARM64 = "arm64"

class S3UploadResult(TypedDict):
    """S3 response result"""
    Bucket: str
    Key: str
    VersionId: NotRequired[str]
    ETag: str
    Success: bool

class CloudFrontInvalidation(TypedDict):
    """CloudFront invalidation response"""
    Id: str
    Status: Literal["InProgress", "Completed", "Failed"]
    DistributionId: str
    CreateTime: str
    Paths: list[str]

class LambdaFunctionState(TypedDict):
    """Lambda function state"""
    State: Literal["Pending", "Active", "Inactive", "Failed"]
    LastUpdateStatus: Optional[str]
    LastUpdateStatusReason: NotRequired[str]
    CodeSha256: str
    Version: str
    ImageUri: NotRequired[str]

class AWSError(TypedDict):
    """Error object"""
    Code: str
    Message: str
    RequestId: str
    Operation: str

class AWSClientConfig(TypedDict):
    """AWS config"""
    Region: AWSRegion
    Profile: NotRequired[str]
    Credentials: NotRequired[AWSCredentials]
    EndpointUrl: NotRequired[str]
    Verify: NotRequired[bool]
    Timeout: NotRequired[float]

class S3ObjectMetadata(TypedDict):
    """S3 metadata object"""
    ContentType: NotRequired[str]
    ContentLength: int
    ETag: str
    LastModified: str
    CacheControl: NotRequired[str]
    ContentEncoding: NotRequired[str]
    ContentDisposition: NotRequired[str]
    ServerSideEncryption: NotRequired[str]
    StorageClass: NotRequired[str]
    Metadata: NotRequired[dict[str, str]]

class LambdaUpdateResponse(TypedDict):
    """Lambda function update response"""
    FunctionName: str
    FunctionArn: str
    Runtime: str
    Role: str
    Handler: NotRequired[str]
    CodeSize: int
    Timeout: int
    MemorySize: int
    LastModified: str
    CodeSha256: str
    Version: str
    Environment: NotRequired[dict[str, str]]
    ImageUri: NotRequired[str]
    Architectures: list[Architecture]

class S3OperationResponse(TypedDict):
    """S3 op response"""
    ResponseMetadata: dict[str, Union[str, int]]
    RequestId: str
    Success: bool
    Error: NotRequired[AWSError]

class CloudFrontOperationResponse(TypedDict):
    """CloudFront op response"""
    Id: str
    Status: str
    Location: str
    InvalidationBatch: dict[str, Union[str, list[str]]]
    ResponseMetadata: dict[str, Union[str, int]]

class GetFunctionResponse(TypedDict):
    """Lambda function response"""
    Configuration: LambdaFunctionState
    Code: dict[str, str]
    Tags: NotRequired[dict[str, str]]

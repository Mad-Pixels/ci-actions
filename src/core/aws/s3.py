from typing import Optional, Union
from pathlib import Path

import logging

from ..types.aws import (
    S3UploadResult,
    S3ObjectMetadata
)
from .base import AWSBase

logger = logging.getLogger(__name__)

class S3Client(AWSBase):
    """Client for s3"""

    async def upload_file(
        self,
        local_path: Union[str, Path],
        bucket: str,
        s3_key: str,
        overwrite: bool = True,
        content_type: Optional[str] = None
    ) -> S3UploadResult:
        """
        Uploads file to S3 with optional overwrite

        Args:
            local_path: Path to local file
            bucket: S3 bucket name
            s3_key: Target S3 key
            overwrite: Whether to overwrite existing file
            content_type: Optional content type
            
        Returns:
            S3UploadResult containing upload details
            
        Raises:
            FileNotFoundError: If local file doesn't exist
        """
        local_path = Path(local_path)
        if not local_path.exists():
            raise FileNotFoundError(f"File not found: {local_path}")

        async with self._session.client('s3') as client:
            if not overwrite:
                try:
                    metadata: S3ObjectMetadata = await client.head_object(
                        Bucket=bucket, 
                        Key=s3_key
                    )
                    logger.info(f"File {s3_key} already exists in bucket {bucket}")
                    return {
                        "Bucket": bucket,
                        "Key": s3_key,
                        "ETag": metadata["ETag"],
                        "Success": False
                    }
                except client.exceptions.ClientError:
                    pass

            extra_args = {}
            if content_type:
                extra_args['ContentType'] = content_type

            try:
                await client.upload_file(
                    str(local_path),
                    bucket,
                    s3_key,
                    ExtraArgs=extra_args
                )
                metadata: S3ObjectMetadata = await client.head_object(
                    Bucket=bucket,
                    Key=s3_key
                )
                logger.info(f"Successfully uploaded {local_path} to s3://{bucket}/{s3_key}")
                return {
                    "Bucket": bucket,
                    "Key": s3_key,
                    "ETag": metadata["ETag"],
                    "VersionId": metadata.get("VersionId"),
                    "Success": True
                }
                
            except Exception as e:
                logger.error(f"Failed to upload file: {e}")
                raise

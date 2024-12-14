from typing import List, Optional

import time
import logging

from ..types.aws import (
    CloudFrontInvalidation,
    CloudFrontOperationResponse
)
from .base import AWSBase

logger = logging.getLogger(__name__)

class CloudFront(AWSBase):
    """Client for CloudFront"""

    async def create_invalidation(
        self,
        distribution_id: str,
        paths: List[str],
        caller_reference: Optional[str] = None
    ) -> CloudFrontInvalidation:
        """
        Creates CloudFront invalidation

        Args:
            distribution_id: CloudFront distribution ID
            paths: List of paths to invalidate
            caller_reference: Optional caller reference

        Returns:
            CloudFrontInvalidation with invalidation details
            
        Raises:
            ValueError: If no paths provided
        """
        if not paths:
            raise ValueError("No paths provided for invalidation")
        
        async with self._session.client('cloudfront') as client:
            try:
                response: CloudFrontOperationResponse = await client.create_invalidation(
                    DistributionId=distribution_id,
                    InvalidationBatch={
                        'Paths': {
                            'Quantity': len(paths),
                            'Items': paths
                        },
                        'CallerReference': caller_reference or str(time.time())
                    }
                )

                invalidation_id = response['Id']
                logger.info(
                    f"Created invalidation {invalidation_id} for distribution {distribution_id}"
                )
                return {
                    'Id': invalidation_id,
                    'Status': 'InProgress',
                    'DistributionId': distribution_id,
                    'CreateTime': response['ResponseMetadata'].get('HTTPHeaders', {}).get('date', str(time.time())),
                    'Paths': paths
                }

            except Exception as e:
                logger.error(f"Failed to create invalidation: {e}")
                raise

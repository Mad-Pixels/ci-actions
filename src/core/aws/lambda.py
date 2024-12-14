from typing import Optional, Tuple

import time
import logging
import asyncio

from ..types.aws import (
    Architecture,
    LambdaFunctionState,
    LambdaUpdateResponse,
    GetFunctionResponse
)
from .base import AWSBase

logger = logging.getLogger(__name__)

class LambdaClient(AWSBase):
    """Client for Lambda"""
    
    async def update_function_code(
        self,
        function_name: str,
        image_uri: str,
        publish: bool = False,
        architectures: Optional[list[Architecture]] = None,
    ) -> LambdaUpdateResponse:
        """
        Updates Lambda function code using container image

        Args:
            function_name: Name or ARN of the Lambda function
            image_uri: URI of the container image
            publish: Whether to publish new version
            architectures: List of compatible architectures
            
        Returns:
            LambdaUpdateResponse with function details
        """
        async with self._session.client('lambda') as client:
            try:
                update_params = {
                    'FunctionName': function_name,
                    'ImageUri': image_uri,
                    'Publish': publish
                }
                if architectures:
                    update_params['Architectures'] = [arch.value for arch in architectures]

                response = await client.update_function_code(**update_params)
                logger.info(
                    f"Updated function {function_name} with image {image_uri}, version: {response['Version']}"
                )
                return response
                
            except client.exceptions.ResourceNotFoundException:
                logger.error(f"Lambda function {function_name} not found")
                raise
            except client.exceptions.InvalidParameterValueException as e:
                logger.error(f"Invalid parameters for function update: {e}")
                raise
            except Exception as e:
                logger.error(f"Failed to update function code: {e}")
                raise

    async def wait_for_function_update(
        self,
        function_name: str,
        timeout: int = 300,
        check_interval: int = 5
    ) -> Tuple[bool, Optional[LambdaFunctionState]]:
        """
        Waits for Lambda function update to complete

        Args:
            function_name: Name or ARN of the Lambda function
            timeout: Maximum time to wait in seconds
            check_interval: Time between checks in seconds
            
        Returns:
            Tuple[bool, Optional[LambdaFunctionState]]: True if successful, with final state
        """
        start_time = time.time()
        async with self._session.client('lambda') as client:
            while (time.time() - start_time) < timeout:
                try:
                    response: GetFunctionResponse = await client.get_function(
                        FunctionName=function_name
                    )
                    
                    state = response['Configuration']['State']
                    if state == 'Active':
                        logger.info(f"Function {function_name} update completed")
                        return True, response['Configuration']
                    elif state == 'Failed':
                        last_error = response['Configuration'].get('LastUpdateStatus', 'Unknown error')
                        logger.error(f"Function update failed: {last_error}")
                        return False, response['Configuration']
                    
                    logger.debug(f"Function state: {state}, waiting...")
                    await asyncio.sleep(check_interval)
                    
                except Exception as e:
                    logger.error(f"Error checking function state: {e}")
                    raise
                
            logger.warning(f"Timed out waiting for function {function_name} update")
            return False, None

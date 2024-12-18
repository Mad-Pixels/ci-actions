from typing import Dict, Any, Optional, AsyncGenerator
from contextlib import asynccontextmanager

import os
import json
import asyncio
import logging

from .masker import OutputMasker

@asynccontextmanager
async def override_env(env: Dict[str, str]):
    """
    Context manager for temporarily modifying environment variables.

    Ensures that the original environment variables are restored after the operation.

    Args:
        env: A dictionary containing environment variables to temporarily override.

    Usage:
        with override_env({"KEY": "VALUE"}):
            # The environment variable KEY will temporarily be set to VALUE
            ...
        # Environment is restored to its original state after the block.
    """
    origin = {}
    for key, value in env.items():
        origin[key] = os.environ.get(key)
        os.environ[key] = value
    try:
        yield
    finally:
        for key in env:
            if origin[key] is None:
                del os.environ[key]
            else:
                os.environ[key] = origin[key]

async def stream_lines(
    stream: asyncio.StreamReader,
    logger: logging.Logger,
    name: str,
    processor: Optional[OutputMasker] = None,
) -> AsyncGenerator[str, None]:
    """
    Asynchronous generator that reads the stream line by line and yields each line as it arrives.
    
    Args:
        stream: An asyncio.StreamReader object to read from.
        logger: A logging.Logger instance to log each line (masked if applicable).
        name: A string identifying the stream (e.g., "STDOUT" or "STDERR").
        processor: An optional OutputMasker instance for masking sensitive data.

    Yields:
        Lines from the stream as they become available.

    Raises:
        Exception: If an error occurs during stream reading.
    """
    try:
        while True:
            line = await stream.readline()
            if not line:
                break
            decoded = line.decode(errors="replace")
            line_for_log = processor.mask(decoded) if processor else decoded
            logger.debug(f"[{name}] {line_for_log.strip()}")
            yield decoded
    except Exception as e:
        logger.error(f"Failed read stream {name}: {e}", exc_info=True)
        raise

def str_to_dict(json_str: str, logger: logging.Logger) -> Dict[str, Any]:
    """
    Parses a JSON-formatted string into a dictionary.

    Args:
        json_str: A string containing JSON-formatted data.

    Returns:
        A dictionary representation of the JSON string.
        Returns an empty dictionary if the string is invalid.

    Logs:
        Logs an error if parsing fails.
    """
    try:
        return json.loads(json_str)
    except json.JSONDecodeError as e:
        logger.error(f"Failed to parse str as JSON: {e}")
        return {}

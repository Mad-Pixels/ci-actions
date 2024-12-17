from typing import Dict, Any, Optional
from contextlib import contextmanager

import os
import json
import asyncio
import logging

from .masker import OutputMasker

@contextmanager
def override_env(env: Dict[str, str]):
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
    origin = dict(os.environ)
    try:
        os.environ.update(env)
        yield
    finally:
        os.environ.clear()
        os.environ.update(origin)

async def read_stream(
    stream: asyncio.StreamReader,
    logger: logging.Logger,
    name: str,
    processor: Optional[OutputMasker] = None,
) -> str:
    """
    Reads lines asynchronously from a stream and applies masking if necessary.

    Args:
        stream: An asyncio.StreamReader object to read from.
        logger: A logging.Logger instance to log each line (masked if applicable).
        name: A string identifying the stream (e.g., "STDOUT" or "STDERR").
        processor: An optional OutputMasker instance for masking sensitive data.

    Returns:
        A string containing the full output read from the stream.

    Raises:
        Exception: If an error occurs during stream reading.
    """
    output = []
    try:
        while True:
            line = await stream.readline()
            if not line:
                break
            decoded = line.decode(errors="replace")

            line_for_log = processor.mask(decoded) if processor else decoded
            logger.debug(f"[{name}] {line_for_log.strip()}")
            output.append(decoded)
        return ''.join(output)
    except Exception as e:
        logger.error(f"Error reading from {name} stream: {e}")
        raise

def str_to_dict(json_str: str) -> Dict[str, Any]:
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
        logging.error("Failed to parse str as JSON: %s{e}")
        return {}

from typing import Dict, Any
from contextlib import contextmanager

import os
import json
import asyncio
import logging

@contextmanager
def override_env(env: Dict[str, str]):
    """Context manager for safely modifying environment variables"""
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
) -> str:
    """Read from a stream asynchronously with logging"""
    output = []
    try:
        while True:
            line = await stream.readline()
            if not line:
                break
            
            decoded = line.decode(errors='replace')
            logger.debug(f"[{name}] {decoded.strip()}")
            output.append(decoded)
        return ''.join(output)
    except Exception as e:
        logger.error(f"Error reading from {name} stream: {e}")
        raise

def str_to_dict(json_str: str) -> Dict[str, Any]:
    """Parse string as JSON to dict"""
    try:
        return json.loads(json_str)
    except json.JSONDecodeError as e:
        logging.error("Failed to parse str as JSON: %s{e}")
        return {}

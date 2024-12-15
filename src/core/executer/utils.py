from typing import Dict
from contextlib import contextmanager

import os
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

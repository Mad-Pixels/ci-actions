from typing import Dict, Any, Optional
from contextlib import contextmanager

import os
import json
import asyncio
import logging

from .masker import OutputMasker

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
    processor: Optional[OutputMasker] = None,
) -> str:
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
    """Parse string as JSON to dict"""
    try:
        return json.loads(json_str)
    except json.JSONDecodeError as e:
        logging.error("Failed to parse str as JSON: %s{e}")
        return {}

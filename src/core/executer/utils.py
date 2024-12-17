from typing import Dict, Any, Optional, AsyncGenerator
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

async def stream_lines(
    stream: asyncio.StreamReader,
    logger: logging.Logger,
    name: str,
    processor: Optional[OutputMasker] = None,
) -> AsyncGenerator[str, None]:
    """
    Asynchronous generator that reads the stream line by line and yields each line as it arrives.

    This allows processing the output in real-time.
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
        logging.error(f"Failed to parse str as JSON: {e}")
        return {}

async def run_command_line_stream(
    cmd: list[str],
    env: Dict[str, str],
    cwd: Optional[str],
    processor: Optional[OutputMasker],
    logger: logging.Logger
) -> AsyncGenerator[str, None]:
    """
    Run a command and yield stdout lines as they become available.

    Args:
        cmd: The command to execute as a list of strings.
        env: Environment variables for the command.
        cwd: Working directory for the command execution.
        processor: Optional OutputMasker for masking sensitive data.
        logger: Logger instance for logging.

    Yields:
        Lines from stdout in real-time.
    """
    process = await asyncio.create_subprocess_exec(
        *cmd,
        stdout=asyncio.subprocess.PIPE,
        stderr=asyncio.subprocess.PIPE,
        env={**os.environ, **env},
        cwd=cwd
    )

    async for line in stream_lines(process.stdout, logger, "STDOUT", processor):
        yield line

    returncode = await process.wait()
    stderr_data = await process.stderr.read()
    stderr_decoded = stderr_data.decode(errors="replace")

    if returncode != 0:
        logger.error(f"Command {cmd} failed with return code {returncode}: {stderr_decoded}")

"""Common test fixtures and configuration"""
import pytest
import logging
import os

@pytest.fixture(autouse=True)
def setup_logging():
    """Configure logging for tests"""
    logging.basicConfig(
        level=logging.DEBUG,
        format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )

@pytest.fixture
def temp_env():
    """Provide clean environment for tests"""
    original_env = dict(os.environ)
    yield os.environ
    os.environ.clear()
    os.environ.update(original_env)
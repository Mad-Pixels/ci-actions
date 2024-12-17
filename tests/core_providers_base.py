from src.core.providers.base import BaseProvider

import pytest

def test_base_provider_is_abstract():
    class DummyProvider(BaseProvider):
        pass
    
    with pytest.raises(TypeError, match="Can't instantiate abstract class DummyProvider"):
        DummyProvider()

def test_base_provider_methods():
    class DummyProvider(BaseProvider):
        def get_environment(self):
            return {"KEY": "value"}
        def get_sensitive(self):
            return {"KEY": "value"}
        def validate(self):
            pass

    provider = DummyProvider()
    env = provider.get_environment()
    sensitive = provider.get_sensitive()
    assert env == {"KEY": "value"}
    assert sensitive == {"KEY": "value"}
    provider.validate()

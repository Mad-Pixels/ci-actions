from src.core.providers.aws import AWSProvider

import pytest

def test_aws_provider_minimal_credentials():
    provider = AWSProvider(
        access_key_id="AKIAXXXX",
        secret_access_key="SECRETXXXX"
    )
    env = provider.get_environment()
    sensitive = provider.get_sensitive()

    assert env == {
        "AWS_ACCESS_KEY_ID": "AKIAXXXX",
        "AWS_SECRET_ACCESS_KEY": "SECRETXXXX"
    }
    assert sensitive == {
        "AWS_ACCESS_KEY_ID": "AKIAXXXX",
        "AWS_SECRET_ACCESS_KEY": "SECRETXXXX"
    }

def test_aws_provider_with_session_token():
    provider = AWSProvider(
        access_key_id="AKIAXXXX",
        secret_access_key="SECRETXXXX",
        session_token="SESSIONXXXX"
    )
    env = provider.get_environment()
    sensitive = provider.get_sensitive()

    assert env == {
        "AWS_ACCESS_KEY_ID": "AKIAXXXX",
        "AWS_SECRET_ACCESS_KEY": "SECRETXXXX",
        "AWS_SESSION_TOKEN": "SESSIONXXXX"
    }
    assert sensitive == {
        "AWS_ACCESS_KEY_ID": "AKIAXXXX",
        "AWS_SECRET_ACCESS_KEY": "SECRETXXXX",
        "AWS_SESSION_TOKEN": "SESSIONXXXX"
    }

def test_aws_provider_validation_success():
    provider = AWSProvider(
        access_key_id="AKIAXXXX",
        secret_access_key="SECRETXXXX"
    )
    provider.validate()

def test_aws_provider_validation_fail():
    provider = AWSProvider(
        access_key_id="",
        secret_access_key=""
    )
    with pytest.raises(ValueError, match="AWS credentials are incomplete."):
        provider.validate()

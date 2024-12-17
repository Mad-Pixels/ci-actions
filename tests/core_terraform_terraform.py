from unittest.mock import AsyncMock, MagicMock
from pathlib import Path

import pytest

from core.terraform.terraform import Terraform
from core.terraform.types import TerraformAction  
from core.terraform.exceptions import TerraformExecutionError
from core.executer.executer import IsolateExecuter
from core.executer.masker import OutputMasker
from core.providers.aws import AWSProvider

pytestmark = pytest.mark.asyncio

class AsyncIterGenerator:
    def __init__(self, data):
        self.data = data
        self.i = 0

    def __aiter__(self):
        return self
    
    async def __anext__(self):
        if self.i >= len(self.data):
            raise StopAsyncIteration
        value = self.data[self.i]  
        self.i += 1
        return value

async def test_terraform_init(mock_executer):
    # Создаем Terraform объект с base_env и sensitive параметрами
    terraform = Terraform(
        base_cwd=Path("/tmp"),
        executer=mock_executer,
        base_env={"BASE_VAR": "value"},
        sensitive={"SECRET": "supersecret"}
    )

    # Переменная для хранения фактических аргументов вызова
    captured_env = None

    def mock_stream(*args, **kwargs):
        nonlocal captured_env
        # Сохраняем env из вызова
        captured_env = kwargs.get('env', {})
        return AsyncIterGenerator(["Initialization complete\n"])

    mock_executer.execute_stream.side_effect = mock_stream

    output = []
    async for line in terraform.init():
        output.append(line)

    # Проверки
    assert output == ["Initialization complete\n"]
    assert captured_env == {"BASE_VAR": "value", "SECRET": "supersecret"}, \
           f"Expected {{'BASE_VAR': 'value', 'SECRET': 'supersecret'}}, got {captured_env}"

async def test_terraform_plan(mock_executer):
    # Инициализируем с base_env
    terraform = Terraform(
        base_cwd=Path("/project"),
        executer=mock_executer,
        base_env={"AWS_ACCESS_KEY_ID": "dummy", "AWS_SECRET_ACCESS_KEY": "dummysecret"}
    )

    captured_env = None
    def mock_stream(*args, **kwargs):
        nonlocal captured_env
        captured_env = kwargs.get('env', {})
        return AsyncIterGenerator(["Plan executed successfully\n"])

    mock_executer.execute_stream.side_effect = mock_stream

    tf_vars = {"region": "us-west-2"}
    args = ["-out=planfile"]
    
    output = []
    async for line in terraform.plan(args=args, tf_vars=tf_vars):
        output.append(line)

    expected_env = {
        "AWS_ACCESS_KEY_ID": "dummy",
        "AWS_SECRET_ACCESS_KEY": "dummysecret",
        "TF_VAR_region": "us-west-2"
    }

    assert output == ["Plan executed successfully\n"]
    assert captured_env == expected_env, f"Expected {expected_env}, got {captured_env}"

async def test_terraform_apply(mock_executer):
    mock_executer.execute_stream.side_effect = lambda *args, **kwargs: AsyncIterGenerator([
        "Applying changes...\n",
        "Error occurred\n"
    ])

    terraform = Terraform(
        base_cwd=Path("/infra"),
        executer=mock_executer,
        base_env={"BASE_VAR": "value"}
    )

    output = []
    with pytest.raises(TerraformExecutionError, match="Error occurred"):
        async for line in terraform.apply():
            output.append(line)
            if "Error" in line:
                raise TerraformExecutionError(TerraformAction.APPLY.value, line.strip())

async def test_terraform_workspace(mock_executer):
    mock_executer.execute_stream.side_effect = lambda *args, **kwargs: AsyncIterGenerator(["Workspace selected\n"])

    terraform = Terraform(
        base_cwd=Path("/workspaces"),
        executer=mock_executer
    )

    output = []
    async for line in terraform.workspace(action="select", name="dev"):
        output.append(line)

    assert output == ["Workspace selected\n"]

    mock_executer.execute_stream.assert_called_once_with(
        ["terraform", "workspace", "select", "dev"],
        env={},
        cwd=Path("/workspaces"),
        mask=True
    )

async def test_terraform_output(mock_executer):
    mock_executer.execute_stream.side_effect = lambda *args, **kwargs: AsyncIterGenerator([
        '{"bucket_name":{"value":"my-bucket"},"region":{"value":"us-east-1"}}\n'
    ])

    terraform = Terraform(
        base_cwd=Path("/env"),
        executer=mock_executer
    )

    result = await terraform.output()
    assert result == {
        "bucket_name": {"value": "my-bucket"},
        "region": {"value": "us-east-1"}
    }

    mock_executer.execute_stream.assert_called_once_with(
        ["terraform", "output", "-json"],
        env={},
        cwd=Path("/env"),
        mask=True
    )

async def test_terraform_with_aws_provider():
    provider = AWSProvider(
        access_key_id="AKIAXXXX",
        secret_access_key="SECRETXXXX"
    )
    masker = OutputMasker()
    masker.sensitive("SECRETXXXX")
    masker.sensitive("myextrasecret")

    executer = AsyncMock(spec=IsolateExecuter)
    executer._processor = masker

    executer.execute_stream.side_effect = lambda *args, **kwargs: AsyncIterGenerator([
        "Applied successfully with secret: SECRETXXXX\n"
    ])

    terraform = Terraform(
        base_cwd=Path("/aws_project"),
        executer=executer,
        provider=provider,
        sensitive={"EXTRA_SECRET": "myextrasecret"}
    )

    masked_output = []
    async for line in terraform.apply():
        masked_line = masker.mask(line)
        masked_output.append(masked_line)

    assert masked_output == ["Applied successfully with secret: ******\n"]

    executer.execute_stream.assert_called_once_with(
        ["terraform", "apply", "-input=false", "-auto-approve"],
        env={
            "AWS_ACCESS_KEY_ID": "AKIAXXXX",
            "AWS_SECRET_ACCESS_KEY": "SECRETXXXX",
            "EXTRA_SECRET": "myextrasecret"
        },
        cwd=Path("/aws_project"),
        mask=True
    )
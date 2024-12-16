from unittest.mock import AsyncMock
from pathlib import Path

import pytest

from src.core.terraform.terraform import Terraform
from src.core.terraform.types import TerraformAction
from src.core.terraform.exceptions import TerraformExecutionError
from src.core.executer.base import ExecutionResult

pytestmark = pytest.mark.asyncio

async def test_terraform_init(mock_executer):
    terraform = Terraform(
        base_cwd=Path("/tmp"),
        executer=mock_executer,
        base_env={"BASE_VAR": "value"},
        sensitive={"SECRET": "supersecret"}
    )

    result = await terraform.init()
    assert result.action == TerraformAction.INIT
    assert result.result.status == 0
    assert "default stdout" in result.result.stdout

    mock_executer.execute.assert_awaited_once_with(
        ["terrafrom", "init"],
        env={"BASE_VAR": "value"},
        cwd=Path("/tmp"),
        mask=True
    )

async def test_terraform_plan(mock_executer):
    mock_executer.execute.return_value = ExecutionResult(
        status=0,
        stdout="plan stdout",
        stderr="plan stderr"
    )

    terraform = Terraform(
        base_cwd=Path("/project"),
        executer=mock_executer,
        base_env={"AWS_ACCESS_KEY_ID":"dummy", "AWS_SECRET_ACCESS_KEY":"dummysecret"}
    )

    tf_vars = {"region": "us-west-2"}
    args = ["-out=planfile"]
    result = await terraform.plan(args=args, tf_vars=tf_vars)
    assert result.action == TerraformAction.PLAN
    assert result.result.stdout == "plan stdout"

    expected_env = {
        "AWS_ACCESS_KEY_ID":"dummy",
        "AWS_SECRET_ACCESS_KEY":"dummysecret",
        "TF_VAR_region":"us-west-2"
    }
    mock_executer.execute.assert_awaited_once_with(
        ["terraform", "plan", "-input=false", "-out=planfile"],
        env=expected_env,
        cwd=Path("/project"),
        mask=True
    )

async def test_terraform_apply(mock_executer):
    mock_executer.execute.side_effect = Exception("Some error")

    terraform = Terraform(
        base_cwd=Path("/infra"),
        executer=mock_executer,
        base_env={"BASE_VAR":"value"}
    )

    with pytest.raises(TerraformExecutionError) as exc_info:
        await terraform.apply()

    assert "Terraform apply error: Some error" in str(exc_info.value)
    mock_executer.execute.assert_awaited_once_with(
        ["terraform", "apply", "-input=false", "-auto-approve"],
        env={"BASE_VAR":"value"},
        cwd=Path("/infra"),
        mask=True
    )

async def test_terraform_workspace(mock_executer):
    mock_executer.execute.return_value = ExecutionResult(
        status=0,
        stdout="workspace selected",
        stderr=""
    )

    terraform = Terraform(
        base_cwd=Path("/workspaces"),
        executer=mock_executer
    )

    result = await terraform.workspace(action="select", name="dev")
    assert result.action == TerraformAction.WORKSPACE
    assert result.result.stdout == "workspace selected"

    mock_executer.execute.assert_awaited_once_with(
        ["terraform", "workspace", "select", "dev"],
        env={},
        cwd=Path("/workspaces"),
        mask=True
    )

async def test_terraform_output(mock_executer):
    mock_executer.execute.return_value = ExecutionResult(
        status=0,
        stdout='{"bucket_name":{"value":"my-bucket"},"region":{"value":"us-east-1"}}',
        stderr=""
    )

    terraform = Terraform(
        base_cwd=Path("/env"),
        executer=mock_executer
    )

    result = await terraform.output()
    assert result.action == TerraformAction.OUTPUT
    assert result.outputs == {
        "bucket_name": {"value":"my-bucket"},
        "region": {"value":"us-east-1"}
    }

    mock_executer.execute.assert_awaited_once_with(
        ["terraform", "output", "-json"],
        env={},
        cwd=Path("/env"),
        mask=True
    )

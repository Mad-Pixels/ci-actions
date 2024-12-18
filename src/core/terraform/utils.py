from typing import Dict, Any

import logging
import json

from core.executer.masker import OutputMasker
from .exceptions import TerraformExecutionError

def parse_terraform_output(json_str: str, logger: logging.Logger) -> Dict[str, Any]:
    """
    Parses the JSON output from 'terraform output -json' into a Python dictionary.

    Args:
        json_str (str): JSON string obtained from the 'terraform output -json' command.
        logger (logging.Logger): Logger for recording parsing errors.

    Returns:
        Dict[str, Any]: Dictionary containing the parsed Terraform outputs.

    Raises:
        TerraformExecutionError: If JSON parsing fails.
    """
    try:
        return json.loads(json_str)
    except json.JSONDecodeError as e:
        logger.error(f"Failed to parse Terraform output JSON: {e}", exc_info=True)
        raise TerraformExecutionError(
            action="output",
            message=f"JSON parsing error: {e}"
        ) from e

def get_default_masker(sensitive: Dict[str, str]) -> OutputMasker:
    """
    Initializes an OutputMasker and registers all sensitive values for masking.

    Args:
        sensitive (Dict[str, str]): Dictionary of sensitive variables to be masked.

    Returns:
        OutputMasker: Instance of OutputMasker with registered sensitive values.
    """
    masker = OutputMasker()
    [masker.sensitive(val) for val in sensitive.values() if val]
    return masker

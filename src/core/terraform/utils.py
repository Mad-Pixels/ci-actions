from typing import Dict, Any

import logging

from core.executer.utils import str_to_dict
from core.executer.masker import OutputMasker

def parse_terraform_output(json_str: str, logger: logging.Logger) -> Dict[str, Any]:
    """Parse 'terraform output -json' into dictionary"""
    return str_to_dict(json_str, logger)

def get_default_masker(sensitive: Dict[str, str]) -> OutputMasker:
    """Initialize masker"""
    masker = OutputMasker()
    for val in sensitive.values():
        if val:
            masker.sensitive(val)
    return masker

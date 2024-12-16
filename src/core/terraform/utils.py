from typing import Dict, Any

from src.core.executer.utils import str_to_dict
from src.core.executer.masker import OutputMasker

def parse_terraform_output(json_str: str) -> Dict[str, Any]:
    """Parse 'terraform output -json' into dictionary"""
    return str_to_dict(json_str)

def get_default_masker(sensitive: Dict[str, str]) -> OutputMasker:
    """Initilaize masker"""
    masker = OutputMasker()
    for val in sensitive.values():
        if val:
            masker.sensitive(val)
    return masker

# TODO impleament default data for masking.

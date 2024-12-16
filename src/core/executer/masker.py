from typing import Set, Pattern, Dict

import logging
import re

MASK_STR='******'

class OutputMasker:
    """Sensitive data masker"""

    def __init__(self):
        self._patterns: Set[Pattern] = set()
        self._sensitive: Set[str] = set()
        self._logger = logging.getLogger(self.__class__.__name__)

    def add_pattern(self, pattern: str) -> None:
        try:
            self._patterns.add(re.compile(pattern))
        except re.error as e:
            self._logger.error(f"Error compiling regexp: {e}")
            raise ValueError(f"Invalid regular expression: {e}")
        
    def sensitive(self, value: str) -> None:
        if value:
            self._sensitive.add(value)

    def mask(self, text: str) -> str:
        if not text:
            return text
        
        for value in self._sensitive:
            text = text.replace(value, MASK_STR)
        for pattern in self._patterns:
            text = pattern.sub(MASK_STR, text)
        return text
    
    def mask_env(self, env: Dict[str, str]) -> Dict[str, str]:
        masked_env = {}
        for k, v in env.items():
            if v in self._sensitive:
                masked_env[k] = MASK_STR
                continue
            masked_env[k] = v
        return masked_env

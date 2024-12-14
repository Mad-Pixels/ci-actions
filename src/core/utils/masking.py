from typing import Set, Pattern
import re

class OutputMasker:
    """Mask sensitive data from output"""

    def __init__(self):
        self._patterns: Set[Pattern] = set()
        self._sensitive_values: Set[str] = set()

    def add_pattern(self, pattern: str):
        """Add mask pattern"""
        self._patterns.add(re.compile(pattern))

    def add_sensitive_value(self, value: str):
        """Add sensitive strings"""
        self._sensitive_values.add(value)

    def mask(self, text: str) -> str:
        """Mask execution"""
        if not text:
            return text
        
        masked = text
        for value in self._sensitive_values:
            if value in masked:
                masked = masked.replace(value, '*' * len(value))
        
        for pattern in self._patterns:
            masked = pattern.sub('*****', masked)
        
        return masked

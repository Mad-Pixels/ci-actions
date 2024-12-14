"""Module for masking sensitive data"""
from typing import Set, Pattern
import re
import logging

logger = logging.getLogger(__name__)

class OutputMasker:
    """Sensitive data masker"""

    def __init__(self):
        self._patterns: Set[Pattern] = set()
        self._sensitive_values: Set[str] = set()
        self._logger = logging.getLogger(self.__class__.__name__)

    def add_pattern(self, pattern: str) -> None:
        """
        Add masking pattern
        
        Args:
            pattern: Regular expression pattern
            
        Raises:
            ValueError: If pattern is invalid
        """
        try:
            compiled = re.compile(pattern)
            self._patterns.add(compiled)
        except re.error as e:
            self._logger.error(f"Error compiling regex: {e}")
            raise ValueError(f"Invalid regular expression: {e}")

    def add_sensitive_value(self, value: str) -> None:
        """
        Add sensitive value to mask
        
        Args:
            value: String to mask
        """
        if not value:
            return
        self._sensitive_values.add(value)

    def mask(self, text: str) -> str:
        """
        Mask data in output:
        - Patterns are replaced with '*****'  
        - Sensitive values are replaced with '*' * len(value)
        
        Args:
            text: Text to mask
            
        Returns:
            Masked text
        """
        if not text:
            return text

        masked = text

        # Mask patterns
        for pattern in self._patterns:
            masked = pattern.sub('*****', masked)

        # Mask sensitive values
        for value in self._sensitive_values:
            masked = masked.replace(value, '*' * len(value))

        return masked
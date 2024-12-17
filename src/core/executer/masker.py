from typing import Set, Pattern, Dict

import logging
import re

MASK_STR = '******'

class OutputMasker:
    """
    Sensitive data masker for output streams and environment variables.

    Features:
    - Masks sensitive values provided directly.
    - Masks values matching predefined regular expressions.
    - Masks environment variables that contain sensitive data.

    Attributes:
        _patterns (Set[Pattern]): Compiled regular expressions for matching sensitive data.
        _sensitive (Set[str]): Set of exact sensitive values to be masked.
        _logger (logging.Logger): Logger for error and debug messages.
    """

    def __init__(self):
        self._patterns: Set[Pattern] = set()
        self._sensitive: Set[str] = set()
        self._logger = logging.getLogger(self.__class__.__name__)

    def add_pattern(self, pattern: str) -> None:
        """
        Add a regular expression pattern to match and mask sensitive data.

        Args:
            pattern: A string representing a valid regular expression.

        Raises:
            ValueError: If the provided regular expression is invalid.
        """
        try:
            self._patterns.add(re.compile(pattern))
        except re.error as e:
            self._logger.error(f"Error compiling regexp: {e}")
            raise ValueError(f"Invalid regular expression: {e}")
        
    def sensitive(self, value: str) -> None:
        """
        Add an exact sensitive value to the list of values to mask.

        Args:
            value: A string representing a sensitive value.
        """
        if value:
            self._sensitive.add(value)

    def mask(self, text: str) -> str:
        """
        Mask sensitive data in the given text.

        Args:
            text: The input string that may contain sensitive data.

        Returns:
            A string with sensitive data replaced by '******'.
        """
        if not text:
            return text
        
        for value in self._sensitive:
            text = text.replace(value, MASK_STR)
        for pattern in self._patterns:
            text = pattern.sub(MASK_STR, text)
        return text
    
    def mask_env(self, env: Dict[str, str]) -> Dict[str, str]:
        """
        Mask sensitive data in environment variables.

        Args:
            env: A dictionary of environment variables.

        Returns:
            A new dictionary with sensitive values replaced by '******'.
        """
        masked_env = {}
        for k, v in env.items():
            if v in self._sensitive:
                masked_env[k] = MASK_STR
                continue
            masked_env[k] = v
        return masked_env

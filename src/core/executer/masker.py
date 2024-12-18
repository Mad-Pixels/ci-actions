from typing import Set, Pattern, Dict

import logging
import re

MASK_STR = '******'

class OutputMasker:
    """
    Sensitive data masker for output streams and environment variables.

    Features:
    - Masks sensitive values provided directly using exact matching
    - Masks values matching predefined regular expressions
    - Masks environment variables that contain sensitive data
    - Support for case-insensitive pattern matching
    - Processes patterns in order: regexes first, then exact matches
    - Handles exact matches by length to avoid partial masking

    Attributes:
        _patterns (Set[Pattern]): Compiled regular expressions for matching sensitive data.
        _sensitive (Set[str]): Set of exact sensitive values to be masked.
        _logger (logging.Logger): Logger for error and debug messages.
    """

    def __init__(self):
        """Initialize an empty masker with no patterns or sensitive values."""
        self._patterns: Set[Pattern] = set()
        self._sensitive: Set[str] = set()
        self._logger = logging.getLogger(self.__class__.__name__)

    def add_pattern(self, pattern: str, *, ignore_case: bool = False) -> None:
        r"""
        Add a regular expression pattern to match and mask sensitive data.

        Args:
            pattern: A string representing a valid regular expression.
            ignore_case: If True, the pattern will match case-insensitively.

        Raises:
            ValueError: If the provided regular expression is invalid.

        Example:
            masker = OutputMasker()
            masker.add_pattern(r'\d{4}-\d{4}-\d{4}-\d{4}')  # Mask credit card numbers
            masker.add_pattern(r'password=\w+', ignore_case=True)  # Mask passwords case-insensitively
        """
        try:
            flags = re.IGNORECASE if ignore_case else 0
            self._patterns.add(re.compile(pattern, flags))
        except re.error as e:
            self._logger.error(f"Error compiling regexp: {e}")
            raise ValueError(f"Invalid regular expression: {e}")
        
    def sensitive(self, value: str) -> None:
        """
        Add an exact sensitive value to the list of values to mask.

        Args:
            value: A string representing a sensitive value.
            
        Example:
            masker = OutputMasker()
            masker.sensitive("my-api-key-123")
        """
        if not value:
            return
        self._sensitive.add(value)

    def mask(self, text: str) -> str:
        r"""
        Mask sensitive data in the given text.

        The masking process happens in two stages:
        1. Apply all regex patterns
        2. Apply exact value matches (sorted by length to avoid partial matches)

        Args:
            text: The input string that may contain sensitive data.

        Returns:
            A string with sensitive data replaced by '******'.

        Example:
            masker = OutputMasker()
            masker.sensitive("api_key_123")
            masker.add_pattern(r'password=\w+')
            
            masked = masker.mask("password=secret&key=api_key_123")
            # Result: "password=******&key=******"
        """
        if not text:
            return text
        
        masked_text = text
        for pattern in self._patterns:
            masked_text = pattern.sub(MASK_STR, masked_text)
        for value in sorted(self._sensitive, key=len, reverse=True):
            masked_text = masked_text.replace(value, MASK_STR)
        return masked_text
    
    def mask_env(self, env: Dict[str, str]) -> Dict[str, str]:
        """
        Mask sensitive data in environment variables.

        Args:
            env: A dictionary of environment variables.

        Returns:
            A new dictionary with sensitive values replaced by '******'.

        Example:
            masker = OutputMasker()
            masker.sensitive("secret_key")
            
            env = {"API_KEY": "secret_key", "DEBUG": "true"}
            masked = masker.mask_env(env)
            # Result: {"API_KEY": "******", "DEBUG": "true"}
        """
        masked_env = {}
        for k, v in env.items():
            masked_env[k] = MASK_STR if v in self._sensitive else v
        return masked_env

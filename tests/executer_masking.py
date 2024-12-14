"""Tests for masking module"""
import pytest
from core.executer.masking import OutputMasker

@pytest.fixture
def masker():
    """Create basic masker instance"""
    return OutputMasker()

@pytest.fixture
def configured_masker():
    """Create masker with predefined patterns and values"""
    masker = OutputMasker()
    masker.add_pattern(r"\d{4}-\d{2}-\d{2}")  # Date pattern
    masker.add_sensitive_value("SECRET_TOKEN")
    return masker

def test_add_pattern_valid(masker):
    """Test adding valid pattern"""
    masker.add_pattern(r"\d+")
    assert len(masker._patterns) == 1

def test_add_pattern_invalid(masker):
    """Test adding invalid pattern"""
    with pytest.raises(ValueError):
        masker.add_pattern(r"[invalid")

def test_add_sensitive_value(masker):
    """Test adding sensitive value"""
    masker.add_sensitive_value("secret")
    assert "secret" in masker._sensitive_values

def test_add_empty_sensitive_value(masker):
    """Test adding empty sensitive value"""
    masker.add_sensitive_value("")
    assert not masker._sensitive_values

@pytest.mark.parametrize("input_text,expected", [
    ("", ""),
    (None, None),
    ("No sensitive data", "No sensitive data"),
    ("SECRET_TOKEN", "*" * 12),  # Length is 12 chars
    ("2024-01-01", "*****"),
    ("SECRET_TOKEN appears on 2024-01-01", 
     "************" + " appears on " + "*****"),
])
def test_mask_text(configured_masker, input_text, expected):
    """Test masking different text inputs"""
    if input_text is None:
        assert configured_masker.mask(input_text) is None
    else:
        assert configured_masker.mask(input_text) == expected

def test_multiple_patterns(masker):
    """Test masking with multiple patterns"""
    masker.add_pattern(r"\d{3}-\d{2}-\d{4}")  # SSN pattern
    masker.add_pattern(r"\b\w+@\w+\.\w+\b")   # Email pattern
    
    text = "SSN: 123-45-6789, Email: test@example.com"
    masked = masker.mask(text)
    
    assert "123-45-6789" not in masked
    assert "test@example.com" not in masked
    assert "SSN: *****, Email: *****" == masked

def test_overlapping_patterns(masker):
    """Test masking with overlapping patterns"""
    masker.add_pattern(r"\w+@\w+\.\w+")  # Email
    masker.add_pattern(r"\w+\.com")      # .com domain
    
    text = "Contact: admin@example.com"
    masked = masker.mask(text)
    assert "admin@example.com" not in masked
    assert "example.com" not in masked
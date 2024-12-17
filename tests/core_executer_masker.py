from src.core.executer.masker import MASK_STR

import pytest


def test_add_pattern_valid(masker):
    masker.add_pattern(r"\d{4}")
    assert len(masker._patterns) == 1


def test_add_pattern_invalid(masker):
    with pytest.raises(ValueError):
        masker.add_pattern(r"(\d{4")


def test_sensitive(masker):
    sensitive_value = "secret"
    masker.sensitive(sensitive_value)
    assert sensitive_value in masker._sensitive


def test_mask_with_pattern(masker):
    masker.add_pattern(r"\d{4}")
    text = "My PIN is 1234"
    masked_text = masker.mask(text)
    assert masked_text == "My PIN is ******"


def test_mask_with_sensitive_value(masker):
    sensitive_value = "password"
    masker.sensitive(sensitive_value)
    text = "My password is very secure."
    masked_text = masker.mask(text)
    assert masked_text == "My ****** is very secure."


def test_mask_with_pattern_and_sensitive(masker):
    masker.add_pattern(r"\d{4}")
    sensitive_value = "secret"
    masker.sensitive(sensitive_value)
    text = "PIN: 1234, secret code: secret."
    masked_text = masker.mask(text)
    assert masked_text == "PIN: ******, ****** code: ******."


def test_mask_empty_text(masker):
    masked_text = masker.mask("")
    assert masked_text == ""


def test_mask_no_patterns_or_sensitive(masker):
    text = "This text has nothing to mask."
    masked_text = masker.mask(text)
    assert masked_text == text


def test_mask_none(masker):
    masked_text = masker.mask(None)
    assert masked_text is None

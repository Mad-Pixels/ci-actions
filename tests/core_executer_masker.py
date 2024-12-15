from core.executer.masker import OutputMasker, MASK_STR

import pytest

@pytest.fixture
def masker():
    """Фикстура для создания экземпляра OutputMasker."""
    return OutputMasker()

def test_add_pattern_valid(masker):
    """Тест на добавление корректного шаблона регулярного выражения."""
    masker.add_pattern(r"\d{4}")
    assert len(masker._patterns) == 1

def test_add_pattern_invalid(masker):
    """Тест на добавление некорректного шаблона регулярного выражения."""
    with pytest.raises(ValueError):
        masker.add_pattern(r"(\d{4")  # Некорректное регулярное выражение

def test_sensitive(masker):
    """Тест на добавление чувствительных данных."""
    sensitive_value = "secret"
    masker.sensitive(sensitive_value)
    assert sensitive_value in masker._sensitive

def test_mask_with_pattern(masker):
    """Тест маскирования текста с использованием регулярных выражений."""
    masker.add_pattern(r"\d{4}")
    text = "My PIN is 1234"
    masked_text = masker.mask(text)
    assert masked_text == "My PIN is ******"

def test_mask_with_sensitive_value(masker):
    """Тест маскирования текста с использованием чувствительных данных."""
    sensitive_value = "password"
    masker.sensitive(sensitive_value)
    text = "My password is very secure."
    masked_text = masker.mask(text)
    assert masked_text == "My ****** is very secure."

def test_mask_with_pattern_and_sensitive(masker):
    """Тест маскирования с использованием шаблонов и чувствительных данных."""
    masker.add_pattern(r"\d{4}")
    sensitive_value = "secret"
    masker.sensitive(sensitive_value)
    text = "PIN: 1234, secret code: secret."
    masked_text = masker.mask(text)
    assert masked_text == "PIN: ******, ****** code: ******."

def test_mask_empty_text(masker):
    """Тест маскирования пустого текста."""
    masked_text = masker.mask("")
    assert masked_text == ""

def test_mask_no_patterns_or_sensitive(masker):
    """Тест маскирования, если не добавлены шаблоны или чувствительные данные."""
    text = "This text has nothing to mask."
    masked_text = masker.mask(text)
    assert masked_text == text

def test_mask_none(masker):
    """Тест маскирования, если входной текст равен None."""
    masked_text = masker.mask(None)
    assert masked_text is None

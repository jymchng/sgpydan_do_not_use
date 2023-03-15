import pytest
from nric_do_not_use import SecretNRIC


@pytest.fixture
def valid_srt_nric():

    _valid_srt_nric = SecretNRIC("S4219468I", '.env.example', 'SECRET_KEY')
    return _valid_srt_nric


def test_no_attributes(valid_srt_nric):

    with pytest.raises(AttributeError):
        valid_srt_nric.prefix

    with pytest.raises(AttributeError):
        valid_srt_nric.suffix

    with pytest.raises(AttributeError):
        valid_srt_nric.digits


def test_not_inheritable():

    with pytest.raises(Exception):
        class UnheritableSecretNRIC(SecretNRIC):
            pass


def test_repr_str_print_nothing(valid_srt_nric):

    assert repr(valid_srt_nric) == '<SECRETNRIC>'
    assert str(valid_srt_nric) == '<SECRETNRIC>'


def test_decrypt(valid_srt_nric):
    
    # cannot test output from `reveal_encrypted()` because its always different
    assert "S4219468I" == valid_srt_nric.decrypt(
        valid_srt_nric.reveal_encrypted(), '.env.example', 'SECRET_KEY')

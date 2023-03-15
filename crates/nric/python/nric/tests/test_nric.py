import pytest
from nric_do_not_use import NRIC


@pytest.fixture
def valid_nric():

    _valid_nric = NRIC("S4219468I")
    return _valid_nric


def test_no_attributes(valid_nric):

    assert valid_nric.prefix == 'S'
    assert valid_nric.suffix == 'I'
    assert valid_nric.digits == '4219468'


def test_not_inheritable():

    with pytest.raises(Exception):
        class UnheritableSecretNRIC(NRIC):
            pass


def test_repr_str(valid_nric):

    assert repr(valid_nric) == '<NRIC::S4219468I>'
    assert str(valid_nric) == 'S4219468I'



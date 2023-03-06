import pytest
from nric_do_not_use import SecretNRIC

@pytest.fixture
def valid_srt_nric():
    
    _valid_srt_nric = SecretNRIC("S4219468I")
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
    
    assert repr(valid_srt_nric) == ''
    assert str(valid_srt_nric) == ''
    

    
        
        


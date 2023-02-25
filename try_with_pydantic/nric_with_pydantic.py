from pydantic import BaseModel
from sgnric_do_not_use import NRIC

class User(BaseModel):
    name: str
    nric: NRIC
    
if __name__ == '__main__': 
    user = User(name='Peter', NRIC='S9962669J')




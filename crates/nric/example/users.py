from pydantic import BaseModel, ValidationError
from nric_do_not_use import NRIC

class User(BaseModel):
    name: str
    nric: NRIC

    class Config:
      arbitrary_types_allowed = True
    
if __name__ == '__main__': 
  
  user = User(name='Peter', nric='S9962669J')
  try:
    user_two = User(name='Peter', nric='B9962669J')
    print(user_two)
  except ValidationError as err:
    print(err)
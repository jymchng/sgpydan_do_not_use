from pydantic import BaseModel, ValidationError
from nric_do_not_use import NRIC, SecretNRIC
from inspect import signature

class User(BaseModel):
    name: str
    nric: NRIC

class SecretUser(BaseModel):
    name: str
    secret_nric: SecretNRIC
    
    class Config:
      allow_arbitrary_types = True
    
if __name__ == '__main__': 
  
  user = User(name='Peter', nric='S9962669J')
  print(user)
  try:
    user_two = User(name='Peter', nric='B9962669J')
    print(user_two)
  except ValidationError as err:
    print(err)
    
  user = SecretUser(name='James', nric='S9962669J')
  print(user)
  try:
    user_two = SecretUser(name='Henry', nric='B9962669J')
    print(user_two)
  except ValidationError as err:
    print(err)
    
  
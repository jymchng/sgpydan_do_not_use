<h1><center>DO NOT USE THIS IN PRODUCTION!</center></h1>

<div align="center">
<h1>SGPYDAN</h1>
<i>pydanic models for the Singaporean</i>
</div>

<p>

## **DISCLAIMER**

This is a student project. This is done just for fun. Please do not use this for anything **serious**!

## What is this about?
The Singaporean NRIC has a [checksum algorithm](https://ivantay2003.medium.com/creation-of-singapore-identity-number-nric-24fc3b446145) to validate the format of the NRIC number.

The checksum algorithm is implemented using Rust, via the `TypeState` and `Builder` pattern.

[`Pyo3`](https://docs.rs/pyo3/latest/pyo3/), a Rust crate, is used to do rust-python bindings and the class `NRIC` within the python module is a Rust struct and its constructor is implemented with Rust.

## Pydantic-compatible

The python class `NRIC` is compatible with a popular third-party python package [`pydantic`](https://docs.pydantic.dev/) for parsing and validation of data models.

Example:

```python
from pydantic import BaseModel, ValidationError
from nric_do_not_use import NRIC

class User(BaseModel):
    name: str
    nric: NRIC

    class Config:
      arbitrary_types_allowed = True
    
if __name__ == '__main__': 
  
  user = User(name='Peter', nric='S9962669J')
  print(user)
  try:
    user_two = User(name='Peter', nric='B9962669J')
    print(user_two)
  except ValidationError as err:
    print(err)
```

Output:
```
...name='Peter' nric=<NRIC::S9962669J>
...1 validation error for User
...nric
...  Prefix cannot be parsed. (type=value_error)
```
This imples that the `user` has been successfully parsed into a `pydantic` model whereas `user_two` failed.

## Why do this?

Mainly, just for fun.

Learnt a lot of Rust and Python through this exercise.
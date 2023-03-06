<h1><div align="center">DO NOT USE THIS FOR ANYTHING SERIOUS!</div></h1>

<div align="center">
<h1>SGPYDANTIC</h1>
<i>Rust-implemented python's pydanic models for Singaporeans</i>
</div>

<p>

## **DISCLAIMER**

This is a student project. This is done just for fun. Please do not use this for anything **serious**!

## What is this about?
The Singaporean NRIC has a [checksum algorithm](https://ivantay2003.medium.com/creation-of-singapore-identity-number-nric-24fc3b446145) to validate the format of the NRIC number.

The checksum algorithm is implemented using Rust, via the `TypeState` and `Builder` pattern.

[`Pyo3`](https://docs.rs/pyo3/latest/pyo3/), a Rust crate, is used to do rust-python bindings and the classes `NRIC` and `SecretNRIC` within the python module are both implemented as Rust structs.


## `SecretNRIC` is slightly more secured than pure-python implementations

### 1. It is Uninheritable

`SecretNRIC` is uninheritable.

### 2. It cannot be found via `inspect.getmembers`, `vars`, `dir` and `gc.get_referrers`

Doesn't mean it is 'secured' because the value of the 'hidden' `NRIC` can still be found by calculating the 'offset' to the memory address. But it does help with preventing leaking of sensitive information since the actual value is not easily accessible.

### 3. `.encrypt()` and `.decrypt()` methods give access to 'secret' value using encryption

### 4. Can its 'secret' value still be found?
The answer is **yes**: [see discussion on pyo3](https://github.com/PyO3/pyo3/discussions/3003#discussioncomment-5201863).

Nonetheless, it does have the chance of making python apps a lot more secured since the 'secret' value requires some additional difficulty to 'retrieve'. It reduces the chances that 'secret' values are leaked 'accidentally'.

## Pydantic-compatible

The python class `NRIC`, implemented in Rust using the `TypeState` and `Builder` patterns, is compatible with a popular third-party python package [`pydantic`](https://docs.pydantic.dev/) for parsing and validation of data models.

Example:

```python
from pydantic import BaseModel, ValidationError
from nric_do_not_use import NRIC

class User(BaseModel):
    name: str
    nric: NRIC


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
... name='Peter' nric=<NRIC::S9962669J>
... 1 validation error for User
... nric
...   Prefix cannot be parsed. (type=value_error)
```
This imples that the `user` has been successfully parsed into a `pydantic` model whereas `user_two` failed.

## Why do this?

Mainly, just for fun.

Learnt a lot of Rust and Python through this exercise.
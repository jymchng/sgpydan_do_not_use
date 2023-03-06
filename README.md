<h1><div align="center">DO NOT USE THIS FOR ANYTHING SERIOUS!</div></h1>

<div align="center">
<h1>🇸🇬 SGPYDANTIC</h1>
<i>Rust-implemented python's pydanic models for Singaporeans</i>
</div>

<p>

# **DISCLAIMER**

This is a student project. This is done just for fun. Please do not use this for anything **serious**!


[![Actions](https://img.shields.io/github/actions/workflow/status/jymchng/sgpydantic_do_not_use/test.yml?branch=main&logo=github&style=flat-square&maxAge=300)](https://github.com/jymchng/sgpydantic_do_not_use/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square&maxAge=2678400)](https://choosealicense.com/licenses/mit/)
[![Source](https://img.shields.io/badge/source-GitHub-303030.svg?maxAge=2678400&style=flat-square)](https://github.com/jymchng/sgpydantic_do_not_use/)
[![Changelog](https://img.shields.io/badge/keep%20a-changelog-8A0707.svg?maxAge=2678400&style=flat-square)](https://github.com/jymchng/sgpydantic_do_not_use/blob/master/CHANGELOG.md)


# 🗺️  What is this about?
The Singaporean NRIC has a [checksum algorithm](https://ivantay2003.medium.com/creation-of-singapore-identity-number-nric-24fc3b446145) to validate the format of the NRIC number.

The checksum algorithm is implemented using Rust, via the `TypeState` and `Builder` pattern.

[`Pyo3`](https://docs.rs/pyo3/latest/pyo3/), a Rust crate, is used to do rust-python bindings and the classes `NRIC` and `SecretNRIC` within the python module are both implemented as Rust structs.

# 💡 `NRIC` and `SecretNRIC` Data Models

## 😸 `SecretNRIC` is slightly more secured than pure-python implementations

Read in detail in [docs](docs/security.md).

### 👩‍❤️‍👨 1. It is uninheritable.

`SecretNRIC` is uninheritable.

```
from nric_do_not_use import SecretNRIC

class NotSoSecretNRIC(SecretNRIC):
    pass

... Traceback (most recent call last):
...   File "<stdin>", line 1, in <module>
... TypeError: type 'builtins.SecretNRIC' is not an acceptable base type
```

###  👩‍💻 2. It cannot be found via `inspect.getmembers`, `vars`, `dir` and `gc.get_referrers`.

Doesn't mean it is 'secured' because the value of the 'hidden' `NRIC` can still be found by calculating the 'offset' to the memory address. But it does help with preventing leaking of sensitive information since the actual value is not easily accessible.

```
import inspect

p = SecretNRIC("S6364259B", '.env.example', 'SECRET_KEY') # 'valid' NRIC

inspect.getmembers(p)

... [('__class__', <class 'builtins.SecretNRIC'>), ('__delattr__', <method-wrapper '__delattr__' of builtins.SecretNRIC object at 0x00000238243EF870>), /.../, ('__subclasshook__', <built-in method __subclasshook__ of type object at 0x0000023823E42620>)]
```

More examples on [docs](docs/security.md).

### 🕵️‍♂️ 3. Initialization of `SecretNRIC` automatically encrypts it. `.decrypt()` method gives access to 'secret' value using encryption (decryption).

`SecretNRIC(nric: str, filepath: str, key_var: str)` is the only pythonic way to initialize the `SecretNRIC`.

Example:
```python
>>> from nric_do_not_use import SecretNRIC
>>> s = SecretNRIC("S1234567D", '.env.example', 'SECRET_KEY')
>>> s
... <SECRETNRIC>
```

A method `.reveal_encryted()` is provided to reveal the encrypted value.

```python
>>> s.reveal_encrypted()
... 'sVykYmi3rFpUNXaoMgUyI6D10yCYWa+OOzQ0NbrMpOw61S/NWw'
```

For `.decrypt()`.

```python
>>> s.decrypt('sVykYmi3rFpUNXaoMgUyI6D10yCYWa+OOzQ0NbrMpOw61S/NWw', '.env.example', 'SECRET_KEY')
... 'S1234567D'
```

### 🔎 4. Can its 'secret' value still be found?
The answer is "**yes**": [see discussion on pyo3](https://github.com/PyO3/pyo3/discussions/3003#discussioncomment-5201863).

Nonetheless, it does have the chance of making python apps a lot more secured since the 'secret' value requires some additional difficulty to 'retrieve'. It greatly reduces the chances that 'secret' values are leaked 'accidentally'.

With encryption, getting the 'value' of the pointer still prevents anyone from accessing the actual-real 'value'. Let's assume `s` is the variable of an instance of `SecretNRIC`, we can use `id(s)` to the pointer to `s`. Even we know the exact offset to where the actual wrapped value of `s` is, dereferencing it would give us an encrypted value anyway.

Therefore, writing sensitive data types in Rust and porting it over to python can make the use of such data types a lot more secured, it is not foolproof, but at least a great enhancement.

<i>P.S. Don't ask how to write 'secure' programs in Rust, I can't even write simple stuff. T T </i>

## 👏🏼 `NRIC` (but not `SecretNRIC`) is pydantic-compatible

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

You can still use `SecretNRIC` in pydantic models, by setting the `allow_arbitrary_types` to be `True` and it will just show as `'<SECRETNRIC>'`.

Example:

```python
from pydantic import BaseModel, ValidationError
from nric_do_not_use import SecretNRIC

class User(BaseModel):
    name: str
    nric: SecretNRIC

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
```

## 🧑🏼‍🚀 Why do this?

Mainly, just for fun and to illustrate the possibility of using pyo3 Rust-python bindings for practical purposes.

Learnt a lot of Rust and Python through this exercise.
# Security of `SecretNRIC` custom python type implemented in Rust

# 1 Uninheritable

```
from nric_do_not_use import SecretNRIC

class NotSoSecretNRIC(SecretNRIC):
    pass

... Traceback (most recent call last):
...   File "<stdin>", line 1, in <module>
... TypeError: type 'builtins.SecretNRIC' is not an acceptable base type
```

# 2 Value cannot be found in a pythonic way

## A. Cannot be found via `inspect.getmembers`

```
import inspect

p = SecretNRIC("S6364259B") # 'valid' NRIC

inspect.getmembers(p)

... [('__class__', <class 'builtins.SecretNRIC'>), ('__delattr__', <method-wrapper '__delattr__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__dir__', 
... <built-in method __dir__ of builtins.SecretNRIC object at 0x00000238243EF870>), ('__doc__', None), ('__eq__', <method-wrapper '__eq__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__format__', <built-in method __format__ of builtins.SecretNRIC object at 0x00000238243EF870>), ('__ge__', <method-wrapper '__ge__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__get_validators__', <built-in method __get_validators__ of type object at 0x0000023823E42620>), ('__getattribute__', <method-wrapper '__getattribute__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__gt__', <method-wrapper '__gt__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__hash__', <method-wrapper '__hash__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__init__', <method-wrapper '__init__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__init_subclass__', <built-in method __init_subclass__ of type object at 0x0000023823E42620>), ('__le__', <method-wrapper '__le__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__lt__', <method-wrapper '__lt__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__module__', 'builtins'), ('__ne__', <method-wrapper '__ne__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__new__', 
... <built-in method __new__ of type object at 0x0000023823E42620>), ('__reduce__', <built-in method __reduce__ of builtins.SecretNRIC object at 0x00000238243EF870>), ('__reduce_ex__', <built-in method __reduce_ex__ of builtins.SecretNRIC object at 0x00000238243EF870>), ('__repr__', <method-wrapper '__repr__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__setattr__', <method-wrapper '__setattr__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__sizeof__', <built-in method __sizeof__ of builtins.SecretNRIC object at 0x00000238243EF870>), ('__str__', <method-wrapper '__str__' of builtins.SecretNRIC object at 0x00000238243EF870>), ('__subclasshook__', <built-in method __subclasshook__ of type object at 0x0000023823E42620>), ('validate', <built-in method validate of type object at 0x0000023823E42620>)]
```

## B. Cannot be found in `dir` and `vars`

```
vars(p)
... Traceback (most recent call last):
...   File "<stdin>", line 1, in <module>
... TypeError: vars() argument must have __dict__ attribute

dir(p)
... ['__class__', '__delattr__', '__dir__', '__doc__', '__eq__', '__format__', '__ge__', '__get_validators__', '__getattribute__', '__gt__', '__hash__', '__init__', '__init_subclass__', '__le__', '__lt__', '__module__', '__ne__', '__new__', '__reduce__', '__reduce_ex__', '__repr__', '__setattr__', '__sizeof__', '__str__', 
... '__subclasshook__', 'validate']
```

## C. Cannot be found in `gc` (garbage collector)
```
import gc
gc.get_referrers(p)

... [{'__name__': '__main__', '__doc__': None, '__package__': None, '__loader__': <class '_frozen_importlib.BuiltinImporter'>, '__spec__': None, '__annotations__': 
... {}, '__builtins__': <module 'builtins' (built-in)>, 'SecretNRIC': <class 'builtins.SecretNRIC'>, 'p': <SECRETNRIC>, 'gc': <module 'gc' (built-in)>}]
```

## D. Accesses to the 'secret' value is gated by encryption

**An instance of `SecretNRIC` can be initialized only if both `filepath` to a '.env.*' file and `SECRET_KEY` denoting the 'key' in the `file_path` file whose value correspond to the 'secret key'**

Example of a `.env` file, in this example, it is named `.env.example`:

```raw
SECRET_KEY=XTbifIW3+ty2y2gf+euLu1Z74Y/4kTWvVNQ899N0Y1g # an example `SECRET_KEY`
```

`SecretNRIC(nric: str, filepath: str, key_var: str)` is the only pythonic way to initialize the `SecretNRIC`.

Example:
```python
>>> from nric_do_not_use import SecretNRIC
>>> s = SecretNRIC("S1234567D", '.env.example', 'SECRET_KEY')
>>> s
... <SECRETNRIC>
```

A method `.reveal_encryted()` is provided to reveal the encrypted value.

Example:
```python
>>> s.reveal_encrypted()
... 'sVykYmi3rFpUNXaoMgUyI6D10yCYWa+OOzQ0NbrMpOw61S/NWw'
```

`.decrypt()` method is provided to access the 'secret' value **if you know the 'secret key'** (by putting it in your `.env` file).

Example:
```python
>>> s.decrypt('sVykYmi3rFpUNXaoMgUyI6D10yCYWa+OOzQ0NbrMpOw61S/NWw', '.env.example', 'SECRET_KEY')
... 'S1234567D'
```

Clearly, we got back the original `NRIC` value.

## E. `__str__` and `__repr__` shows only `'<SECRETNRIC>'`
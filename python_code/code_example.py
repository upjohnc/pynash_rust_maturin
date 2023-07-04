from collections import Counter
from typing import Dict

import pytest
from pynash_maturin import MultiplyTwo, count_words, error_me, sum_as_string


# add two integers and return result as a string type

print("Sum as string")


# python version
def sum_string_python(a: int, b: int) -> str:
    return str(a + b)


print("Python result: ", sum_string_python(1, 2))

# rust version
result = sum_as_string(1, 2)
print("Rust type returned: ", type(result))
print("Rust result: ", result)
print()

#####

# Count the words in a string

print("Count words in a string")


# python version
def counter_python(words: str) -> Dict:
    thing = words.split(" ")
    return dict(Counter(thing))


print("Count words python version: ", counter_python("what the what"))

# rust call
result = count_words("what what the the")
print("Rust type: ", type(result))
print("Rust result: ", result)
print()

#####

# As a class multiply 2 to the class attribute

print("Class for multiply 2 to an integer")


# Python version
class MultiplyTwoPython:
    def __init__(self, base_number: int):
        self.base_number = base_number

    def action(self):
        return 2 * self.base_number


mult_two_python = MultiplyTwoPython(20)
print("Python result: ", mult_two_python.action())

# Rust call
mult_two = MultiplyTwo(10)
print("Rust result: ", mult_two.action())
print()

# rust extras
# retrieve the docs of the rust code
print("Docstring for count_words")
print("`", count_words.__doc__, "`")
print()


print("Example of handling exceptions")
# handling errors in rust
result = error_me(1)
print(result)
print()

try:
    result = error_me(2)
except ValueError as e:
    print("ok failed correctly")
    print("Error message: ", e)
    print("Exception type: ", e.__class__)
print()


# python tests around the rust code
def test_error_me():
    result = error_me(1)
    assert result == 1


def test_error_raise():
    with pytest.raises(Exception):
        result = error_me(2)

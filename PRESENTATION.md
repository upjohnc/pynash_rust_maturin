# Pynash - Rust and Python

## Items to go over

- Why does Rust matter for Python
- Compare Python to Rust
- Go over how to use Pyo3 to set the rust crate to work as a python package
- Go over how to use Maturin to build the rust crate as a python package

## Why care

- faster than python: rust is low-level, compiled language
- trade-off: rust is more verbose than python, takes more time to write
- pydantic v2: ported some of the code to rust and gained a 22x speed increase

## Python & Rust

> to the code...

## Iterative Development

- `maturin develop`: builds the crate and places it in the virtual env
    - allows for iterative development on the rust crate and then quick testing in the python code.

## Production

`maturin build` will build the crate as a whl or `maturin publish` will build the crate and
push to pypi.


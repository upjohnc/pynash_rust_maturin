# Pyo3, Rust, and Maturin

## Reason for Rust

Rust is a compiled and low-level (compared to Python) language.
These features make Rust faster than Python.
There are packages that have been written in rust which are used
in the Python ecosystem.
For example: [ruff](https://pypi.org/project/ruff/)
and [polars](https://pypi.org/project/polars/).

The example code in this repo is to give a starting point and
a basic understanding of the tools available in the rust ecosystem.

## Pyo3 in Example Code

### Functions

Functions need to be taged with macro `#[pyfunction]` and then added to the module
(in the code example it is in the `fn pynash_maturin`).
For the return type, you can use a native rust type or the python equivalent.

Pyo3 does have `Pyresult<>` for the situations when you knowingly will
raise an exception.  The example function `error_me` shows how you can raise a
particular python exception type.

[reference of python to rust types](https://pyo3.rs/v0.19.1/conversions/tables)

### Classes

Classes are little more involved.  To try to show the parallels between rust
and python, rust `struct` would be the attributes of a class and `impl` would
be the methods of the class.

As the example code shows for MultiplyTwo, the macro tag `#[pyclass]` is added to `struct`
and `impl` has `#[pymethods]`.  For the matching `__init__` method, the macro tag `#[new]`
as added to one of the methods (common practice is the `new` method in rust).

## Maturin

Maturin is the command line tool that is used to build the rust crate for use as
the python package.

The key commands:

* `init` : sets the basis of rust project to define how the
  build will create a binary for use in python.
    * creates a `pyproject.toml` file and `Cargo.toml`
        * in the `Cargo.toml` is the package name, crate type of `cdylib`,
          and the `pyo3` as a rust dependency.
* `build` : creates the binary in `./target/***`
* `develop` : used for iterative dev process to build the binary and place
  it in the virtualenv `./site-packages/****`
* `publish` : build the binary and push to pypi as a python package


### Maturin: Dev Process

Maturin has the feature of supporting iterative development.
You can work on your rust code,
create the rust crate with the `develop` command and maturin will
build the crate and then place it in your python virtualenv.
This allows you to easily run your python script with the rust
crate imported.

Steps:
```
> maturin develop
> python <script.py>
```

### Maturin: Production Build

The package can be built and pushed to pypi (`maturin publish`) which then can be `pip install` into your project.

Since the `whl` that is created is a zip file, you could simply create the crate
and use it side-by-side in your python code.  I don't know if this is best practice,
but it is a way of not using pypi if you so choose.
You could build the crate (`maturin build`)
and retrieve it from the `target/wheels/<cargoname>.whl` build location.
You can unzip it and then copy it to the root directory of your python
code.  The `.justfile` has the recipe `production` as an example.

There are notes in the Maturin repo about building for linux environments
and for building for different python versions.
[Maturin Build Docs](https://www.maturin.rs/distribution.html)



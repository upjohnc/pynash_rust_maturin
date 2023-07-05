# Pyo3, Rust, and Maturin

## Reason for Rust

Rust is a compiled and low-level (compared to Python) language.
These features make Rust faster than Python.
There are packages that have been written in rust which are used
in the Python ecosystem.
For example: [ruff](https://pypi.org/project/ruff/)
and [polars](https://pypi.org/project/polars/).

The example code in this repo is to give a starting point or in other words,
a basic understanding of the tools available in the rust ecosystem.

## Pyo3 in Example Code

return type: exception pyresult
pyany using gil

class creation
impl + struct

## Maturin

The key commands

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
but it will let you not have to use pypi if you don't want.
You could build the crate (`maturin build`)
and retrieve it from the `target/wheels/<cargoname>.whl` build location.
You can unzip it and then copy it to the root directory of your python
code.  The `.justfile` has the recipe `production` as an example.

There are notes in the Maturin repo about building for linux environments
and for building for different python versions.
[Maturin Build Docs](https://www.maturin.rs/distribution.html)



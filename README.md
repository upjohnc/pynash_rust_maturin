# Pynash - Maturin for Python Packages

This is the code for the pynash presentation on maturin.
The presentation is to show how a developer can port some of his/her
python code to rust and then call the rust code from the python project.

The rust tools that are used are maturin and pyo3.

## Branch usage

The repo is set up with different branches to show the different stages of porting the python code.

* 1_just_python
    * starting point with all code in python
* 2_python_and_rust
    * side by side of the original python code with its rust equal
* 3_python_importing_package
    * python code importing the built rust code

## Usage

To see the python code work with the package:
* checkout the branch `3_python_importing_package`
    * `git checkout 3_python_importing_package`
* create a virtual env
    * `python -m venv .venv`
    * `source ./.venv/bin/activate`
    * `pip install -r requirements.txt`
* run the python code
    * `python src/***.py`

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


## Dev Process

Show a basic python code and then writing parts of that code in rust
Dev process to iteratively build the rust binary and then build a production binary to transport with your code

## Production Build

The package can be built and pushed to pypi which then can be `pip install` into your project.  If
you do not want to add it to pypi you could build the binary and retrieve it from the `target***` build
location.  Then pull the binary to the root of your python src code.  Since the `.whl` is a zip file,
you can unzip it, rename the extension of the binary, and then copy it to the root directory of your python
code.

## Resources

* [blog post on porting to rust](https://www.jelmer.uk/port-py-to-rust.html)
* [Moshe Zadka blog post on maturin](https://opensource.com/article/23/3/python-loves-rust)
* [youtube demo of maturin](https://www.youtube.com/watch?v=DpUlfWP_gtg)
* [maturin](https://github.com/PyO3/maturin)
* [pyo3](https://docs.rs/pyo3/latest/pyo3/)

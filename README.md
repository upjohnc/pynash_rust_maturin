# Pynash - Maturin for Python Packages

This is the code for the pynash presentation on maturin.
The presentation is to show how a developer can port some of his/her
python code to rust and then call the rust code from the python project.

The rust tools that are used are maturin and pyo3.

## Pyo3, Rust, and Maturin

To help with understanding the code in this repo, there is a page that
explains some of the features used.
<br>**Link: [pyo3 explanation](./PYO3.md)**

## Run Code Locally

To see the python code work with the package:
* create a virtual env
    * `python -m venv .venv`
    * `source ./.venv/bin/activate`
    * `pip install -r requirements.txt`
* python package setup
    * `maturin develop`
* run the python code
    * `python python_code/code_example.py`

## Common Commands

There is a `.justfile` for common commands.  The file's intention
is to share what the common commands used in this repo are as well
as simplfy the running of those commands from the terminal.

[just cli tool](https://github.com/casey/just)

## Resources

* Rust Crates
    * [maturin](https://github.com/PyO3/maturin)
    * [pyo3](https://docs.rs/pyo3/latest/pyo3/)

* Blogs and Video used in creating this repo
    * [blog post on porting to rust](https://www.jelmer.uk/port-py-to-rust.html)
    * [Moshe Zadka blog post on maturin](https://opensource.com/article/23/3/python-loves-rust)
    * [youtube demo of maturin](https://www.youtube.com/watch?v=DpUlfWP_gtg)

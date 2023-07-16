default:
    just --list

# Create the rust package to run in the python code
develop:
    maturin develop

# Run the python code
python:
    python python_code/code_example.py

# Create the rust crate and move it to the root of the python code
production:
    rm -rf pynash*
    rm -rf python_code/pynash*
    maturin build
    unzip ./target/wheels/pynash_maturin-0.1.0-cp310-cp310-macosx_10_7_x86_64.whl "pynash_maturin/pynash_maturin*.so"
    mv pynash_maturin/pynash_ma* ./python_code/
    rm -rf pynash*


# Run the python unit tests
pytest:
    python -m pytest ./python_code/code_example.py -v

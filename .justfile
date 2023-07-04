default:
    just --list

# Create the rust package to run in the python code
develop:
    maturin develop

# Run the python code
python:
    python python_code/code_example.py

# Create the rust binary for production
production:
    maturin build

# Run the python unit tests
pytest:
    python -m pytest ./python_code/code_example.py -v


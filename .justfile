default:
    just --list

create-venv:
     python -m venv .venv

source-venv:
    source .venv/bin/activate

basic-python:
    python python/rust_meetup_pyo3/basic_python.py

rust-python:
    python python/rust_meetup_pyo3/rust_python.py

maturin-develop:
    maturin develop



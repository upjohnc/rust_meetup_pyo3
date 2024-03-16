# Description

Lightning presentation at Rust meetup March 2024.

The design of the repo is to show how a python
project could be piecemeal re-written in Rust.
For instance, a few functions or classes
can be chosen for re-write without effecting the
whole python project.

Under `python/rust_meetup_pyo3` is `basic_python.py`
which has a function and a class that we have chosen
to port to Rust.  Under that directory is `rust_python.py`
which is the `basic_python.py` with the function
and class being imported from the library created
under the `src` directory.

## Usage

For running the `rust_python.py` code:

```bash
python -m venv .venv  # create virtualenv
source .venv/bin/activate  # set the virtualenv
pip install -r requirements.txt  # install packages
maturin develop  # compile rust and install in the virtual env
python python/rust_meetup_pyo3/rust_python.py  # run the python example
```

## Reference

- [Maturin docs](https://www.maturin.rs/)
- [pyo3 docs](https://pyo3.rs/v0.15.1/)

# Pyo3 + Maturin - port Python Code

**Situation:**

- Python project that could benefit from being ported to Rust.
- Don't want to re-write everything in Rust.

**Solution:**

Iteratively move functions and classes to Rust that will then be built as a Python library.

---

# Maturin

- Tool to build the Rust code into a Python package
  - key feature: can build the Rust code and place in the virtual env locally.  Allows for quick iterative testing.

---

# Example

- `basic_python.py` has some code we want to port
- `lib.rs` is the re-write of the function and class
- `rust_python.py` is the change to the python code making use of the rust library

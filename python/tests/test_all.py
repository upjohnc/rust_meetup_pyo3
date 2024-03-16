import pytest
import rust_meetup_pyo3


def test_sum_as_string():
    assert rust_meetup_pyo3.sum_as_string(1, 1) == "2"

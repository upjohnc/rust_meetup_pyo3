import rust_meetup_pyo3


def test_concat_():
    result = rust_meetup_pyo3.concat_("1", "2")
    expected = "1-2"
    assert result == expected


def test_math_add():
    result = rust_meetup_pyo3.Math_(1, 2).add()
    expected = 3
    assert result == expected


def test_math_mult():
    result = rust_meetup_pyo3.Math_(1, 2).mult()
    expected = 2
    assert result == expected

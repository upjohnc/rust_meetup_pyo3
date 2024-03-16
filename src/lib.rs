use pyo3::prelude::*;

/// Concat two strings
#[pyfunction]
fn concat_(a: String, b: String) -> PyResult<String> {
    Ok(format!("{}-{}", a, b))
}

#[pyclass]
struct Math_ {
    left: usize,
    right: usize,
}

#[pymethods]
impl Math_ {
    #[new]
    fn new(left: usize, right: usize) -> Self {
        Self { left, right }
    }

    fn add(&self) -> usize {
        self.left + self.right
    }

    fn mult(&self) -> usize {
        self.left * self.right
    }
}

#[pymodule]
fn rust_meetup_pyo3(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(concat_,m)?)?;
    m.add_class::<Math_>()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        let left = "a".to_string();
        let right = "b".to_string();
        let result = concat_(left.clone(), right.clone()).unwrap();
        assert_eq!(result, format!("{}-{}", left, right));
    }

    #[test]
    fn test_mult() {
        let m= Math_::new(1, 2);
        let result = m.mult();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_add() {
        let m= Math_::new(1, 2);
        let result = m.add();
        assert_eq!(result, 3);
    }
}

use pyo3::{exceptions::PyValueError, prelude::*};
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
}

/// Example of a python add_class
/// The struct plus the impl
#[pyclass]
struct MultiplyTwo {
    base_number: usize,
}

#[pymethods]
impl MultiplyTwo {
    #[new]
    fn new(base_number: usize) -> Self {
        MultiplyTwo { base_number }
    }

    fn action(&self) -> usize {
        2 * self.base_number
    }
}

/// Example of raising a Python exception
#[pyfunction]
fn error_me(a: usize) -> PyResult<usize> {
    if a == 1 {
        Ok(a)
    } else {
        Err(PyValueError::new_err("Not 1"))
    }
}

/// Count the words in a string
#[pyfunction]
fn count_words(words: &str) -> Py<PyAny> {
    let mut result = HashMap::new();
    words.split(' ').for_each(|i| {
        result.entry(i).and_modify(|x| *x += 1).or_insert(1);
    });
    Python::with_gil(|py| result.to_object(py))
}

/// A Python module implemented in Rust.
#[pymodule]
fn pynash_maturin(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    m.add_function(wrap_pyfunction!(error_me, m)?)?;
    m.add_class::<MultiplyTwo>()?;
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_sum_string() {
        let input = vec![((1, 2), 3), ((3, 3), 6)];
        for i in input {
            let result = sum_as_string(i.0 .0, i.0 .1);
            assert_eq!(result, i.1.to_string());
        }
    }
}

use pyo3::prelude::*;

#[pymodule]
mod test_rust_project {
    use pyo3::prelude::*;

    #[pyfunction]
    fn add_numbers(a: i64, b: i64) -> PyResult<i64> {
        Ok(a + b)
    }
}

use pyo3::prelude::*;

use pcl_rustic_core::hello_from_core;

#[pyfunction]
fn hello_from_bind() -> String {
    hello_from_core()
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello_from_bind, m)?)?;
    Ok(())
}

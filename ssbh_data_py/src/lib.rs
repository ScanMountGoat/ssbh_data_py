use pyo3::prelude::*;

#[pymodule]
fn ssbh_data_py(py: Python, module: &PyModule) -> PyResult<()> {
    ssbh_data_py_types::ssbh_data_py(py, module)?;
    Ok(())
}

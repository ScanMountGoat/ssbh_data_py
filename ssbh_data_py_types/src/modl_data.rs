use crate::{MapPy, PyInit, PyRepr, Pyi, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};

create_exception!(ssbh_data_py, ModlDataError, pyo3::exceptions::PyException);

pub fn modl_data(py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    let modl_data = PyModule::new(py, "modl_data")?;
    modl_data.add_class::<ModlData>()?;
    modl_data.add_class::<ModlEntryData>()?;
    modl_data.add_function(wrap_pyfunction!(read_modl, &modl_data)?)?;

    module.add_submodule(&modl_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.modl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::modl_data::ModlData)]
#[pyrepr("ssbh_data_py.modl_data")]
#[pyi(has_methods = true)]
pub struct ModlData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    pub model_name: String,

    #[pyo3(get, set)]
    pub skeleton_file_name: String,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[str]")]
    pub material_file_names: Py<PyList>,

    #[pyo3(get, set)]
    pub animation_file_name: Option<String>,

    #[pyo3(get, set)]
    pub mesh_file_name: String,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[ModlEntryData]")]
    pub entries: Py<PyList>,
}

#[pymethods]
impl ModlData {
    #[new]
    #[pyo3(signature = (major_version = 1, minor_version = 7))]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(ModlData {
            major_version,
            minor_version,
            model_name: "".into(),
            skeleton_file_name: "".into(),
            material_file_names: PyList::empty(py).into(),
            animation_file_name: None,
            mesh_file_name: "".into(),
            entries: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py, false)?
            .write_to_file(path)
            .map_err(PyErr::from)
    }

    fn __repr__(&self) -> String {
        self.py_repr()
    }
}

// TODO: Can we document the actual default value here?
// Add the default to some sort of derive attribute?
impl PyiMethods for ModlData {
    fn pyi_methods() -> String {
        r#"    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 7,
    ) -> None: ...
    
    def save(self, path: str) -> None: ..."#
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.modl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::modl_data::ModlEntryData)]
#[pyrepr("ssbh_data_py.modl_data")]
pub struct ModlEntryData {
    #[pyo3(get, set)]
    pub mesh_object_name: String,

    #[pyo3(get, set)]
    pub mesh_object_subindex: u64,

    #[pyo3(get, set)]
    pub material_label: String,
}

#[pyfunction]
fn read_modl(py: Python, path: &str) -> PyResult<ModlData> {
    ssbh_data::modl_data::ModlData::from_file(path)
        .map_err(|e| ModlDataError::new_err(format!("{}", e)))?
        .map_py(py, false)
}

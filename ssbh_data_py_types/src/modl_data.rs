use pyo3::{create_exception, prelude::*};

create_exception!(ssbh_data_py, ModlDataError, pyo3::exceptions::PyException);

#[pymodule]
pub mod modl_data {
    pub use super::*;

    use crate::{PyInit, PyRepr, Pyi, PyiMethods};
    use map_py::{MapPy, TypedList};

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::modl_data::ModlData)]
    #[pyrepr("ssbh_data_py.modl_data")]
    #[pyi(has_methods = true)]
    pub struct ModlData {
        pub major_version: u16,
        pub minor_version: u16,
        pub model_name: String,
        pub skeleton_file_name: String,
        pub material_file_names: TypedList<String>,
        pub animation_file_name: Option<String>,
        pub mesh_file_name: String,
        pub entries: TypedList<ModlEntryData>,
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
                material_file_names: TypedList::empty(py),
                animation_file_name: None,
                mesh_file_name: "".into(),
                entries: TypedList::empty(py),
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.clone()
                .map_py(py)?
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

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::modl_data::ModlEntryData)]
    #[pyrepr("ssbh_data_py.modl_data")]
    pub struct ModlEntryData {
        pub mesh_object_name: String,
        pub mesh_object_subindex: u64,
        pub material_label: String,
    }

    #[pyfunction]
    fn read_modl(py: Python, path: &str) -> PyResult<ModlData> {
        ssbh_data::modl_data::ModlData::from_file(path)
            .map_err(|e| ModlDataError::new_err(format!("{}", e)))?
            .map_py(py)
    }
}

use pyo3::{create_exception, prelude::*};

create_exception!(ssbh_data_py, MeshExDataError, pyo3::exceptions::PyException);

#[pymodule]
pub mod meshex_data {
    pub use super::*;

    use crate::mesh_data::mesh_data::MeshObjectData;
    use crate::{MapPy, PyInit, PyRepr, Pyi, PyiMethods};
    use pyo3::types::PyList;

    // TODO: Add static methods for constructing types.
    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::meshex_data::MeshExData)]
    #[pyrepr("ssbh_data_py.meshex_data")]
    #[pyi(has_methods = true)]
    pub struct MeshExData {
        #[pyi(python_type = "list[MeshObjectGroupData]")]
        pub mesh_object_groups: Py<PyList>,
    }

    #[pymethods]
    impl MeshExData {
        #[new]
        fn new(py: Python) -> PyResult<Self> {
            Ok(MeshExData {
                mesh_object_groups: PyList::empty(py).into(),
            })
        }

        #[staticmethod]
        fn from_mesh_objects(py: Python, objects: Vec<MeshObjectData>) -> PyResult<Self> {
            ssbh_data::meshex_data::MeshExData::from_mesh_objects(
                &objects
                    .iter()
                    .map(|o| o.map_py(py))
                    .collect::<Result<Vec<_>, _>>()?,
            )
            .map_py(py)
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.map_py(py)?.write_to_file(path).map_err(PyErr::from)
        }

        fn __repr__(&self) -> String {
            self.py_repr()
        }
    }

    // TODO: Can we document the actual default value here?
    // Add the default to some sort of derive attribute?
    impl PyiMethods for MeshExData {
        fn pyi_methods() -> String {
            "    def __init__(self) -> None: ...

    @staticmethod
    def from_mesh_objects(objects: list[MeshObjectData]) -> MeshExData: ...
    
    def save(self, path: str) -> None: ..."
                .to_string()
        }
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::meshex_data::MeshObjectGroupData)]
    #[pyrepr("ssbh_data_py.meshex_data")]
    pub struct MeshObjectGroupData {
        pub bounding_sphere: BoundingSphere,

        pub mesh_object_name: String,

        pub mesh_object_full_name: String,

        #[pyi(python_type = "list[EntryFlags]")]
        pub entry_flags: Py<PyList>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::meshex_data::EntryFlags)]
    #[pyrepr("ssbh_data_py.meshex_data")]
    pub struct EntryFlags {
        pub draw_model: bool,

        pub cast_shadow: bool,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::meshex_data::BoundingSphere)]
    #[pyrepr("ssbh_data_py.meshex_data")]
    pub struct BoundingSphere {
        #[pyi(python_type = "list[float]")]
        pub center: Py<PyList>,

        pub radius: f32,
    }

    #[pyfunction]
    fn read_meshex(py: Python, path: &str) -> PyResult<MeshExData> {
        ssbh_data::meshex_data::MeshExData::from_file(path)
            .map_err(|e| MeshExDataError::new_err(format!("{}", e)))?
            .map_py(py)
    }
}

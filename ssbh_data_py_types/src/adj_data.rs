use pyo3::create_exception;
use pyo3::prelude::*;

create_exception!(ssbh_data_py, AdjDataError, pyo3::exceptions::PyException);

#[pymodule]
pub mod adj_data {
    pub use super::*;

    use crate::{mesh_data::map_into_vector_data, PyRepr, Pyi, PyiMethods};
    use map_py::{MapPy, TypedList};
    use numpy::PyArray1;

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::adj_data::AdjData)]
    #[pyrepr("ssbh_data_py.adj_data")]
    #[pyi(has_methods = true)]
    pub struct AdjData {
        pub entries: TypedList<AdjEntryData>,
    }

    #[pymethods]
    impl AdjData {
        #[new]
        fn new(py: Python) -> PyResult<Self> {
            Ok(AdjData {
                entries: TypedList::empty(py),
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.clone()
                .map_py(py)?
                .write_to_file(path)
                .map_err(|e| AdjDataError::new_err(format!("{e}")))
        }

        fn __repr__(&self) -> String {
            self.py_repr()
        }
    }

    impl PyiMethods for AdjData {
        fn pyi_methods() -> String {
            "    def __init__(self,) -> None: ...
        
        def save(self, path: str) -> None: ..."
                .to_string()
        }
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::adj_data::AdjEntryData)]
    #[pyrepr("ssbh_data_py.adj_data")]
    #[pyi(has_methods = true)]
    pub struct AdjEntryData {
        pub mesh_object_index: usize,
        pub vertex_adjacency: Py<PyArray1<i16>>,
    }

    #[pymethods]
    impl AdjEntryData {
        #[new]
        fn new(py: Python, mesh_object_index: usize) -> PyResult<Self> {
            Ok(Self {
                mesh_object_index,
                vertex_adjacency: PyArray1::from_slice(py, &[]).into(),
            })
        }

        #[staticmethod]
        fn from_mesh_object(
            py: Python,
            mesh_object_index: usize,
            mesh_object: &crate::mesh_data::mesh_data::MeshObjectData,
        ) -> PyResult<Self> {
            let vertex_indices: Vec<u32> = mesh_object.vertex_indices.extract(py)?;
            let positions: Vec<crate::mesh_data::mesh_data::AttributeData> =
                mesh_object.positions.list.extract(py)?;
            // TODO: Avoid unwrap?
            let vertex_positions = map_into_vector_data(positions[0].data.clone(), py)?;
            let entry = ssbh_data::adj_data::AdjEntryData::from_vector_data(
                mesh_object_index,
                &vertex_positions,
                &vertex_indices,
            );
            entry.map_py(py)
        }

        fn __repr__(&self) -> String {
            self.py_repr()
        }
    }

    impl PyiMethods for AdjEntryData {
        fn pyi_methods() -> String {
            "    def __init__(
            self,
            mesh_object_index: int
        ) -> None: ...
        
        @staticmethod
        def from_mesh_object(mesh_object_index: int,
                             mesh_object: MeshObjectData) -> AdjEntryData: ..."
                .to_string()
        }
    }

    #[pyfunction]
    fn read_adj(py: Python, path: &str) -> PyResult<AdjData> {
        ssbh_data::adj_data::AdjData::from_file(path)
            .map_err(|e| AdjDataError::new_err(format!("{e}")))?
            .map_py(py)
    }
}

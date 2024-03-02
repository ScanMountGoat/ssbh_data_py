use crate::{mesh_data, MapPy, PyRepr, Pyi, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};

create_exception!(ssbh_data_py, AdjDataError, pyo3::exceptions::PyException);

pub fn adj_data(py: Python, module: &PyModule) -> PyResult<()> {
    let adj_data = PyModule::new(py, "adj_data")?;
    adj_data.add_class::<AdjData>()?;
    adj_data.add_class::<AdjEntryData>()?;
    adj_data.add_function(wrap_pyfunction!(read_adj, adj_data)?)?;

    module.add_submodule(adj_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.adj_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::adj_data::AdjData)]
#[pyrepr("ssbh_data_py.adj_data")]
#[pyi(has_methods = true)]
pub struct AdjData {
    #[pyo3(get, set)]
    #[pyi(python_type = "list[AdjEntryData]")]
    pub entries: Py<PyList>,
}

#[pymethods]
impl AdjData {
    #[new]
    fn new(py: Python) -> PyResult<Self> {
        Ok(AdjData {
            entries: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py, false)?
            .write_to_file(path)
            .map_err(|e| AdjDataError::new_err(format!("{}", e)))
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

#[pyclass(module = "ssbh_data_py.adj_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::adj_data::AdjEntryData)]
#[pyrepr("ssbh_data_py.adj_data")]
#[pyi(has_methods = true)]
pub struct AdjEntryData {
    #[pyo3(get, set)]
    pub mesh_object_index: usize,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[int]")]
    pub vertex_adjacency: PyObject,
}

#[pymethods]
impl AdjEntryData {
    #[new]
    fn new(py: Python, mesh_object_index: usize) -> PyResult<Self> {
        Ok(Self {
            mesh_object_index,
            vertex_adjacency: PyList::empty(py).into(),
        })
    }

    #[staticmethod]
    fn from_mesh_object(
        py: Python,
        mesh_object_index: usize,
        mesh_object: &mesh_data::MeshObjectData,
    ) -> PyResult<Self> {
        let vertex_indices: Vec<u32> = mesh_object.vertex_indices.extract(py)?;
        let positions: Vec<mesh_data::AttributeData> = mesh_object.positions.extract(py)?;
        // TODO: Avoid unwrap?
        let vertex_positions = positions.first().unwrap().data.map_py(py, false)?;
        let entry = ssbh_data::adj_data::AdjEntryData::from_vector_data(
            mesh_object_index,
            &vertex_positions,
            &vertex_indices,
        );
        entry.map_py(py, false)
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
        .map_err(|e| AdjDataError::new_err(format!("{}", e)))?
        .map_py(py, false)
}

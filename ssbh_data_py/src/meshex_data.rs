use crate::mesh_data::MeshObjectData;
use crate::{MapPy, PyRepr, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data::SsbhData;
use ssbh_data_py_derive::{MapPy, PyInit, PyRepr, Pyi};

create_exception!(ssbh_data_py, MeshExDataError, pyo3::exceptions::PyException);

pub fn meshex_data(py: Python, module: &PyModule) -> PyResult<()> {
    let meshex_data = PyModule::new(py, "meshex_data")?;
    meshex_data.add_class::<MeshExData>()?;
    meshex_data.add_class::<MeshObjectGroupData>()?;
    meshex_data.add_class::<BoundingSphere>()?;
    meshex_data.add_function(wrap_pyfunction!(read_meshex, meshex_data)?)?;

    module.add_submodule(meshex_data)?;
    Ok(())
}

// TODO: Add static methods for constructing types.
#[pyclass(module = "ssbh_data_py.meshex_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::meshex_data::MeshExData)]
#[pyrepr("ssbh_data_py.meshex_data")]
#[pyi(has_methods = true)]
pub struct MeshExData {
    #[pyo3(get, set)]
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
                .map(|o| o.map_py(py, false))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .map_py(py, false)
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
impl PyiMethods for MeshExData {
    fn pyi_methods() -> String {
        "    def __init__(self) -> None: ...

    @staticmethod
    def from_mesh_objects(objects: list[MeshObjectData]) -> MeshExData: ...
    
    def save(self, path: str) -> None: ..."
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.meshex_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::meshex_data::MeshObjectGroupData)]
#[pyrepr("ssbh_data_py.meshex_data")]
pub struct MeshObjectGroupData {
    #[pyo3(get, set)]
    pub bounding_sphere: BoundingSphere,

    #[pyo3(get, set)]
    pub mesh_object_name: String,

    #[pyo3(get, set)]
    pub mesh_object_full_name: String,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[MeshObjectGroupData]")]
    pub entry_flags: Py<PyList>,
}

#[pyclass(module = "ssbh_data_py.meshex_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::meshex_data::EntryFlags)]
#[pyrepr("ssbh_data_py.meshex_data")]
pub struct EntryFlags {
    #[pyo3(get, set)]
    pub draw_model: bool,

    #[pyo3(get, set)]
    pub cast_shadow: bool,
}

#[pyclass(module = "ssbh_data_py.meshex_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::meshex_data::BoundingSphere)]
#[pyrepr("ssbh_data_py.meshex_data")]
pub struct BoundingSphere {
    #[pyo3(get, set)]
    #[pyi(python_type = "list[float]")]
    pub center: Py<PyList>,

    #[pyo3(get, set)]
    pub radius: f32,
}

#[pyfunction]
fn read_meshex(py: Python, path: &str) -> PyResult<MeshExData> {
    ssbh_data::meshex_data::MeshExData::from_file(path)
        .map_err(|e| MeshExDataError::new_err(format!("{}", e)))?
        .map_py(py, false)
}

#[cfg(test)]
mod tests {
    use crate::run_python_code;
    use indoc::indoc;

    #[test]
    fn read_meshex() {
        // Test exceptions.
        run_python_code(indoc! {r#"
            try:
                ssbh_data_py.meshex_data.read_meshex("invalid")
            except ssbh_data_py.MeshExDataError as e:
                assert True
        "#})
        .unwrap();
    }

    #[test]
    fn create_meshex() {
        run_python_code(indoc! {r#"
            m = ssbh_data_py.meshex_data.MeshExData()
            assert m.mesh_object_groups == []
        "#})
        .unwrap();
    }

    #[test]
    fn create_mesh_object_group() {
        run_python_code(indoc! {r#"
            sphere = ssbh_data_py.meshex_data.BoundingSphere([1, 2, 3], 4)
            m = ssbh_data_py.meshex_data.MeshObjectGroupData(sphere, "a", "a_VIS", [])
            assert m.bounding_sphere.center == [1.0, 2.0, 3.0]
            assert m.bounding_sphere.radius == 4.0
            assert m.mesh_object_name == "a"
            assert m.mesh_object_full_name == "a_VIS"
            assert m.entry_flags == []
        "#})
        .unwrap();
    }

    #[test]
    fn create_meshex_from_objects() {
        run_python_code(indoc! {r#"
            ssbh_data_py.meshex_data.MeshExData.from_mesh_objects([])
            ssbh_data_py.meshex_data.MeshExData.from_mesh_objects([
                ssbh_data_py.mesh_data.MeshObjectData('a', 0),
                ssbh_data_py.mesh_data.MeshObjectData('a', 1)
            ])
        "#})
        .unwrap();
    }
}

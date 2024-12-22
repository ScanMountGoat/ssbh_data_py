use crate::create_py_list_from_slice;
use crate::{MapPy, PyInit, PyRepr, Pyi, PyiMethods};
use num_traits::AsPrimitive;
use numpy::PyArrayMethods;
use numpy::{ndarray::Dim, IntoPyArray, PyArray, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data::mesh_data::VectorData as VectorDataRs;

create_exception!(ssbh_data_py, MeshDataError, pyo3::exceptions::PyException);

pub fn mesh_data(py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    let mesh_data = PyModule::new(py, "mesh_data")?;
    mesh_data.add_class::<MeshData>()?;
    mesh_data.add_class::<MeshObjectData>()?;
    mesh_data.add_class::<AttributeData>()?;
    mesh_data.add_class::<BoneInfluence>()?;
    mesh_data.add_class::<VertexWeight>()?;

    mesh_data.add_function(wrap_pyfunction!(read_mesh, &mesh_data)?)?;
    mesh_data.add_function(wrap_pyfunction!(transform_points, &mesh_data)?)?;
    mesh_data.add_function(wrap_pyfunction!(transform_vectors, &mesh_data)?)?;
    mesh_data.add_function(wrap_pyfunction!(calculate_smooth_normals, &mesh_data)?)?;
    mesh_data.add_function(wrap_pyfunction!(calculate_tangents_vec4, &mesh_data)?)?;

    module.add_submodule(&mesh_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.mesh_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::mesh_data::MeshData)]
#[pyrepr("ssbh_data_py.mesh_data")]
#[pyi(has_methods = true)]
pub struct MeshData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[MeshObjectData]")]
    pub objects: Py<PyList>,
}

#[pymethods]
impl MeshData {
    #[new]
    #[pyo3(signature = (major_version = 1, minor_version = 10))]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(MeshData {
            major_version,
            minor_version,
            objects: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py)?
            .write_to_file(path)
            .map_err(|e| MeshDataError::new_err(format!("{}", e)))
    }

    fn __repr__(&self) -> String {
        self.py_repr()
    }
}

impl PyiMethods for MeshData {
    fn pyi_methods() -> String {
        r#"    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 10,
    ) -> None: ...

    def save(self, path: str) -> None: ..."#
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.mesh_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::mesh_data::MeshObjectData)]
#[pyrepr("ssbh_data_py.mesh_data")]
pub struct MeshObjectData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub subindex: u64,

    #[pyo3(get, set)]
    #[pyinit(default = "String::new()")]
    #[pyi(default = "''")]
    pub parent_bone_name: String,

    #[pyo3(get, set)]
    #[pyinit(default = "false")]
    #[pyi(default = "False")]
    pub disable_depth_test: bool,

    #[pyo3(get, set)]
    #[pyinit(default = "false")]
    #[pyi(default = "False")]
    pub disable_depth_write: bool,

    #[pyo3(get, set)]
    #[pyinit(default = "0")]
    #[pyi(default = "0")]
    pub sort_bias: i32,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "numpy.array([])", python_type = "numpy.ndarray")]
    pub vertex_indices: PyObject,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[AttributeData]")]
    pub positions: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[AttributeData]")]
    pub normals: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[AttributeData]")]
    pub binormals: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[AttributeData]")]
    pub tangents: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[AttributeData]")]
    pub texture_coordinates: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[AttributeData]")]
    pub color_sets: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[BoneInfluence]")]
    pub bone_influences: Py<PyList>,
}

#[pyclass(module = "ssbh_data_py.mesh_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::mesh_data::BoneInfluence)]
#[pyrepr("ssbh_data_py.mesh_data")]
pub struct BoneInfluence {
    #[pyo3(get, set)]
    pub bone_name: String,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[VertexWeight]")]
    pub vertex_weights: Py<PyList>,
}

#[pyclass(module = "ssbh_data_py.mesh_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::mesh_data::VertexWeight)]
#[pyrepr("ssbh_data_py.mesh_data")]
pub struct VertexWeight {
    #[pyo3(get, set)]
    pub vertex_index: u32,

    #[pyo3(get, set)]
    pub vertex_weight: f32,
}

#[pyclass(module = "ssbh_data_py.mesh_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::mesh_data::AttributeData)]
#[pyrepr("ssbh_data_py.mesh_data")]
pub struct AttributeData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "numpy.array([])", python_type = "numpy.ndarray")]
    pub data: PyObject,
}

impl MapPy<PyObject> for VectorDataRs {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        match self {
            VectorDataRs::Vector2(v) => vectors_pyarray(py, v),
            VectorDataRs::Vector3(v) => vectors_pyarray(py, v),
            VectorDataRs::Vector4(v) => vectors_pyarray(py, v),
        }
    }
}

fn vectors_pyarray<const N: usize>(py: Python, values: &[[f32; N]]) -> PyResult<PyObject> {
    // This flatten will be optimized in Release mode.
    // This avoids needing unsafe code.
    // TODO: Can we avoid flattening and then reshaping?
    // TODO: Handle errors?
    let count = values.len();
    Ok(values
        .iter()
        .flat_map(|v| v)
        .copied()
        .collect::<Vec<f32>>()
        .into_pyarray(py)
        .reshape((count, N))
        .unwrap()
        .into_any()
        .into())
}

impl MapPy<VectorDataRs> for PyObject {
    fn map_py(&self, py: Python) -> PyResult<VectorDataRs> {
        // We don't know the type from Python at this point.
        // Try all the supported types and fail if all conversions fail.
        // TODO: Is there an easy way to convert f64 to f32 for the entire array?
        self.extract::<Vec<[f32; 2]>>(py)
            .map(VectorDataRs::Vector2)
            .or_else(|_| self.extract::<Vec<[f32; 3]>>(py).map(VectorDataRs::Vector3))
            .or_else(|_| self.extract::<Vec<[f32; 4]>>(py).map(VectorDataRs::Vector4))
        // .or_else(|_| self.extract::<&PyArray2<f32>>(py).and_then(vector_data))
        // .or_else(|_| self.extract::<&PyArray2<f64>>(py).and_then(vector_data))
        // .or_else(|_| self.extract::<&PyArray2<i8>>(py).and_then(vector_data))
        // .or_else(|_| self.extract::<&PyArray2<i16>>(py).and_then(vector_data))
        // .or_else(|_| self.extract::<&PyArray2<i32>>(py).and_then(vector_data))
    }
}

#[pyfunction]
fn read_mesh(py: Python, path: &str) -> PyResult<MeshData> {
    ssbh_data::mesh_data::MeshData::from_file(path)
        .map_err(|e| MeshDataError::new_err(format!("{}", e)))?
        .map_py(py)
}

#[pyfunction]
fn transform_points(py: Python, points: PyObject, transform: PyObject) -> PyResult<PyObject> {
    let points = points.map_py(py)?;
    let transform = transform.map_py(py)?;
    let transformed_points = ssbh_data::mesh_data::transform_points(&points, &transform);
    transformed_points.map_py(py)
}

#[pyfunction]
fn transform_vectors(py: Python, points: PyObject, transform: PyObject) -> PyResult<PyObject> {
    let points = points.map_py(py)?;
    let transform = transform.map_py(py)?;
    let transformed_points = ssbh_data::mesh_data::transform_vectors(&points, &transform);
    transformed_points.map_py(py)
}

#[pyfunction]
fn calculate_smooth_normals(
    py: Python,
    positions: PyObject,
    vertex_indices: PyObject,
) -> PyResult<Py<PyList>> {
    let positions = positions.map_py(py)?;
    let vertex_indices = vertex_indices.extract::<Vec<u32>>(py)?;
    let normals = ssbh_data::mesh_data::calculate_smooth_normals(&positions, &vertex_indices);
    create_py_list_from_slice(py, &normals)
}

#[pyfunction]
fn calculate_tangents_vec4(
    py: Python,
    positions: PyObject,
    normals: PyObject,
    uvs: PyObject,
    vertex_indices: PyObject,
) -> PyResult<Py<PyList>> {
    let positions = positions.map_py(py)?;
    let normals = normals.map_py(py)?;
    let uvs = uvs.map_py(py)?;

    let vertex_indices = vertex_indices.extract::<Vec<u32>>(py)?;
    let tangents =
        ssbh_data::mesh_data::calculate_tangents_vec4(&positions, &normals, &uvs, &vertex_indices)
            .map_err(|e| MeshDataError::new_err(format!("{}", e)))?;

    create_py_list_from_slice(py, &tangents)
}

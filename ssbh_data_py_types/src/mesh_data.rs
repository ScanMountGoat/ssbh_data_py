use map_py::MapPy;
use numpy::{IntoPyArray, PyArray2, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::{create_exception, prelude::*};
use ssbh_data::mesh_data::VectorData as VectorDataRs;

create_exception!(ssbh_data_py, MeshDataError, pyo3::exceptions::PyException);

#[pymodule]
pub mod mesh_data {
    pub use super::*;

    use crate::{PyInit, PyRepr, Pyi, PyiMethods};
    use map_py::{MapPy, TypedList};
    use numpy::{PyArray1, PyArray2};

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::mesh_data::MeshData)]
    #[pyrepr("ssbh_data_py.mesh_data")]
    #[pyi(has_methods = true)]
    pub struct MeshData {
        pub major_version: u16,
        pub minor_version: u16,
        pub objects: TypedList<MeshObjectData>,
    }

    #[pymethods]
    impl MeshData {
        #[new]
        #[pyo3(signature = (major_version = 1, minor_version = 10))]
        fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
            Ok(MeshData {
                major_version,
                minor_version,
                objects: TypedList::empty(py),
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.clone()
                .map_py(py)?
                .write_to_file(path)
                .map_err(|e| MeshDataError::new_err(format!("{e}")))
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

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::mesh_data::MeshObjectData)]
    #[pyrepr("ssbh_data_py.mesh_data")]
    pub struct MeshObjectData {
        pub name: String,

        pub subindex: u64,

        #[pyinit(default = "String::new()")]
        #[pyi(default = "''")]
        pub parent_bone_name: String,

        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub disable_depth_test: bool,

        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub disable_depth_write: bool,

        #[pyinit(default = "0")]
        #[pyi(default = "0")]
        pub sort_bias: i32,

        #[pyinit(default = "numpy::PyArray1::zeros(py, 0, false).into()")]
        #[pyi(default = "numpy.array([])")]
        pub vertex_indices: Py<PyArray1<u32>>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub positions: TypedList<AttributeData>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub normals: TypedList<AttributeData>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub binormals: TypedList<AttributeData>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub tangents: TypedList<AttributeData>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub texture_coordinates: TypedList<AttributeData>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub color_sets: TypedList<AttributeData>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub bone_influences: TypedList<BoneInfluence>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::mesh_data::BoneInfluence)]
    #[pyrepr("ssbh_data_py.mesh_data")]
    pub struct BoneInfluence {
        pub bone_name: String,
        pub vertex_weights: TypedList<VertexWeight>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::mesh_data::VertexWeight)]
    #[pyrepr("ssbh_data_py.mesh_data")]
    pub struct VertexWeight {
        pub vertex_index: u32,

        pub vertex_weight: f32,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::mesh_data::AttributeData)]
    #[pyrepr("ssbh_data_py.mesh_data")]
    pub struct AttributeData {
        pub name: String,

        #[pyinit(default = "numpy::PyArray2::zeros(py, [0, 0], false).into()")]
        #[pyi(default = "numpy.array([])")]
        #[map(from(map_from_vector_data), into(map_into_vector_data))]
        pub data: Py<PyArray2<f32>>,
    }

    #[pyfunction]
    fn read_mesh(py: Python, path: &str) -> PyResult<MeshData> {
        ssbh_data::mesh_data::MeshData::from_file(path)
            .map_err(|e| MeshDataError::new_err(format!("{e}")))?
            .map_py(py)
    }

    #[pyfunction]
    fn transform_points(
        py: Python,
        points: Py<PyArray2<f32>>,
        transform: Py<PyArray2<f32>>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let points = map_into_vector_data(points, py)?;
        let transform = transform.map_py(py)?;
        let transformed_points = ssbh_data::mesh_data::transform_points(&points, &transform);
        map_from_vector_data(transformed_points, py)
    }

    #[pyfunction]
    fn transform_vectors(
        py: Python,
        points: Py<PyArray2<f32>>,
        transform: Py<PyArray2<f32>>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let points = map_into_vector_data(points, py)?;
        let transform = transform.map_py(py)?;
        let transformed_points = ssbh_data::mesh_data::transform_vectors(&points, &transform);
        map_from_vector_data(transformed_points, py)
    }

    #[pyfunction]
    fn calculate_smooth_normals(
        py: Python,
        positions: Py<PyArray2<f32>>,
        vertex_indices: Py<PyArray1<u32>>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let positions = map_into_vector_data(positions, py)?;
        let vertex_indices = vertex_indices.extract::<Vec<u32>>(py)?;
        let normals = ssbh_data::mesh_data::calculate_smooth_normals(&positions, &vertex_indices);
        normals.map_py(py)
    }

    #[pyfunction]
    fn calculate_tangents_vec4(
        py: Python,
        positions: Py<PyArray2<f32>>,
        normals: Py<PyArray2<f32>>,
        uvs: Py<PyArray2<f32>>,
        vertex_indices: Py<PyArray1<u32>>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let positions = map_into_vector_data(positions, py)?;
        let normals = map_into_vector_data(normals, py)?;
        let uvs = map_into_vector_data(uvs, py)?;

        let vertex_indices = vertex_indices.extract::<Vec<u32>>(py)?;
        let tangents = ssbh_data::mesh_data::calculate_tangents_vec4(
            &positions,
            &normals,
            &uvs,
            &vertex_indices,
        )
        .map_err(|e| MeshDataError::new_err(format!("{e}")))?;
        tangents.map_py(py)
    }
}

pub fn map_from_vector_data(value: VectorDataRs, py: Python) -> PyResult<Py<PyArray2<f32>>> {
    match &value {
        VectorDataRs::Vector2(v) => vectors_pyarray(py, v),
        VectorDataRs::Vector3(v) => vectors_pyarray(py, v),
        VectorDataRs::Vector4(v) => vectors_pyarray(py, v),
    }
}

pub fn map_into_vector_data(value: Py<PyArray2<f32>>, py: Python) -> PyResult<VectorDataRs> {
    let array = value.as_any().downcast_bound::<PyArray2<f32>>(py)?;
    match array.readonly().shape()[1] {
        2 => value.map_py(py).map(VectorDataRs::Vector2),
        3 => value.map_py(py).map(VectorDataRs::Vector3),
        4 => value.map_py(py).map(VectorDataRs::Vector4),
        dim => Err(MeshDataError::new_err(format!(
            "Unsupported vector dimensions {dim}"
        ))),
    }
}

fn vectors_pyarray<const N: usize>(py: Python, values: &[[f32; N]]) -> PyResult<Py<PyArray2<f32>>> {
    // This flatten will be optimized in Release mode.
    // This avoids needing unsafe code.
    // TODO: Can we avoid flattening and then reshaping?
    // TODO: Handle errors?
    let count = values.len();
    Ok(values
        .iter()
        .flatten()
        .copied()
        .collect::<Vec<f32>>()
        .into_pyarray(py)
        .reshape((count, N))
        .unwrap()
        .into())
}

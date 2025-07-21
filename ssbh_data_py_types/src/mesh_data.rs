use pyo3::{create_exception, prelude::*};

create_exception!(ssbh_data_py, MeshDataError, pyo3::exceptions::PyException);

#[pymodule]
pub mod mesh_data {
    pub use super::*;

    use crate::{MapPy, PyInit, PyRepr, Pyi, PyiMethods};
    use numpy::{IntoPyArray, PyArray2, PyUntypedArrayMethods};
    use numpy::{PyArray1, PyArrayMethods};
    use pyo3::types::PyList;
    use ssbh_data::mesh_data::VectorData as VectorDataRs;

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::mesh_data::MeshData)]
    #[pyrepr("ssbh_data_py.mesh_data")]
    #[pyi(has_methods = true)]
    pub struct MeshData {
        pub major_version: u16,

        pub minor_version: u16,

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
        #[pyi(default = "numpy.array([])", python_type = "numpy.ndarray")]
        pub vertex_indices: Py<PyArray1<u32>>,

        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[AttributeData]")]
        pub positions: Py<PyList>,

        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[AttributeData]")]
        pub normals: Py<PyList>,

        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[AttributeData]")]
        pub binormals: Py<PyList>,

        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[AttributeData]")]
        pub tangents: Py<PyList>,

        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[AttributeData]")]
        pub texture_coordinates: Py<PyList>,

        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[AttributeData]")]
        pub color_sets: Py<PyList>,

        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[BoneInfluence]")]
        pub bone_influences: Py<PyList>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::mesh_data::BoneInfluence)]
    #[pyrepr("ssbh_data_py.mesh_data")]
    pub struct BoneInfluence {
        pub bone_name: String,

        #[pyi(python_type = "list[VertexWeight]")]
        pub vertex_weights: Py<PyList>,
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
        #[pyi(default = "numpy.array([])", python_type = "numpy.ndarray")]
        pub data: Py<PyArray2<f32>>,
    }

    fn vectors_pyarray<const N: usize>(
        py: Python,
        values: &[[f32; N]],
    ) -> PyResult<Py<PyArray2<f32>>> {
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

    impl MapPy<Py<PyArray2<f32>>> for VectorDataRs {
        fn map_py(&self, py: Python) -> PyResult<Py<PyArray2<f32>>> {
            match self {
                VectorDataRs::Vector2(v) => vectors_pyarray(py, v),
                VectorDataRs::Vector3(v) => vectors_pyarray(py, v),
                VectorDataRs::Vector4(v) => vectors_pyarray(py, v),
            }
        }
    }

    impl MapPy<VectorDataRs> for Py<PyArray2<f32>> {
        fn map_py(&self, py: Python) -> PyResult<VectorDataRs> {
            let array = self.as_any().downcast_bound::<PyArray2<f32>>(py)?;
            match array.readonly().shape()[1] {
                2 => self.map_py(py).map(VectorDataRs::Vector2),
                3 => self.map_py(py).map(VectorDataRs::Vector3),
                4 => self.map_py(py).map(VectorDataRs::Vector4),
                dim => Err(MeshDataError::new_err(format!(
                    "Unsupported vector dimensions {dim}"
                ))),
            }
        }
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
        let points = points.map_py(py)?;
        let transform = transform.map_py(py)?;
        let transformed_points = ssbh_data::mesh_data::transform_points(&points, &transform);
        transformed_points.map_py(py)
    }

    #[pyfunction]
    fn transform_vectors(
        py: Python,
        points: Py<PyArray2<f32>>,
        transform: Py<PyArray2<f32>>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let points = points.map_py(py)?;
        let transform = transform.map_py(py)?;
        let transformed_points = ssbh_data::mesh_data::transform_vectors(&points, &transform);
        transformed_points.map_py(py)
    }

    #[pyfunction]
    fn calculate_smooth_normals(
        py: Python,
        positions: Py<PyArray2<f32>>,
        vertex_indices: Py<PyArray1<u32>>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let positions = positions.map_py(py)?;
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
        let positions = positions.map_py(py)?;
        let normals = normals.map_py(py)?;
        let uvs = uvs.map_py(py)?;

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

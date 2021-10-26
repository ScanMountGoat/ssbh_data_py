use crate::create_py_list_from_slice;
use crate::MapPy;
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data::mesh_data::VectorData as VectorDataRs;
use ssbh_data::SsbhData;
use ssbh_data_py_derive::MapPy;

create_exception!(ssbh_data_py, MeshDataError, pyo3::exceptions::PyException);

pub fn mesh_data(py: Python, module: &PyModule) -> PyResult<()> {
    let mesh_data = PyModule::new(py, "mesh_data")?;
    mesh_data.add_class::<MeshData>()?;
    mesh_data.add_class::<MeshObjectData>()?;
    mesh_data.add_class::<AttributeData>()?;
    mesh_data.add_class::<BoneInfluence>()?;
    mesh_data.add_class::<VertexWeight>()?;

    mesh_data.add_function(wrap_pyfunction!(read_mesh, mesh_data)?)?;
    mesh_data.add_function(wrap_pyfunction!(transform_points, mesh_data)?)?;
    mesh_data.add_function(wrap_pyfunction!(transform_vectors, mesh_data)?)?;
    mesh_data.add_function(wrap_pyfunction!(calculate_smooth_normals, mesh_data)?)?;
    mesh_data.add_function(wrap_pyfunction!(calculate_tangents_vec4, mesh_data)?)?;

    module.add_submodule(mesh_data)?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::mesh_data::MeshData)]
struct MeshData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    pub objects: Py<PyList>,
}

#[pymethods]
impl MeshData {
    #[new]
    #[args(major_version = 1, minor_version = 10)]
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
}

#[pyclass]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::mesh_data::MeshObjectData)]
pub struct MeshObjectData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub sub_index: u64,

    #[pyo3(get, set)]
    pub parent_bone_name: String,

    #[pyo3(get, set)]
    pub vertex_indices: PyObject,

    #[pyo3(get, set)]
    pub positions: Py<PyList>,

    #[pyo3(get, set)]
    pub normals: Py<PyList>,

    #[pyo3(get, set)]
    pub binormals: Py<PyList>,

    #[pyo3(get, set)]
    pub tangents: Py<PyList>,

    #[pyo3(get, set)]
    pub texture_coordinates: Py<PyList>,

    #[pyo3(get, set)]
    pub color_sets: Py<PyList>,

    #[pyo3(get, set)]
    pub bone_influences: Py<PyList>,
}

#[pymethods]
impl MeshObjectData {
    #[new]
    fn new(py: Python, name: String, sub_index: u64) -> PyResult<Self> {
        Ok(MeshObjectData {
            name,
            sub_index,
            parent_bone_name: "".to_string(),
            vertex_indices: PyList::empty(py).into(),
            positions: PyList::empty(py).into(),
            normals: PyList::empty(py).into(),
            binormals: PyList::empty(py).into(),
            tangents: PyList::empty(py).into(),
            texture_coordinates: PyList::empty(py).into(),
            color_sets: PyList::empty(py).into(),
            bone_influences: PyList::empty(py).into(),
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::mesh_data::BoneInfluence)]
pub struct BoneInfluence {
    #[pyo3(get, set)]
    pub bone_name: String,
    #[pyo3(get, set)]
    pub vertex_weights: Py<PyList>,
}

#[pymethods]
impl BoneInfluence {
    #[new]
    fn new(py: Python, bone_name: String, vertex_weights: Vec<VertexWeight>) -> PyResult<Self> {
        Ok(BoneInfluence {
            bone_name,
            vertex_weights: PyList::new(
                py,
                vertex_weights
                    .into_iter()
                    .map(|w| Py::new(py, w))
                    .collect::<Result<Vec<Py<VertexWeight>>, _>>()?,
            )
            .into(),
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::mesh_data::VertexWeight)]
pub struct VertexWeight {
    #[pyo3(get, set)]
    pub vertex_index: u32,

    #[pyo3(get, set)]
    pub vertex_weight: f32,
}

#[pymethods]
impl VertexWeight {
    #[new]
    fn new(vertex_index: u32, vertex_weight: f32) -> PyResult<Self> {
        Ok(VertexWeight {
            vertex_index,
            vertex_weight,
        })
    }
}

#[pyclass]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::mesh_data::AttributeData)]
pub struct AttributeData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub data: PyObject,
}

#[pymethods]
impl AttributeData {
    #[new]
    fn new(py: Python, name: String) -> PyResult<Self> {
        Ok(AttributeData {
            name,
            data: PyList::empty(py).into(),
        })
    }
}

impl MapPy<PyObject> for VectorDataRs {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(match self {
            VectorDataRs::Vector2(v) => create_py_list_from_slice(py, v).into(),
            VectorDataRs::Vector3(v) => create_py_list_from_slice(py, v).into(),
            VectorDataRs::Vector4(v) => create_py_list_from_slice(py, v).into(),
        })
    }
}

impl MapPy<VectorDataRs> for PyObject {
    fn map_py(&self, py: Python) -> PyResult<VectorDataRs> {
        // We don't know the type from Python at this point.
        // Try all the supported types and fail if all conversions fail.
        // TODO: This still works with numpy arrays but might not be the most efficient.
        self.extract::<Vec<[f32; 2]>>(py)
            .map(VectorDataRs::Vector2)
            .or_else(|_| self.extract::<Vec<[f32; 3]>>(py).map(VectorDataRs::Vector3))
            .or_else(|_| self.extract::<Vec<[f32; 4]>>(py).map(VectorDataRs::Vector4))
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
    let transform = transform.extract::<[[f32; 4]; 4]>(py)?;
    let transformed_points = ssbh_data::mesh_data::transform_points(&points, &transform);
    transformed_points.map_py(py)
}

#[pyfunction]
fn transform_vectors(py: Python, points: PyObject, transform: PyObject) -> PyResult<PyObject> {
    let points = points.map_py(py)?;
    let transform = transform.extract::<[[f32; 4]; 4]>(py)?;
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
    Ok(create_py_list_from_slice(py, &normals))
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

    Ok(create_py_list_from_slice(py, &tangents))
}

#[cfg(test)]
mod tests {
    use crate::MapPy;
    use crate::{eval_python_code, eval_python_code_numpy, run_python_code, run_python_code_numpy};
    use indoc::indoc;
    use pyo3::PyObject;
    use ssbh_data::mesh_data::VectorData;

    #[test]
    fn create_mesh() {
        run_python_code(indoc! {r#"
            m = ssbh_data_py.mesh_data.MeshData(3, 4)
            assert m.major_version == 3
            assert m.minor_version == 4

            m = ssbh_data_py.mesh_data.MeshData(3)
            assert m.major_version == 3
            assert m.minor_version == 10

            m = ssbh_data_py.mesh_data.MeshData()
            assert m.major_version == 1
            assert m.minor_version == 10
        "#})
        .unwrap();
    }

    #[test]
    fn create_modify_mesh_object() {
        run_python_code(indoc! {r#"
            m = ssbh_data_py.mesh_data.MeshObjectData("abc", 1)
            assert m.name == "abc"
            assert m.sub_index == 1
            assert m.parent_bone_name == ""
            assert m.vertex_indices == []
            assert m.positions == []
            assert m.normals == []
            assert m.binormals == []
            assert m.tangents == []
            assert m.texture_coordinates == []
            assert m.color_sets == []
            assert m.bone_influences == []

            m.vertex_indices = [1, 2, 3]
            assert m.vertex_indices == [1,2,3]
        "#})
        .unwrap();
    }

    #[test]
    fn mesh_object_vertex_indices_ndarray() {
        run_python_code_numpy(indoc! {r#"
            m = ssbh_data_py.mesh_data.MeshObjectData("abc", 1)
            m.vertex_indices = numpy.array([1, 2, 3])
            assert m.vertex_indices.tolist() == [1,2,3]
        "#})
        .unwrap();
    }

    #[test]
    fn create_modify_attribute_data() {
        run_python_code(indoc! {r#"
            a = ssbh_data_py.mesh_data.AttributeData("abc")
            assert a.name == "abc"
            assert a.data == []

            a.name = "def"
            a.data = [[1.0, 2.0]]
            assert a.name == "def"
            assert a.data == [[1.0, 2.0]]

            # Test mutability for nested types.
            a.data[0][1] = 0.3
            assert a.data == [[1.0, 0.3]]
            a.data[0] = [2.5, 3.5]
            assert a.data == [[2.5, 3.5]]
        "#})
        .unwrap();
    }

    #[test]
    fn create_modify_vertex_weight() {
        run_python_code(indoc! {r#"
            v = ssbh_data_py.mesh_data.VertexWeight(1, 0.5)
            assert v.vertex_index == 1
            assert v.vertex_weight == 0.5

            v.vertex_index = 0
            v.vertex_weight = 0.0
            assert v.vertex_index == 0
            assert v.vertex_weight == 0.0
        "#})
        .unwrap();
    }

    #[test]
    fn create_modify_bone_influence() {
        run_python_code(indoc! {r#"
            b = ssbh_data_py.mesh_data.BoneInfluence("abc", [])
            assert b.bone_name == "abc"
            assert b.vertex_weights == []

            b.bone_name = "def"
            b.vertex_weights = [ssbh_data_py.mesh_data.VertexWeight(1, 0.5)]
            assert b.bone_name == "def"
            assert len(b.vertex_weights) == 1
            assert b.vertex_weights[0].vertex_weight == 0.5

            # Test mutability for nested types.
            b.vertex_weights[0].vertex_index = 2
            assert b.vertex_weights[0].vertex_index == 2
        "#})
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn vector2_from_pylist_invalid_type() {
        eval_python_code("[[0, 1], [2, 'a']]", |py, x| {
            let _: VectorData = PyObject::from(x).map_py(py).unwrap();
        });
    }

    #[test]
    #[should_panic]
    fn vector2_from_pylist_invalid_component_count() {
        eval_python_code("[[0.0, 1.0], [2.0]]", |py, x| {
            let _: VectorData = PyObject::from(x).map_py(py).unwrap();
        });
    }

    #[test]
    fn vector2_from_pylist_ints() {
        eval_python_code("[[0, 1], [2, 3]]", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector2_from_ndarray_ints() {
        eval_python_code_numpy("numpy.array([[0, 1], [2, 3]],dtype=numpy.int8)", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector2_from_pylist() {
        eval_python_code("[[0.0, 1.0], [2.0, 3.0]]", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector2_from_tuples() {
        eval_python_code("[(0.0, 1.0), (2.0, 3.0)]", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector2_from_ndarray() {
        eval_python_code_numpy("numpy.array([[0.0, 1.0], [2.0, 3.0]])", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector3_from_pylist() {
        eval_python_code("[[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(
                VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
                value
            );
        });
    }

    #[test]
    fn vector3_from_tuples() {
        eval_python_code("[(0.0, 1.0, 2.0), (3.0, 4.0, 5.0)]", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(
                VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
                value
            );
        });
    }

    #[test]
    fn vector3_from_ndarray() {
        eval_python_code_numpy(
            "numpy.array([[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]])",
            |py, x| {
                let value = PyObject::from(x).map_py(py).unwrap();
                assert_eq!(
                    VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
                    value
                );
            },
        );
    }

    #[test]
    fn vector4_from_pylist() {
        eval_python_code("[[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(
                VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
                value
            );
        });
    }

    #[test]
    fn vector4_from_tuples() {
        eval_python_code("[(0.0, 1.0, 2.0, 3.0), (4.0, 5.0, 6.0, 7.0)]", |py, x| {
            let value = PyObject::from(x).map_py(py).unwrap();
            assert_eq!(
                VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
                value
            );
        });
    }

    #[test]
    fn vector4_from_ndarray() {
        eval_python_code_numpy(
            "numpy.array([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]])",
            |py, x| {
                let value = PyObject::from(x).map_py(py).unwrap();
                assert_eq!(
                    VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
                    value
                );
            },
        );
    }

    #[test]
    #[should_panic]
    fn vector_from_5x5_pylist() {
        // Vector5 is not a valid variant.
        eval_python_code("[[1.0,2.0,3.0,4.0,5.0]]", |py, x| {
            let _: VectorData = PyObject::from(x).map_py(py).unwrap();
        });
    }

    #[test]
    #[should_panic]
    fn vector_from_5x5_ndarray() {
        // Vector5 is not a valid variant.
        eval_python_code_numpy("numpy.zeros((5,5))", |py, x| {
            let _: VectorData = PyObject::from(x).map_py(py).unwrap();
        });
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn vector_from_empty_pylist() {
        // TODO: How to infer the type when there are no elements?
        eval_python_code("[]", |py, x| {
            let _: VectorData = PyObject::from(x).map_py(py).unwrap();
        });
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn vector_from_empty_ndarray() {
        // TODO: How to infer the type when there are no elements?
        eval_python_code_numpy("numpy.array()", |py, x| {
            let _: VectorData = PyObject::from(x).map_py(py).unwrap();
        });
    }

    #[test]
    fn transform_points_pylist() {
        run_python_code(indoc! {r#"
            points = [[1,2,3],[4,5,6]]
            transform = [
                [1,0,0,0],
                [0,1,0,0],
                [0,0,1,0],
                [-1,-2,-3,1]
            ]
            transformed = ssbh_data_py.mesh_data.transform_points(points, transform)
            assert transformed == [[0,0,0],[3,3,3]]
        "#})
        .unwrap();
    }

    #[test]
    fn transform_points_tuple() {
        run_python_code(indoc! {r#"
            points = ((1,2,3),(4,5,6))
            transform = (
                (1,0,0,0),
                (0,1,0,0),
                (0,0,1,0),
                (-1,-2,-3,1)
            )
            transformed = ssbh_data_py.mesh_data.transform_points(points, transform)
            assert transformed == [[0,0,0],[3,3,3]]
        "#})
        .unwrap();
    }

    #[test]
    fn transform_points_ndarray() {
        run_python_code_numpy(indoc! {r#"
            points = numpy.array([[1,2,3],[4,5,6]])
            transform = numpy.array([
                [1,0,0,0],
                [0,1,0,0],
                [0,0,1,0],
                [-1,-2,-3,1]
            ])
            transformed = ssbh_data_py.mesh_data.transform_points(points, transform)
            assert transformed == [[0,0,0],[3,3,3]]
        "#})
        .unwrap();
    }

    #[test]
    fn transform_vectors_pylist() {
        run_python_code(indoc! {r#"
            points = [[1,2,3],[4,5,6]]
            transform = [
                [1,0,0,0],
                [0,1,0,0],
                [0,0,1,0],
                [-1,-2,-3,1]
            ]
            transformed = ssbh_data_py.mesh_data.transform_vectors(points, transform)
            assert transformed == [[1,2,3],[4,5,6]]
        "#})
        .unwrap();
    }

    #[test]
    fn transform_vectors_tuple() {
        run_python_code(indoc! {r#"
            points = ((1,2,3),(4,5,6))
            transform = (
                (1,0,0,0),
                (0,1,0,0),
                (0,0,1,0),
                (-1,-2,-3,1)
            )
            transformed = ssbh_data_py.mesh_data.transform_vectors(points, transform)
            assert transformed == [[1,2,3],[4,5,6]]
        "#})
        .unwrap();
    }

    #[test]
    fn transform_vectors_ndarray() {
        run_python_code_numpy(indoc! {r#"
            points = numpy.array([[1,2,3],[4,5,6]])
            transform = numpy.array([
                [1,0,0,0],
                [0,1,0,0],
                [0,0,1,0],
                [-1,-2,-3,1]
            ])
            transformed = ssbh_data_py.mesh_data.transform_vectors(points, transform)
            assert transformed == [[1,2,3],[4,5,6]]
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_smooth_normals_pylist() {
        run_python_code(indoc! {r#"
            ssbh_data_py.mesh_data.calculate_smooth_normals([[0,0,0]]*36, list(range(36)))
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_smooth_normals_tuple() {
        run_python_code(indoc! {r#"
            ssbh_data_py.mesh_data.calculate_smooth_normals(((0,0,0),(1,1,1),(2,2,2)), (0,1,2))
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_smooth_normals_ndarray() {
        run_python_code_numpy(indoc! {r#"
            ssbh_data_py.mesh_data.calculate_smooth_normals(numpy.zeros((12,4)), numpy.arange(12))
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_tangents_vec4_pylist() {
        run_python_code(indoc! {r#"
            ssbh_data_py.mesh_data.calculate_tangents_vec4([[0,0,0],[1,1,1],[2,2,2]], [[0,0,0],[1,1,1],[2,2,2]], [[0,0],[1,1],[2,2]], [0,1,2])
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_tangents_vec4_tuple() {
        run_python_code(indoc! {r#"
            ssbh_data_py.mesh_data.calculate_tangents_vec4(((0,0,0),(1,1,1),(2,2,2)), ((0,0,0),(1,1,1),(2,2,2)), ((0,0),(1,1),(2,2)), (0,1,2))
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_tangents_vec4_ndarray() {
        run_python_code_numpy(indoc! {r#"
            ssbh_data_py.mesh_data.calculate_tangents_vec4(numpy.zeros((12,4)), numpy.zeros((12,4)), numpy.zeros((12,2)), numpy.arange(12))
        "#})
        .unwrap();
    }
}

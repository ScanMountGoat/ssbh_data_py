use pyo3::wrap_pyfunction;
use pyo3::{prelude::*, types::PyList};
use ssbh_data::mesh_data::VectorData as VectorDataRs;
use ssbh_data::SsbhData;

use crate::{create_py_list, create_py_list_from_slice, create_vec};

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

    module.add_submodule(mesh_data)?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone)]
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
        let objects: Vec<_> = create_vec(py, &self.objects, create_mesh_object_rs)?;
        let mesh_data = ssbh_data::mesh_data::MeshData {
            major_version: self.major_version,
            minor_version: self.minor_version,
            objects,
        };

        // TODO: Convert these errors to python exceptions instead of relying on panic handler?
        mesh_data.write_to_file(path).unwrap();
        Ok(())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
                    .map(|w| Py::new(py, w).unwrap())
                    .collect::<Vec<Py<VertexWeight>>>(),
            )
            .into(),
        })
    }
}

#[pyclass]
#[derive(Debug, Clone)]
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

fn create_mesh_object_rs(
    py: Python,
    data: &MeshObjectData,
) -> PyResult<ssbh_data::mesh_data::MeshObjectData> {
    Ok(ssbh_data::mesh_data::MeshObjectData {
        name: data.name.clone(),
        sub_index: data.sub_index,
        parent_bone_name: data.parent_bone_name.clone(),
        vertex_indices: data.vertex_indices.extract::<Vec<u32>>(py).unwrap(),
        positions: create_vec(py, &data.positions, create_attribute_rs)?,
        normals: create_vec(py, &data.normals, create_attribute_rs)?,
        binormals: create_vec(py, &data.binormals, create_attribute_rs)?,
        tangents: create_vec(py, &data.tangents, create_attribute_rs)?,
        texture_coordinates: create_vec(py, &data.texture_coordinates, create_attribute_rs)?,
        color_sets: create_vec(py, &data.color_sets, create_attribute_rs)?,
        bone_influences: create_vec(py, &data.bone_influences, create_bone_influence_rs)?,
    })
}

fn create_mesh_object_py(
    py: Python,
    data: &ssbh_data::mesh_data::MeshObjectData,
) -> PyResult<MeshObjectData> {
    Ok(MeshObjectData {
        name: data.name.clone(),
        sub_index: data.sub_index,
        parent_bone_name: data.parent_bone_name.clone(),
        vertex_indices: create_py_list_from_slice(py, &data.vertex_indices).into(),
        positions: create_py_list(py, &data.positions, create_attribute_data_py)?,
        normals: create_py_list(py, &data.normals, create_attribute_data_py)?,
        binormals: create_py_list(py, &data.binormals, create_attribute_data_py)?,
        tangents: create_py_list(py, &data.tangents, create_attribute_data_py)?,
        texture_coordinates: create_py_list(
            py,
            &data.texture_coordinates,
            create_attribute_data_py,
        )?,
        color_sets: create_py_list(py, &data.color_sets, create_attribute_data_py)?,
        bone_influences: create_py_list(py, &data.bone_influences, create_bone_influence)?,
    })
}

fn create_bone_influence(
    py: Python,
    influence: &ssbh_data::mesh_data::BoneInfluence,
) -> PyResult<BoneInfluence> {
    Ok(BoneInfluence {
        bone_name: influence.bone_name.clone(),
        vertex_weights: create_py_list(py, &influence.vertex_weights, |_, w| {
            Ok(VertexWeight {
                vertex_index: w.vertex_index,
                vertex_weight: w.vertex_weight,
            })
        })?,
    })
}

fn create_attribute_data_py(
    py: Python,
    attribute_data: &ssbh_data::mesh_data::AttributeData,
) -> PyResult<AttributeData> {
    Ok(AttributeData {
        name: attribute_data.name.clone(),
        data: vector_data_to_py_list(py, &attribute_data.data)?.into(),
    })
}

fn vector_data_to_py_list(py: Python, data: &VectorDataRs) -> PyResult<Py<PyList>> {
    // TODO: Investigate if it's worth converting to tuples.
    // TODO: Numpy?
    // This substantially improves performance.
    Ok(match &data {
        VectorDataRs::Vector2(v) => PyList::new(py, v.iter().map(|[x,y]| (x,y))).into(),
        VectorDataRs::Vector3(v) => PyList::new(py, v.iter().map(|[x,y,z]| (x,y,z))).into(),
        VectorDataRs::Vector4(v) => PyList::new(py, v.iter().map(|[x,y,z,w]| (x,y,z,w))).into(),
    })
}

#[pyclass]
#[derive(Debug, Clone)]
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

fn create_attribute_rs(
    py: Python,
    attribute: &AttributeData,
) -> PyResult<ssbh_data::mesh_data::AttributeData> {
    Ok(ssbh_data::mesh_data::AttributeData {
        name: attribute.name.clone(),
        data: create_vector_data_rs(&attribute.data.as_ref(py))?,
    })
}

fn create_vector_data_rs(data: &PyAny) -> PyResult<VectorDataRs> {
    // We don't know the type from Python at this point.
    // Try all the supported types and fail if all conversions fail.
    // TODO: This still works with numpy arrays but might not be the most efficient.
    data.extract::<Vec<[f32; 2]>>()
        .map(VectorDataRs::Vector2)
        .or_else(|_| data.extract::<Vec<[f32; 3]>>().map(VectorDataRs::Vector3))
        .or_else(|_| data.extract::<Vec<[f32; 4]>>().map(VectorDataRs::Vector4))
}

fn create_bone_influence_rs(
    py: Python,
    influence: &BoneInfluence,
) -> PyResult<ssbh_data::mesh_data::BoneInfluence> {
    Ok(ssbh_data::mesh_data::BoneInfluence {
        bone_name: influence.bone_name.clone(),
        vertex_weights: create_vec(py, &influence.vertex_weights, |_, w: &VertexWeight| {
            Ok(ssbh_data::mesh_data::VertexWeight {
                vertex_index: w.vertex_index,
                vertex_weight: w.vertex_weight,
            })
        })?,
    })
}

#[pyfunction]
fn read_mesh(py: Python, path: &str) -> PyResult<MeshData> {
    match ssbh_data::mesh_data::MeshData::from_file(path) {
        Ok(mesh_data) => {
            let objects: Result<Vec<_>, _> = mesh_data
                .objects
                .iter()
                .map(|o| Py::new(py, create_mesh_object_py(py, o)?))
                .collect();

            Ok(MeshData {
                major_version: mesh_data.major_version,
                minor_version: mesh_data.minor_version,
                objects: PyList::new(py, objects?).into(),
            })
        }
        // TODO: How to handle errors or return None?
        _ => panic!("Failed to read mesh."),
    }
}

#[pyfunction]
fn transform_points(py: Python, points: Py<PyList>, transform: &PyList) -> PyResult<Py<PyList>> {
    let points = create_vector_data_rs(points.as_ref(py))?;
    let transform = transform.extract::<[[f32; 4]; 4]>()?;
    let transformed_points = ssbh_data::mesh_data::transform_points(&points, &transform);
    vector_data_to_py_list(py, &transformed_points)
}

#[pyfunction]
fn transform_vectors(py: Python, points: Py<PyList>, transform: &PyList) -> PyResult<Py<PyList>> {
    let points = create_vector_data_rs(points.as_ref(py))?;
    let transform = transform.extract::<[[f32; 4]; 4]>()?;
    let transformed_points = ssbh_data::mesh_data::transform_vectors(&points, &transform);
    vector_data_to_py_list(py, &transformed_points)
}

#[cfg(test)]
mod tests {
    use crate::{eval_python_code, eval_python_code_numpy, mesh_data::create_vector_data_rs, run_python_code, run_python_code_numpy};
    use indoc::indoc;
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
        eval_python_code("[[0, 1], [2, 'a']]", |_, x| {
            create_vector_data_rs(x).unwrap();
        });
    }

    #[test]
    #[should_panic]
    fn vector2_from_pylist_invalid_component_count() {
        eval_python_code("[[0.0, 1.0], [2.0]]", |_, x| {
            create_vector_data_rs(x).unwrap();
        });
    }

    #[test]
    fn vector2_from_pylist_ints() {
        eval_python_code("[[0, 1], [2, 3]]", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector2_from_ndarray_ints() {
        eval_python_code_numpy("numpy.array([[0, 1], [2, 3]],dtype=numpy.int8)", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector2_from_pylist() {
        eval_python_code("[[0.0, 1.0], [2.0, 3.0]]", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector2_from_tuples() {
        eval_python_code("[(0.0, 1.0), (2.0, 3.0)]", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector2_from_ndarray() {
        eval_python_code_numpy("numpy.array([[0.0, 1.0], [2.0, 3.0]])", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        });
    }

    #[test]
    fn vector3_from_pylist() {
        eval_python_code("[[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(
                VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
                value
            );
        });
    }

    #[test]
    fn vector3_from_tuples() {
        eval_python_code("[(0.0, 1.0, 2.0), (3.0, 4.0, 5.0)]", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(
                VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
                value
            );
        });
    }

    #[test]
    fn vector3_from_ndarray() {
        eval_python_code_numpy("numpy.array([[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]])", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(
                VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
                value
            );
        });
    }

    #[test]
    fn vector4_from_pylist() {
        eval_python_code("[[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(
                VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
                value
            );
        });
    }

    
    #[test]
    fn vector4_from_tuples() {
        eval_python_code("[(0.0, 1.0, 2.0, 3.0), (4.0, 5.0, 6.0, 7.0)]", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(
                VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
                value
            );
        });
    }

    #[test]
    fn vector4_from_ndarray() {
        eval_python_code_numpy("numpy.array([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]])", |_, x| {
            let value = create_vector_data_rs(x).unwrap();
            assert_eq!(
                VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
                value
            );
        });
    }

    #[test]
    #[should_panic]
    fn vector_from_5x5_pylist() {
        // Vector5 is not a valid variant.
        eval_python_code("[[1.0,2.0,3.0,4.0,5.0]]", |_, x| {
            create_vector_data_rs(x).unwrap();
        });
    }

    #[test]
    #[should_panic]
    fn vector_from_5x5_ndarray() {
        // Vector5 is not a valid variant.
        eval_python_code_numpy("numpy.zeros((5,5))", |_, x| {
            create_vector_data_rs(x).unwrap();
        });
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn vector_from_empty_pylist() {
        // TODO: How to infer the type when there are no elements?
        eval_python_code("[]", |_, x| {
            create_vector_data_rs(x).unwrap();
        });
    }

    #[test]
    #[ignore]
    #[should_panic]
    fn vector_from_empty_ndarray() {
        // TODO: How to infer the type when there are no elements?
        eval_python_code_numpy("numpy.array()", |_, x| {
            create_vector_data_rs(x).unwrap();
        });
    }
}

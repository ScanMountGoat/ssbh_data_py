use pyo3::wrap_pyfunction;
use pyo3::{prelude::*, types::PyList};

use crate::{create_py_list, create_py_list_from_slice, create_vec};

pub fn mesh_data(py: Python, module: &PyModule) -> PyResult<()> {
    let mesh_data = PyModule::new(py, "mesh_data")?;
    mesh_data.add_class::<Mesh>()?;
    mesh_data.add_class::<MeshObjectData>()?;
    mesh_data.add_class::<AttributeData>()?;
    mesh_data.add_class::<BoneInfluence>()?;
    mesh_data.add_class::<VertexWeight>()?;

    mesh_data.add_function(wrap_pyfunction!(read_mesh, mesh_data)?)?;

    module.add_submodule(mesh_data)?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone)]
struct Mesh {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    pub objects: Py<PyList>,
}

#[pymethods]
impl Mesh {
    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        let objects: Vec<_> = create_vec(py, &self.objects, create_mesh_object_rs)?;

        // TODO: Convert these errors to python exceptions instead of relying on panic handler?
        let ssbh_mesh = ssbh_data::mesh_data::create_mesh(
            self.major_version,
            self.minor_version,
            objects.as_slice(),
        )
        .unwrap();

        ssbh_mesh.write_to_file(path).unwrap();
        Ok(())
    }
}

#[pymethods]
impl Mesh {
    #[new]
    #[args(major_version = 1, minor_version = 10)]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(Mesh {
            major_version,
            minor_version,
            objects: PyList::empty(py).into(),
        })
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
    pub vertex_indices: Py<PyList>,

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
        vertex_indices: create_py_list_from_slice(py, &data.vertex_indices),
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
    let data = match &attribute_data.data {
        ssbh_data::mesh_data::VectorData::Vector2(v) => create_py_list_from_slice(py, v),
        ssbh_data::mesh_data::VectorData::Vector3(v) => create_py_list_from_slice(py, v),
        ssbh_data::mesh_data::VectorData::Vector4(v) => create_py_list_from_slice(py, v),
    };
    Ok(AttributeData {
        name: attribute_data.name.clone(),
        data,
    })
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct AttributeData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub data: Py<PyList>,
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
    // We don't know the type from Python at this point.
    // Try all the supported types and fail if all conversions fail.
    if let Ok(v) = attribute.data.extract::<Vec<[f32; 2]>>(py) {
        Ok(ssbh_data::mesh_data::AttributeData {
            name: attribute.name.clone(),
            data: ssbh_data::mesh_data::VectorData::Vector2(v),
        })
    } else if let Ok(v) = attribute.data.extract::<Vec<[f32; 3]>>(py) {
        Ok(ssbh_data::mesh_data::AttributeData {
            name: attribute.name.clone(),
            data: ssbh_data::mesh_data::VectorData::Vector3(v),
        })
    } else {
        match attribute.data.extract::<Vec<[f32; 4]>>(py) {
            Ok(v) => Ok(ssbh_data::mesh_data::AttributeData {
                name: attribute.name.clone(),
                data: ssbh_data::mesh_data::VectorData::Vector4(v),
            }),
            Err(e) => Err(e),
        }
    }
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

// TODO: In the future, this should be handled entirely by ssbh_data.
// It should be possible to do this without an ssbh_lib dependency.
#[pyfunction]
fn read_mesh(py: Python, path: &str) -> PyResult<Mesh> {
    match ssbh_lib::formats::mesh::Mesh::from_file(path) {
        Ok(mesh) => {
            let objects: Result<Vec<_>, _> = ssbh_data::mesh_data::read_mesh_objects(&mesh)
                .unwrap()
                .iter()
                .map(|o| Py::new(py, create_mesh_object_py(py, o)?))
                .collect();

            Ok(Mesh {
                major_version: mesh.major_version,
                minor_version: mesh.minor_version,
                objects: PyList::new(py, objects?).into(),
            })
        }
        // TODO: How to handle errors or return None?
        _ => panic!("Failed to read mesh."),
    }
}

#[cfg(test)]
mod tests {
    use pyo3::prelude::*;
    use pyo3::types::IntoPyDict;

    use indoc::indoc;

    use crate::ssbh_data_py;

    #[test]
    fn create_mesh() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
                m = ssbh_data_py.mesh_data.Mesh(3, 4)
                assert m.major_version == 3
                assert m.minor_version == 4

                m = ssbh_data_py.mesh_data.Mesh(3)
                assert m.major_version == 3
                assert m.minor_version == 10

                m = ssbh_data_py.mesh_data.Mesh()
                assert m.major_version == 1
                assert m.minor_version == 10
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }

    #[test]
    fn create_mesh_object() {
        // TODO: Wrap initialization in a function?
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
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
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }

    #[test]
    fn create_modify_attribute_data() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
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
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }

    #[test]
    fn create_modify_vertex_weight() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
                v = ssbh_data_py.mesh_data.VertexWeight(1, 0.5)
                assert v.vertex_index == 1
                assert v.vertex_weight == 0.5

                v.vertex_index = 0
                v.vertex_weight = 0.0
                assert v.vertex_index == 0
                assert v.vertex_weight == 0.0
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }

    #[test]
    fn create_modify_bone_influence() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
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
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }
}

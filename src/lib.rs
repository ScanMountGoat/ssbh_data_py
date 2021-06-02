use pyo3::type_object::PyBorrowFlagLayout;
use pyo3::{prelude::*, types::PyList};
use pyo3::{wrap_pyfunction, PyClass};

#[pyclass]
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
        // TODO: Convert errors to Python exception?
        let objects: Vec<_> = create_rs_list(py, &self.objects, create_mesh_object_rs);

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
    #[args(major_version=1, minor_version=10)]
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
    fn new(py: Python, name: &str, sub_index: u64) -> PyResult<Self> {
        Ok(MeshObjectData {
            name: name.to_string(),
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
    fn new(py: Python, bone_name: &str, vertex_weights: Vec<VertexWeight>) -> PyResult<Self> {
        Ok(BoneInfluence {
            bone_name: bone_name.to_string(),
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

// TODO: Return result.
fn create_mesh_object_rs(
    py: Python,
    data: &MeshObjectData,
) -> ssbh_data::mesh_data::MeshObjectData {
    ssbh_data::mesh_data::MeshObjectData {
        name: data.name.clone(),
        sub_index: data.sub_index,
        parent_bone_name: data.parent_bone_name.clone(),
        vertex_indices: data.vertex_indices.extract::<Vec<u32>>(py).unwrap(),
        positions: create_rs_list(py, &data.positions, create_attribute_rs),
        normals: create_rs_list(py, &data.normals, create_attribute_rs),
        binormals: create_rs_list(py, &data.binormals, create_attribute_rs),
        tangents: create_rs_list(py, &data.tangents, create_attribute_rs),
        texture_coordinates: create_rs_list(py, &data.texture_coordinates, create_attribute_rs),
        color_sets: create_rs_list(py, &data.color_sets, create_attribute_rs),
        bone_influences: create_rs_list(py, &data.bone_influences, create_bone_influence_rs),
    }
}

fn create_py_list<T, C: PyClass, U: Into<PyClassInitializer<C>>, F: Fn(Python, &T) -> U>(
    py: Python,
    elements: &[T],
    create_p: F,
) -> PyResult<Py<PyList>>
where
    C::BaseLayout: PyBorrowFlagLayout<C::BaseType>,
{
    let items: Result<Vec<_>, _> = elements
        .iter()
        .map(|e| Py::new(py, create_p(py, e)))
        .collect();

    Ok(PyList::new(py, items?).into())
}

fn create_py_list_from_slice<T: IntoPy<U> + Copy, U: ToPyObject>(
    py: Python,
    elements: &[T],
) -> Py<PyList> {
    PyList::new(py, elements.iter().map(|m| m.into_py(py))).into()
}

// TODO: This should return a result.
fn create_rs_list<T, P: PyClass + Clone, F: Fn(Python, &P) -> T>(
    py: Python,
    elements: &Py<PyList>,
    create_t: F,
) -> Vec<T> {
    elements
        .as_ref(py)
        .iter()
        .filter_map(|i| i.extract::<P>().ok())
        .map(|i| create_t(py, &i))
        .collect()
}

// TODO: Return a result?
fn create_mesh_object_py(
    py: Python,
    data: &ssbh_data::mesh_data::MeshObjectData,
) -> MeshObjectData {
    MeshObjectData {
        name: data.name.clone(),
        sub_index: data.sub_index,
        parent_bone_name: data.parent_bone_name.clone(),
        vertex_indices: create_py_list_from_slice(py, &data.vertex_indices),
        positions: create_py_list(py, &data.positions, create_attribute_data_py).unwrap(),
        normals: create_py_list(py, &data.normals, create_attribute_data_py).unwrap(),
        binormals: create_py_list(py, &data.binormals, create_attribute_data_py).unwrap(),
        tangents: create_py_list(py, &data.tangents, create_attribute_data_py).unwrap(),
        texture_coordinates: create_py_list(
            py,
            &data.texture_coordinates,
            create_attribute_data_py,
        )
        .unwrap(),
        color_sets: create_py_list(py, &data.color_sets, create_attribute_data_py).unwrap(),
        bone_influences: create_py_list(py, &data.bone_influences, create_bone_influence).unwrap(),
    }
}

// TODO: Return a result?
fn create_bone_influence(
    py: Python,
    influence: &ssbh_data::mesh_data::BoneInfluence,
) -> BoneInfluence {
    BoneInfluence {
        bone_name: influence.bone_name.clone(),
        vertex_weights: create_py_list(py, &influence.vertex_weights, |_, w| VertexWeight {
            vertex_index: w.vertex_index,
            vertex_weight: w.vertex_weight,
        })
        .unwrap(),
    }
}

fn create_attribute_data_py(
    py: Python,
    attribute_data: &ssbh_data::mesh_data::AttributeData,
) -> AttributeData {
    let data = match &attribute_data.data {
        ssbh_data::mesh_data::VectorData::Vector2(v) => create_py_list_from_slice(py, v),
        ssbh_data::mesh_data::VectorData::Vector3(v) => create_py_list_from_slice(py, v),
        ssbh_data::mesh_data::VectorData::Vector4(v) => create_py_list_from_slice(py, v),
    };
    AttributeData {
        name: attribute_data.name.clone(),
        data,
    }
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
    fn new(py: Python, name: &str) -> PyResult<Self> {
        Ok(AttributeData {
            name: name.to_string(),
            data: PyList::empty(py).into(),
        })
    }
}

// TODO: Return result.
fn create_attribute_rs(
    py: Python,
    attribute: &AttributeData,
) -> ssbh_data::mesh_data::AttributeData {
    // Filter out objects of the wrong type.
    // TODO: Throw an error instead?
    // HACK: Convert to vec first to get around PyO3 not supporting arrays with length larger than 32.

    // We don't know the type from Python at this, point so try all of them.
    // TODO: Is there a nicer way to write this?
    if let Ok(v) = attribute.data.extract::<Vec<[f32; 2]>>(py) {
        ssbh_data::mesh_data::AttributeData {
            name: attribute.name.clone(),
            data: ssbh_data::mesh_data::VectorData::Vector2(v),
        }
    } else if let Ok(v) = attribute.data.extract::<Vec<[f32; 3]>>(py) {
        ssbh_data::mesh_data::AttributeData {
            name: attribute.name.clone(),
            data: ssbh_data::mesh_data::VectorData::Vector3(v),
        }
    } else if let Ok(v) = attribute.data.extract::<Vec<[f32; 4]>>(py) {
        ssbh_data::mesh_data::AttributeData {
            name: attribute.name.clone(),
            data: ssbh_data::mesh_data::VectorData::Vector4(v),
        }
    } else {
        panic!("Unsupported type")
    }
}

fn create_bone_influence_rs(
    py: Python,
    influence: &BoneInfluence,
) -> ssbh_data::mesh_data::BoneInfluence {
    ssbh_data::mesh_data::BoneInfluence {
        bone_name: influence.bone_name.clone(),
        vertex_weights: create_rs_list(py, &influence.vertex_weights, |_, w: &VertexWeight| {
            ssbh_data::mesh_data::VertexWeight {
                vertex_index: w.vertex_index,
                vertex_weight: w.vertex_weight,
            }
        }),
    }
}

#[pyfunction]
fn read_mesh(py: Python, path: &str) -> PyResult<Mesh> {
    // TODO: How to handle errors or return None?
    match ssbh_lib::formats::mesh::Mesh::from_file(path) {
        Ok(mesh) => {
            let objects: Vec<_> = ssbh_data::mesh_data::read_mesh_objects(&mesh)
                .unwrap()
                .iter()
                .map(|o| Py::new(py, create_mesh_object_py(py, o)).unwrap())
                .collect();

            Ok(Mesh {
                major_version: mesh.major_version,
                minor_version: mesh.minor_version,
                objects: PyList::new(py, objects).into(),
            })
        }
        _ => panic!("Failed to read mesh."),
    }
}

#[pymodule]
fn ssbh_data_py(py: Python, module: &PyModule) -> PyResult<()> {
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

#[cfg(test)]
mod tests {
    use pyo3::prelude::*;
    use pyo3::types::IntoPyDict;

    use crate::ssbh_data_py;

    use indoc::indoc;

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

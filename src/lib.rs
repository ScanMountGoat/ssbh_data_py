use pyo3::wrap_pyfunction;
use pyo3::{prelude::*, types::PyList};

#[pyclass]
struct Mesh {
    data: ssbh_lib::formats::mesh::Mesh,

    #[pyo3(get, set)]
    pub objects: Py<PyList>,
}

#[pymethods]
impl Mesh {
    fn save(&mut self, py: Python, path: &str) -> PyResult<()> {
        // Filter out objects of the wrong type.
        // TODO: Throw an error instead?
        let objects: Vec<_> = self
            .objects
            .as_ref(py)
            .iter()
            .filter_map(|o| o.extract::<MeshObjectData>().ok())
            .map(|o| create_mesh_object_rs(py, &o))
            .collect();

        ssbh_data::mesh_data::update_mesh(&mut self.data, objects.as_slice()).unwrap();

        self.data.write_to_file(path).unwrap();
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
    pub vertex_indices: Vec<u32>,

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

// TODO: Use macros to automatically generate these wrapper types?
#[pyclass]
#[derive(Debug, Clone)]
pub struct BoneInfluence {
    #[pyo3(get, set)]
    pub bone_name: String,
    // TODO: This should probably be pylist to allow for mutability.
    #[pyo3(get, set)]
    pub vertex_weights: Vec<VertexWeight>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct VertexWeight {
    #[pyo3(get, set)]
    pub vertex_index: u32,

    #[pyo3(get, set)]
    pub vertex_weight: f32,
}

fn create_mesh_object_rs(
    py: Python,
    data: &MeshObjectData,
) -> ssbh_data::mesh_data::MeshObjectData {
    ssbh_data::mesh_data::MeshObjectData {
        name: data.name.clone(),
        sub_index: data.sub_index,
        parent_bone_name: data.parent_bone_name.clone(),
        vertex_indices: data.vertex_indices.clone(),
        positions: create_attributes_rs(py, &data.positions),
        normals: create_attributes_rs(py, &data.normals),
        binormals: create_attributes_rs(py, &data.binormals),
        tangents: create_attributes_rs(py, &data.tangents),
        texture_coordinates: create_attributes_rs(py, &data.texture_coordinates),
        color_sets: create_attributes_rs(py, &data.color_sets),
        bone_influences: create_bone_influences_rs(py, &data.bone_influences),
    }
}

fn create_attribute_list_py(
    py: Python,
    attributes: &[ssbh_data::mesh_data::AttributeData],
) -> Py<PyList> {
    PyList::new(
        py,
        attributes
            .iter()
            .map(|a| Py::new(py, create_attribute_data_py(py, a)).unwrap())
            .collect::<Vec<Py<AttributeData>>>(),
    )
    .into()
}

// The Python library only uses a separate type to able to create a pyclass from it.
// TODO: Can this be shared with the original implementation?
fn create_mesh_object_py(
    py: Python,
    data: &ssbh_data::mesh_data::MeshObjectData,
) -> MeshObjectData {
    MeshObjectData {
        name: data.name.clone(),
        sub_index: data.sub_index,
        parent_bone_name: data.parent_bone_name.clone(),
        vertex_indices: data.vertex_indices.clone(),
        positions: create_attribute_list_py(py, &data.positions),
        normals: create_attribute_list_py(py, &data.normals),
        binormals: create_attribute_list_py(py, &data.binormals),
        tangents: create_attribute_list_py(py, &data.tangents),
        texture_coordinates: create_attribute_list_py(py, &data.texture_coordinates),
        color_sets: create_attribute_list_py(py, &data.color_sets),
        bone_influences: PyList::new(
            py,
            data.bone_influences
                .iter()
                .map(|i| Py::new(py, create_bone_influence(py, i)).unwrap())
                .collect::<Vec<Py<BoneInfluence>>>(),
        )
        .into(),
    }
}

fn create_bone_influence(_py: Python, i: &ssbh_data::mesh_data::BoneInfluence) -> BoneInfluence {
    BoneInfluence {
        bone_name: i.bone_name.clone(),
        vertex_weights: i
            .vertex_weights
            .iter()
            .map(|w| VertexWeight {
                vertex_index: w.vertex_index,
                vertex_weight: w.vertex_weight,
            })
            .collect(),
    }
}

fn create_attribute_data_py(
    py: Python,
    attribute_data: &ssbh_data::mesh_data::AttributeData,
) -> AttributeData {
    let data = match &attribute_data.data {
        ssbh_data::mesh_data::VectorData::Vector2(v) => {
            PyList::new(py, v.iter().map(|m| m.into_py(py))).into()
        }
        ssbh_data::mesh_data::VectorData::Vector3(v) => {
            PyList::new(py, v.iter().map(|m| m.into_py(py))).into()
        }
        ssbh_data::mesh_data::VectorData::Vector4(v) => {
            PyList::new(py, v.iter().map(|m| m.into_py(py))).into()
        }
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

fn create_attributes_rs(
    py: Python,
    attributes: &Py<PyList>,
) -> Vec<ssbh_data::mesh_data::AttributeData> {
    // Filter out objects of the wrong type.
    // TODO: Throw an error instead?
    attributes
        .as_ref(py)
        .iter()
        .filter_map(|a| a.extract::<AttributeData>().ok())
        .map(|a| create_attribute_rs(py, &a))
        .collect()
}

fn create_bone_influences_rs(
    py: Python,
    bone_influences: &Py<PyList>,
) -> Vec<ssbh_data::mesh_data::BoneInfluence> {
    bone_influences
        .as_ref(py)
        .iter()
        .filter_map(|i| i.extract::<BoneInfluence>().ok())
        .map(|i| create_bone_influence_rs(&i))
        .collect()
}

fn create_bone_influence_rs(
    influence: &BoneInfluence,
) -> ssbh_data::mesh_data::BoneInfluence {
    ssbh_data::mesh_data::BoneInfluence {
        bone_name: influence.bone_name.clone(),
        vertex_weights: influence
            .vertex_weights
            .iter()
            .map(|w| ssbh_data::mesh_data::VertexWeight {
                vertex_index: w.vertex_index,
                vertex_weight: w.vertex_weight,
            })
            .collect(),
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
                data: mesh,
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
    mesh_data.add_function(wrap_pyfunction!(read_mesh, mesh_data)?)?;

    module.add_submodule(mesh_data)?;
    Ok(())
}

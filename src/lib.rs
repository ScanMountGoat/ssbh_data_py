use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use ssbh_lib::SsbhFile;

#[pyclass]
struct Mesh {
    data: ssbh_lib::formats::mesh::Mesh,

    #[pyo3(get)]
    pub objects: Vec<MeshObjectData>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct MeshObjectData {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub sub_index: i64,

    #[pyo3(get)]
    pub parent_bone_name: String,

    #[pyo3(get)]
    pub vertex_indices: Vec<u32>,

    #[pyo3(get)]
    pub positions: AttributeDataVec3,

    #[pyo3(get)]
    pub normals: AttributeDataVec3,

    #[pyo3(get)]
    pub tangents: AttributeDataVec4,

    #[pyo3(get)]
    pub texture_coordinates: Vec<AttributeDataVec2>,

    #[pyo3(get)]
    pub color_sets: Vec<AttributeDataVec4>,
}

// Generics aren't allowed, so list the types explicitly.
#[pyclass]
#[derive(Debug, Clone)]
pub struct AttributeDataVec4 {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub data: Vec<[f32; 4]>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct AttributeDataVec2 {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub data: Vec<[f32; 2]>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct AttributeDataVec3 {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub data: Vec<[f32; 3]>,
}

// TODO: Use a macro for this?
impl From<ssbh_data::mesh_data::AttributeData<3>> for AttributeDataVec3 {
    fn from(v: ssbh_data::mesh_data::AttributeData<3>) -> Self {
        Self {
            name: v.name,
            data: v.data,
        }
    }
}

impl From<ssbh_data::mesh_data::AttributeData<4>> for AttributeDataVec4 {
    fn from(v: ssbh_data::mesh_data::AttributeData<4>) -> Self {
        Self {
            name: v.name,
            data: v.data,
        }
    }
}

impl From<ssbh_data::mesh_data::AttributeData<2>> for AttributeDataVec2 {
    fn from(v: ssbh_data::mesh_data::AttributeData<2>) -> Self {
        Self {
            name: v.name,
            data: v.data,
        }
    }
}

#[pyclass]
struct Skel {
    data: ssbh_lib::formats::skel::Skel,
}

#[pyclass]
struct Matl {
    data: ssbh_lib::formats::matl::Matl,
}

#[pyfunction]
fn read_mesh(path: &str) -> PyResult<Mesh> {
    // TODO: How to handle errors or return None?
    match ssbh_lib::read_ssbh(path).unwrap().data {
        SsbhFile::Mesh(mesh) => {
            let objects = ssbh_data::mesh_data::get_mesh_object_data(&mesh)
                .into_iter()
                .map(|m| MeshObjectData {
                    name: m.name,
                    sub_index: m.sub_index,
                    parent_bone_name: m.parent_bone_name,
                    vertex_indices: m.vertex_indices,
                    positions: m.positions.into(),
                    normals: m.normals.into(),
                    tangents: m.tangents.into(),
                    texture_coordinates: m
                        .texture_coordinates
                        .iter()
                        .map(|a| a.clone().into())
                        .collect(),
                    color_sets: m.color_sets.iter().map(|a| a.clone().into()).collect(),
                })
                .collect();
            Ok(Mesh {
                data: mesh,
                objects,
            })
        }
        _ => panic!("Failed to read mesh."),
    }
}

#[pyfunction]
fn read_skel(path: &str) -> PyResult<Skel> {
    // TODO: How to handle errors or return None?
    match ssbh_lib::read_ssbh(path).unwrap().data {
        SsbhFile::Skel(data) => Ok(Skel { data }),
        _ => panic!("Failed to read skel."),
    }
}

#[pymodule]
fn ssbh_data_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    m.add_class::<MeshObjectData>()?;
    m.add_class::<Skel>()?;

    m.add_function(wrap_pyfunction!(read_mesh, m)?)?;
    m.add_function(wrap_pyfunction!(read_skel, m)?)?;

    Ok(())
}

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use ssbh_data::mesh_data::AttributeData;
use ssbh_lib::SsbhFile;

#[pyclass]
struct Mesh {
    data: ssbh_lib::formats::mesh::Mesh,

    #[pyo3(get)]
    pub objects: Vec<MeshObjectData>,
}

#[pymethods]
impl Mesh {
    // TODO: What should this return type be?
    fn save(&mut self, path: &str) -> PyResult<()> {
        let objects: Vec<_> = self
            .objects
            .iter()
            .map(|o| ssbh_data::mesh_data::MeshObjectData::from(o))
            .collect();
        ssbh_data::mesh_data::update_mesh(&mut self.data, objects.as_slice()).unwrap();

        // TODO: add a write_mesh method?
        ssbh_lib::write_mesh_to_file(path, &self.data).unwrap();
        Ok(())
    }
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

// The Python library only uses a separate type to able to create a pyclass from it.
// TODO: Can this be shared with the original implementation?
impl From<&ssbh_data::mesh_data::MeshObjectData> for MeshObjectData {
    fn from(data: &ssbh_data::mesh_data::MeshObjectData) -> Self {
        Self {
            name: data.name.clone(),
            sub_index: data.sub_index,
            parent_bone_name: data.parent_bone_name.clone(),
            vertex_indices: data.vertex_indices.clone(),
            positions: data.positions.clone().into(),
            normals: data.normals.clone().into(),
            tangents: data.tangents.clone().into(),
            texture_coordinates: data
                .texture_coordinates
                .iter()
                .map(|a| a.clone().into())
                .collect(),
            color_sets: data.color_sets.iter().map(|a| a.clone().into()).collect(),
        }
    }
}

impl From<ssbh_data::mesh_data::MeshObjectData> for MeshObjectData {
    fn from(data: ssbh_data::mesh_data::MeshObjectData) -> Self {
        data.into()
    }
}

impl From<&MeshObjectData> for ssbh_data::mesh_data::MeshObjectData {
    fn from(data: &MeshObjectData) -> Self {
        Self {
            name: data.name.clone(),
            sub_index: data.sub_index,
            parent_bone_name: data.parent_bone_name.clone(),
            vertex_indices: data.vertex_indices.clone(),
            positions: data.positions.clone().into(),
            normals: data.normals.clone().into(),
            tangents: data.tangents.clone().into(),
            texture_coordinates: data
                .texture_coordinates
                .iter()
                .map(|a| a.clone().into())
                .collect(),
            color_sets: data.color_sets.iter().map(|a| a.clone().into()).collect(),
        }
    }
}

impl From<MeshObjectData> for ssbh_data::mesh_data::MeshObjectData {
    fn from(data: MeshObjectData) -> Self {
        data.into()
    }
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
macro_rules! attribute_data_impl {
    ($ty1:ident, $ty2:ident <$n:literal>) => {
        impl From<$ty1> for $ty2<$n> {
            fn from(v: $ty1) -> Self {
                Self {
                    name: v.name,
                    data: v.data,
                }
            }
        }

        impl From<$ty2<$n>> for $ty1 {
            fn from(v: $ty2<$n>) -> Self {
                Self {
                    name: v.name,
                    data: v.data,
                }
            }
        }
    };
}

attribute_data_impl!(AttributeDataVec2, AttributeData<2>);
attribute_data_impl!(AttributeDataVec3, AttributeData<3>);
attribute_data_impl!(AttributeDataVec4, AttributeData<4>);

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
            let objects: Vec<_> = ssbh_data::mesh_data::read_mesh_objects(&mesh)
                .unwrap()
                .iter()
                .map(|m| m.into())
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

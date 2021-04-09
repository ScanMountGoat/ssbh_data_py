use std::{borrow::Borrow, convert::TryInto};

use pyo3::wrap_pyfunction;
use pyo3::{prelude::*, types::PyList};
use ssbh_lib::SsbhFile;

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

        ssbh_lib::write_mesh_to_file(path, &self.data).unwrap();
        Ok(())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct MeshObjectData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub sub_index: i64,

    #[pyo3(get, set)]
    pub parent_bone_name: String,

    #[pyo3(get, set)]
    pub vertex_indices: Vec<u32>,

    #[pyo3(get, set)]
    pub positions: Py<AttributeData>,

    #[pyo3(get, set)]
    pub normals: Py<AttributeData>,

    #[pyo3(get, set)]
    pub tangents: Py<AttributeData>,

    #[pyo3(get, set)]
    pub texture_coordinates: Py<PyList>,

    #[pyo3(get, set)]
    pub color_sets: Py<PyList>,
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
        positions: create_attribute_rs_from_ref(py, &data.positions),
        normals: create_attribute_rs_from_ref(py, &data.normals),
        tangents: create_attribute_rs_from_ref(py, &data.tangents),
        texture_coordinates: create_attributes_rs(py, &data.texture_coordinates),
        color_sets: create_attributes_rs(py, &data.color_sets),
    }
}

// The Python library only uses a separate type to able to create a pyclass from it.
// TODO: Can this be shared with the original implementation?
fn create_mesh_object_py(
    py: Python,
    data: &ssbh_data::mesh_data::MeshObjectData,
) -> MeshObjectData {
    // TODO: This is truly horrifying...
    MeshObjectData {
        name: data.name.clone(),
        sub_index: data.sub_index,
        parent_bone_name: data.parent_bone_name.clone(),
        vertex_indices: data.vertex_indices.clone(),
        positions: Py::new(py, create_attribute_data_3(py, &data.positions)).unwrap(),
        normals: Py::new(py, create_attribute_data_4(py, &data.normals)).unwrap(),
        tangents: Py::new(py, create_attribute_data_4(py, &data.tangents)).unwrap(),
        texture_coordinates: PyList::new(
            py,
            data.texture_coordinates
                .iter()
                .map(|a| Py::new(py, create_attribute_data_2(py, a)).unwrap())
                .collect::<Vec<Py<AttributeData>>>(),
        )
        .into(),
        color_sets: PyList::new(
            py,
            data.color_sets
                .iter()
                .map(|a| Py::new(py, create_attribute_data_4(py, a)).unwrap())
                .collect::<Vec<Py<AttributeData>>>(),
        )
        .into(),
    }
}

// PyO3 doesn't seem to have const generics yet for converting [T;N] to Python types.
fn create_attribute_data_4(
    py: Python,
    attribute_data: &ssbh_data::mesh_data::AttributeData<4>,
) -> AttributeData {
    let data = PyList::new(py, attribute_data.data.iter().map(|m| m.into_py(py))).into();
    AttributeData {
        name: attribute_data.name.clone(),
        data,
    }
}

fn create_attribute_data_3(
    py: Python,
    attribute_data: &ssbh_data::mesh_data::AttributeData<3>,
) -> AttributeData {
    let data = PyList::new(py, attribute_data.data.iter().map(|m| m.into_py(py))).into();
    AttributeData {
        name: attribute_data.name.clone(),
        data,
    }
}

fn create_attribute_data_2(
    py: Python,
    attribute_data: &ssbh_data::mesh_data::AttributeData<2>,
) -> AttributeData {
    let data = PyList::new(py, attribute_data.data.iter().map(|m| m.into_py(py))).into();
    AttributeData {
        name: attribute_data.name.clone(),
        data,
    }
}

// Generics aren't allowed, so list the types explicitly.
#[pyclass]
#[derive(Debug, Clone)]
pub struct AttributeData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub data: Py<PyList>,
}

fn create_attribute_rs<const N: usize>(
    py: Python,
    attribute: &AttributeData,
) -> ssbh_data::mesh_data::AttributeData<N> {
    // Filter out objects of the wrong type.
    // TODO: Throw an error instead?
    // HACK: Convert to vec first to get around PyO3 not supporting arrays with length larger than 32.
    let data: Vec<_> = attribute
        .data
        .as_ref(py)
        .iter()
        .filter_map(|e| e.extract::<Vec<f32>>().ok())
        .filter_map(|e| e.try_into().ok())
        .collect();
    ssbh_data::mesh_data::AttributeData::<N> {
        name: attribute.name.clone(),
        data,
    }
}

fn create_attribute_rs_from_ref<const N: usize>(
    py: Python,
    attribute: &Py<AttributeData>,
) -> ssbh_data::mesh_data::AttributeData<N> {
    // Filter out objects of the wrong type.
    // TODO: Throw an error instead?
    // HACK: Convert to vec first to get around PyO3 not supporting arrays with length larger than 32.
    let attribute = &*attribute.as_ref(py).borrow();
    create_attribute_rs(py, &attribute)
}

fn create_attributes_rs<const N: usize>(
    py: Python,
    attributes: &Py<PyList>,
) -> Vec<ssbh_data::mesh_data::AttributeData<N>> {
    // Filter out objects of the wrong type.
    // TODO: Throw an error instead?
    attributes
        .as_ref(py)
        .iter()
        .filter_map(|a| a.extract::<AttributeData>().ok())
        .map(|a| create_attribute_rs(py, &a))
        .collect()
}

#[pyfunction]
fn read_mesh(py: Python, path: &str) -> PyResult<Mesh> {
    // TODO: How to handle errors or return None?
    match ssbh_lib::read_ssbh(path).unwrap().data {
        SsbhFile::Mesh(mesh) => {
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
fn ssbh_data_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    m.add_class::<MeshObjectData>()?;
    m.add_class::<AttributeData>()?;

    m.add_function(wrap_pyfunction!(read_mesh, m)?)?;

    Ok(())
}

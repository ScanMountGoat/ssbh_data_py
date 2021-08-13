use pyo3::wrap_pyfunction;
use pyo3::{prelude::*, types::PyList};

use crate::{create_py_list, create_py_list_from_slice, create_vec};

pub fn anim_data(py: Python, module: &PyModule) -> PyResult<()> {
    let anim_data = PyModule::new(py, "anim_data")?;
    anim_data.add_class::<AnimData>()?;
    anim_data.add_class::<GroupData>()?;
    anim_data.add_class::<NodeData>()?;
    anim_data.add_class::<TrackData>()?;
    anim_data.add_class::<Transform>()?;
    anim_data.add_class::<UvTransform>()?;

    anim_data.add_function(wrap_pyfunction!(read_anim, anim_data)?)?;

    module.add_submodule(anim_data)?;
    Ok(())
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct AnimData {
    // TODO: Support versions other than 2.0?
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    pub groups: Py<PyList>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct GroupData {
    #[pyo3(get, set)]
    pub group_type: u64, // TODO: Some sort of Python enum?

    #[pyo3(get, set)]
    pub nodes: Py<PyList>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct NodeData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub tracks: Py<PyList>,
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct TrackData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub values: Py<PyList>, // TODO: Is inferring the value type the best option?
}

#[pymethods]
impl AnimData {
    #[new]
    #[args(major_version = 2, minor_version = 0)]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(Self {
            major_version,
            minor_version,
            groups: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        // TODO: Support saving.
        Ok(())
    }
}

#[pyfunction]
fn read_anim(py: Python, path: &str) -> PyResult<AnimData> {
    match ssbh_data::anim_data::AnimData::from_file(path) {
        Ok(anim) => {
            let data = create_anim_data_py(py, &anim)?;
            Ok(data)
        }
        // TODO: How to handle errors or return None?
        _ => panic!("Failed to read anim."),
    }
}

fn create_anim_data_py(py: Python, data: &ssbh_data::anim_data::AnimData) -> PyResult<AnimData> {
    Ok(AnimData {
        major_version: data.major_version,
        minor_version: data.minor_version,
        groups: create_py_list(py, &data.groups, create_group_data_py)?,
    })
}

// TODO: Document what component counts are expected.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Transform {
    pub scale: Py<PyList>,
    pub rotation: Py<PyList>,
    pub translation: Py<PyList>,
    pub compensate_scale: f32,
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct UvTransform {
    pub unk1: f32,
    pub unk2: f32,
    pub unk3: f32,
    pub unk4: f32,
    pub unk5: f32,
}

fn create_uv_transform_py(
    _py: Python,
    transform: &ssbh_data::anim_data::UvTransform,
) -> PyResult<UvTransform> {
    Ok(UvTransform {
        unk1: transform.unk1,
        unk2: transform.unk2,
        unk3: transform.unk3,
        unk4: transform.unk4,
        unk5: transform.unk5,
    })
}

fn create_group_data_py(
    py: Python,
    group: &ssbh_data::anim_data::GroupData,
) -> PyResult<GroupData> {
    Ok(GroupData {
        group_type: group.group_type as u64,
        nodes: create_py_list(py, &group.nodes, create_node_data_py)?,
    })
}

// TODO: IntoIter and avoid clone?
fn create_node_data_py(
    py: Python,
    node: &ssbh_data::anim_data::NodeData,
) -> PyResult<NodeData> {
    Ok(NodeData {
        name: node.name.to_string(),
        tracks: create_py_list(py, &node.tracks, create_track_data_py)?,
    })
}

fn create_track_data_py(
    py: Python,
    track: &ssbh_data::anim_data::TrackData,
) -> PyResult<TrackData> {
    Ok(TrackData {
        name: track.name.to_string(),
        values: create_track_values_py(py, &track.values),
    })
}

fn create_transform_py(
    py: Python,
    transform: &ssbh_data::anim_data::Transform,
) -> PyResult<Transform> {
    Ok(Transform {
        scale: PyList::new(py, &[transform.scale.x, transform.scale.y, transform.scale.z]).into(),
        rotation: PyList::new(py, &[transform.rotation.x, transform.rotation.y, transform.rotation.z, transform.rotation.z]).into(),
        translation: PyList::new(py, &[transform.translation.x, transform.translation.y, transform.translation.z]).into(),
        compensate_scale: transform.compensate_scale,
    })
}

// TODO: Don't expose Vectors from ssbh_data?
fn vector4_values_to_py_list(
    py: Python,
    values: &[ssbh_data::anim_data::Vector4],
) -> Py<PyList> {
    let lists = values.iter().map(|v| PyList::new(py, &[v.x, v.y, v.z, v.w]));
    PyList::new(py, lists).into()
}


// TODO: Shared types for vectors and matrices?
// TODO: Return result.
fn create_track_values_py(py: Python, track_values: &ssbh_data::anim_data::TrackValues) -> Py<PyList> {
    match track_values {
        ssbh_data::anim_data::TrackValues::Transform(values) => create_py_list(py, values, create_transform_py).unwrap(),
        ssbh_data::anim_data::TrackValues::UvTransform(values) => create_py_list(py, values, create_uv_transform_py).unwrap(),
        ssbh_data::anim_data::TrackValues::Float(values) => create_py_list_from_slice(py, values),
        ssbh_data::anim_data::TrackValues::PatternIndex(values) => create_py_list_from_slice(py, values),
        ssbh_data::anim_data::TrackValues::Boolean(values) => create_py_list_from_slice(py, values),
        ssbh_data::anim_data::TrackValues::Vector4(values) => vector4_values_to_py_list(py, values),
    }
}

// 1. Convert the track data enum into Python lists.
// 2. Create the necessary node and group structure.

#[cfg(test)]
mod tests {
    use crate::run_python_code;
    use indoc::indoc;
}

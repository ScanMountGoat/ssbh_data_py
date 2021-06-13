use pyo3::wrap_pyfunction;
use pyo3::{prelude::*, types::PyList};

use crate::{create_py_list, create_py_list_from_slice, create_vec};

pub fn skel_data(py: Python, module: &PyModule) -> PyResult<()> {
    let skel_data = PyModule::new(py, "skel_data")?;
    skel_data.add_class::<SkelData>()?;
    skel_data.add_class::<BoneData>()?;
    skel_data.add_function(wrap_pyfunction!(read_skel, skel_data)?)?;
    skel_data.add_function(wrap_pyfunction!(calculate_relative_transform, skel_data)?)?;

    module.add_submodule(skel_data)?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone)]
struct SkelData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    pub bones: Py<PyList>,
}

#[pyclass]
#[derive(Debug, Clone)]
struct BoneData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub transform: Py<PyList>,

    #[pyo3(get, set)]
    pub parent_index: Option<usize>,
}

#[pymethods]
impl BoneData {
    #[new]
    fn new(
        py: Python,
        name: String,
        transform: [[f32; 4]; 4],
        parent_index: Option<usize>,
    ) -> PyResult<Self> {
        Ok(BoneData {
            name: name.clone(),
            transform: create_py_list_from_slice(py, &transform),
            parent_index,
        })
    }
}

#[pymethods]
impl SkelData {
    #[new]
    #[args(major_version = 1, minor_version = 0)]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(SkelData {
            major_version,
            minor_version,
            bones: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        let data = create_skel_data_rs(py, &self)?;
        data.write_to_file(path)?;
        Ok(())
    }

    fn calculate_world_transform(&self, py: Python, bone: &BoneData) -> PyResult<Py<PyList>> {
        let data = create_skel_data_rs(py, &self)?;
        let bone_data = create_bone_data_rs(py, &bone)?;
        let transform = data.calculate_world_transform(&bone_data);
        Ok(create_py_list_from_slice(py, &transform))
    }
}

#[pyfunction]
fn read_skel(py: Python, path: &str) -> PyResult<SkelData> {
    match ssbh_data::skel_data::SkelData::from_file(path) {
        Ok(skel) => {
            let data = create_skel_data_py(py, &skel)?;
            Ok(data)
        }
        // TODO: How to handle errors or return None?
        _ => panic!("Failed to read skel."),
    }
}

#[pyfunction]
fn calculate_relative_transform(
    py: Python,
    world_transform: &PyList,
    parent_world_transform: Option<&PyList>,
) -> PyResult<Py<PyList>> {
    // TODO: There might be a cleaner way to write this.
    let world_transform = world_transform.extract::<[[f32; 4]; 4]>()?;
    let transform = match parent_world_transform {
        Some(m) => {
            let parent_world_transform = m.extract::<[[f32; 4]; 4]>()?;
            ssbh_data::skel_data::calculate_relative_transform(
                &world_transform,
                Some(&parent_world_transform),
            )
        }
        None => ssbh_data::skel_data::calculate_relative_transform(&world_transform, None),
    };
    Ok(create_py_list_from_slice(py, &transform))
}

fn create_skel_data_py(py: Python, data: &ssbh_data::skel_data::SkelData) -> PyResult<SkelData> {
    Ok(SkelData {
        major_version: data.major_version,
        minor_version: data.minor_version,
        bones: create_py_list(py, &data.bones, create_bone_data_py)?,
    })
}

fn create_skel_data_rs(py: Python, data: &SkelData) -> PyResult<ssbh_data::skel_data::SkelData> {
    Ok(ssbh_data::skel_data::SkelData {
        major_version: data.major_version,
        minor_version: data.minor_version,
        bones: create_vec(py, &data.bones, create_bone_data_rs)?,
    })
}

fn create_bone_data_rs(py: Python, data: &BoneData) -> PyResult<ssbh_data::skel_data::BoneData> {
    Ok(ssbh_data::skel_data::BoneData {
        name: data.name.clone(),
        transform: data.transform.extract::<[[f32; 4]; 4]>(py)?,
        parent_index: data.parent_index,
    })
}

fn create_bone_data_py(py: Python, data: &ssbh_data::skel_data::BoneData) -> PyResult<BoneData> {
    Ok(BoneData {
        name: data.name.clone(),
        transform: create_py_list_from_slice(py, &data.transform),
        parent_index: data.parent_index,
    })
}

#[cfg(test)]
mod tests {
    use pyo3::prelude::*;
    use pyo3::types::IntoPyDict;

    use indoc::indoc;

    use crate::ssbh_data_py;

    #[test]
    fn create_skel() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
                s = ssbh_data_py.skel_data.SkelData()
                assert s.major_version == 1
                assert s.minor_version == 0
                assert s.bones == []
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }

    #[test]
    fn create_bone_data() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
                b = ssbh_data_py.skel_data.BoneData("abc", [[0,0,0,0]]*4, 5)
                assert b.name == "abc"
                assert b.transform == [[0,0,0,0]]*4
                assert b.parent_index == 5

                b = ssbh_data_py.skel_data.BoneData("abc", [[1,1,1,1]]*4, None)
                assert b.name == "abc"
                assert b.transform == [[1,1,1,1]]*4
                assert b.parent_index == None
                b.transform[1][2] = 3
                assert b.transform[1] == [1,1,3,1]
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }

    #[test]
    fn calculate_relative_transform_with_parent() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
                world_transform = [
                    [2, 0, 0, 0],
                    [0, 4, 0, 0],
                    [0, 0, 8, 0],
                    [0, 0, 0, 1],
                ]
                parent_world_transform = [
                    [1, 0, 0, 0],
                    [0, 1, 0, 0],
                    [0, 0, 1, 0],
                    [1, 2, 3, 1],
                ]
                relative_transform = [
                    [2.0, 0, 0, 0],
                    [0, 4, 0, 0],
                    [0, 0, 8, 0],
                    [-2, -8, -24, 1],
                ]
                assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, parent_world_transform) == relative_transform
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }

    
    #[test]
    fn calculate_relative_transform_no_parent() {
        let gil = Python::acquire_gil();
        let py = gil.python();

        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(
            indoc! {r#"
                world_transform = [
                    [0, 1, 2, 3],
                    [4, 5, 6, 7],
                    [8, 9, 10, 11],
                    [12, 13, 14, 15],
                ]
                assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None) == world_transform
            "#},
            None,
            Some(&ctx),
        )
        .unwrap();
    }
}

use pyo3::wrap_pyfunction;
use pyo3::{prelude::*, types::PyList};

use crate::{create_py_list, create_py_list_from_slice};

pub fn skel_data(py: Python, module: &PyModule) -> PyResult<()> {
    let skel_data = PyModule::new(py, "skel_data")?;
    skel_data.add_class::<SkelData>()?;
    skel_data.add_class::<BoneData>()?;
    skel_data.add_function(wrap_pyfunction!(read_skel, skel_data)?)?;

    module.add_submodule(skel_data)?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone)]
struct SkelData {
    #[pyo3(get, set)]
    pub bones: Py<PyList>,
}

#[pyclass]
#[derive(Debug, Clone)]
struct BoneData {
    #[pyo3(get, set)]
    pub name: String,

    // TODO: [[f32; 4]; 4]
    #[pyo3(get, set)]
    pub transform: Py<PyList>,

    #[pyo3(get, set)]
    pub parent_index: Option<usize>,
}


#[pymethods]
impl BoneData {
    #[new]
    fn new(py: Python, name: String, transform: [[f32; 4]; 4], parent_index: Option<usize>) -> PyResult<Self> {
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
    #[args(major_version = 1, minor_version = 7)]
    fn new(py: Python) -> PyResult<Self> {
        Ok(SkelData {
            bones: PyList::empty(py).into()
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        // TODO: Convert to ssbh_data and then save.
        Ok(())
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
        _ => panic!("Failed to read modl."),
    }
}


fn create_skel_data_py(py: Python, data: &ssbh_data::skel_data::SkelData) -> PyResult<SkelData> {
    Ok(SkelData {
        bones: create_py_list(py, &data.bones, create_bone_data_py)?,
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
}

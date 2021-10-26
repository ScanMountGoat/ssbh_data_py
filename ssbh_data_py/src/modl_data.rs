use crate::MapPy;
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data::SsbhData;
use ssbh_data_py_derive::MapPy;

create_exception!(ssbh_data_py, ModlDataError, pyo3::exceptions::PyException);

pub fn modl_data(py: Python, module: &PyModule) -> PyResult<()> {
    let modl_data = PyModule::new(py, "modl_data")?;
    modl_data.add_class::<ModlData>()?;
    modl_data.add_class::<ModlEntryData>()?;
    modl_data.add_function(wrap_pyfunction!(read_modl, modl_data)?)?;

    module.add_submodule(modl_data)?;
    Ok(())
}

#[pyclass]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::modl_data::ModlData)]
struct ModlData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    pub model_name: String,

    #[pyo3(get, set)]
    pub skeleton_file_name: String,

    #[pyo3(get, set)]
    pub material_file_names: Py<PyList>,

    #[pyo3(get, set)]
    pub animation_file_name: Option<String>,

    #[pyo3(get, set)]
    pub mesh_file_name: String,

    #[pyo3(get, set)]
    pub entries: Py<PyList>,
}

#[pymethods]
impl ModlData {
    #[new]
    #[args(major_version = 1, minor_version = 7)]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(ModlData {
            major_version,
            minor_version,
            model_name: "".into(),
            skeleton_file_name: "".into(),
            material_file_names: PyList::empty(py).into(),
            animation_file_name: None,
            mesh_file_name: "".into(),
            entries: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py)?.write_to_file(path).map_err(PyErr::from)
    }
}

#[pyclass]
#[derive(Debug, Clone, MapPy)]
#[map(ssbh_data::modl_data::ModlEntryData)]
pub struct ModlEntryData {
    #[pyo3(get, set)]
    pub mesh_object_name: String,

    #[pyo3(get, set)]
    pub mesh_object_sub_index: u64,

    #[pyo3(get, set)]
    pub material_label: String,
}

#[pymethods]
impl ModlEntryData {
    #[new]
    fn new(
        _py: Python,
        mesh_object_name: String,
        mesh_object_sub_index: u64,
        material_label: String,
    ) -> PyResult<Self> {
        Ok(ModlEntryData {
            mesh_object_name,
            mesh_object_sub_index,
            material_label,
        })
    }
}

#[pyfunction]
fn read_modl(py: Python, path: &str) -> PyResult<ModlData> {
    ssbh_data::modl_data::ModlData::from_file(path)
        .map_err(|e| ModlDataError::new_err(format!("{}", e)))?
        .map_py(py)
}

#[cfg(test)]
mod tests {
    use crate::run_python_code;
    use indoc::indoc;

    #[test]
    fn create_modl() {
        run_python_code(indoc! {r#"
            m = ssbh_data_py.modl_data.ModlData(3, 4)
            assert m.major_version == 3
            assert m.minor_version == 4
            assert m.model_name == ""
            assert m.skeleton_file_name == ""
            assert m.material_file_names == []
            assert m.animation_file_name == None
            assert m.mesh_file_name == ""
            assert m.entries == []

            m = ssbh_data_py.modl_data.ModlData(3)
            assert m.major_version == 3
            assert m.minor_version == 7

            m = ssbh_data_py.modl_data.ModlData()
            assert m.major_version == 1
            assert m.minor_version == 7
        "#})
        .unwrap();
    }

    #[test]
    fn create_modl_entry() {
        run_python_code(indoc! {r#"
            m = ssbh_data_py.modl_data.ModlEntryData("a", 7, "b")
            assert m.mesh_object_name == "a"
            assert m.mesh_object_sub_index == 7
            assert m.material_label == "b"
        "#})
        .unwrap();
    }
}

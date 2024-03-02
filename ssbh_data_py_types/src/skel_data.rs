use crate::{python_enum, MapPy, PyInit, PyRepr, Pyi, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};

use crate::create_py_list_from_slice;

mod enums;

create_exception!(ssbh_data_py, SkelDataError, pyo3::exceptions::PyException);

pub fn skel_data(py: Python, module: &PyModule) -> PyResult<()> {
    let skel_data = PyModule::new(py, "skel_data")?;
    skel_data.add_class::<SkelData>()?;
    skel_data.add_class::<BoneData>()?;
    skel_data.add_class::<BillboardType>()?;

    skel_data.add_function(wrap_pyfunction!(read_skel, skel_data)?)?;
    skel_data.add_function(wrap_pyfunction!(calculate_relative_transform, skel_data)?)?;

    module.add_submodule(skel_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.skel_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::skel_data::SkelData)]
#[pyrepr("ssbh_data_py.skel_data")]
#[pyi(has_methods = true)]
pub struct SkelData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[BoneData]")]
    pub bones: Py<PyList>,
}

#[pyclass(module = "ssbh_data_py.skel_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::skel_data::BoneData)]
#[pyrepr("ssbh_data_py.skel_data")]
pub struct BoneData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[list[float]]")]
    pub transform: PyObject,

    #[pyo3(get, set)]
    #[pyinit(default = "None")]
    #[pyi(default = "None")]
    pub parent_index: Option<usize>,

    #[pyo3(get, set)]
    #[pyinit(default = "ssbh_data::skel_data::BillboardType::Disabled.into()")]
    #[pyi(default = "BillboardType.Disabled")]
    pub billboard_type: BillboardType,
}

impl PyiMethods for SkelData {
    fn pyi_methods() -> String {
        r#"    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 0,
    ) -> None: ...

    def save(self, path: str) -> None: ...

    def calculate_world_transform(
        self, bone: BoneData) -> list[list[float]]: ..."#
            .to_string()
    }
}

#[pymethods]
impl SkelData {
    #[new]
    #[pyo3(signature = (major_version = 1, minor_version = 0))]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(SkelData {
            major_version,
            minor_version,
            bones: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py, false)?
            .write_to_file(path)
            .map_err(|e| SkelDataError::new_err(format!("{}", e)))
    }

    fn calculate_world_transform(&self, py: Python, bone: &BoneData) -> PyResult<Py<PyList>> {
        let data: ssbh_data::skel_data::SkelData = self.map_py(py, false)?;
        let bone_data: ssbh_data::skel_data::BoneData = bone.map_py(py, false)?;
        let transform = data
            .calculate_world_transform(&bone_data)
            .map_err(|e| SkelDataError::new_err(format!("{}", e)))?;
        Ok(create_py_list_from_slice(py, &transform))
    }
}

python_enum!(
    BillboardType,
    ssbh_data::skel_data::BillboardType,
    SkelDataError,
    "ssbh_data_py.skel_data"
);

#[pyfunction]
fn read_skel(py: Python, path: &str) -> PyResult<SkelData> {
    ssbh_data::skel_data::SkelData::from_file(path)
        .map_err(|e| SkelDataError::new_err(format!("{}", e)))?
        .map_py(py, false)
}

#[pyfunction]
fn calculate_relative_transform(
    py: Python,
    world_transform: PyObject,
    parent_world_transform: Option<PyObject>,
) -> PyResult<Py<PyList>> {
    let world_transform = world_transform.map_py(py, false)?;
    let transform = match parent_world_transform {
        Some(m) => ssbh_data::skel_data::calculate_relative_transform(
            &world_transform,
            Some(&m.map_py(py, false)?),
        ),
        None => ssbh_data::skel_data::calculate_relative_transform(&world_transform, None),
    };
    Ok(create_py_list_from_slice(py, &transform))
}

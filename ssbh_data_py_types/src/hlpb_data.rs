use pyo3::{create_exception, prelude::*};

create_exception!(ssbh_data_py, HlpbDataError, pyo3::exceptions::PyException);

#[pymodule]
pub mod hlpb_data {
    pub use super::*;

    use crate::{MapPy, PyInit, PyRepr, Pyi, PyiMethods};
    use pyo3::types::PyList;

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::hlpb_data::HlpbData)]
    #[pyrepr("ssbh_data_py.hlpb_data")]
    #[pyi(has_methods = true)]
    pub struct HlpbData {
        pub major_version: u16,

        pub minor_version: u16,

        #[pyi(python_type = "list[AimConstraintData]")]
        pub aim_constraints: Py<PyList>,

        #[pyi(python_type = "list[OrientConstraintData]")]
        pub orient_constraints: Py<PyList>,
    }

    #[pymethods]
    impl HlpbData {
        #[new]
        #[pyo3(signature = (major_version = 1, minor_version = 0))]
        fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
            Ok(HlpbData {
                major_version,
                minor_version,
                aim_constraints: PyList::empty(py).into(),
                orient_constraints: PyList::empty(py).into(),
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.map_py(py)?.write_to_file(path).map_err(PyErr::from)
        }

        fn __repr__(&self) -> String {
            self.py_repr()
        }
    }

    // TODO: Can we document the actual default value here?
    // Add the default to some sort of derive attribute?
    impl PyiMethods for HlpbData {
        fn pyi_methods() -> String {
            r#"    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 0,
    ) -> None: ...
    
    def save(self, path: str) -> None: ..."#
                .to_string()
        }
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::hlpb_data::AimConstraintData)]
    #[pyrepr("ssbh_data_py.hlpb_data")]
    pub struct AimConstraintData {
        pub name: String,

        pub aim_bone_name1: String,

        pub aim_bone_name2: String,

        pub target_bone_name1: String,

        pub target_bone_name2: String,

        #[pyinit(default = "\"DEFAULT\".into()")]
        #[pyi(default = "'Default'")]
        pub aim_type1: String,

        #[pyinit(default = "\"DEFAULT\".into()")]
        #[pyi(default = "'Default'")]
        pub aim_type2: String,

        #[pyinit(default = "0")]
        #[pyi(default = "0")]
        pub unk1: u32,

        #[pyinit(default = "0")]
        #[pyi(default = "0")]
        pub unk2: u32,

        #[pyinit(default = "PyList::new(py, [1.0, 0.0, 0.0])?.into()")]
        #[pyi(python_type = "list[float]", default = "[1.0, 0.0, 0.0]")]
        pub aim: Py<PyList>,

        #[pyinit(default = "PyList::new(py, [0.0, 1.0, 0.0])?.into()")]
        #[pyi(python_type = "list[float]", default = "[0.0, 1.0, 0.0]")]
        pub up: Py<PyList>,

        #[pyinit(default = "PyList::new(py, [0.0, 0.0, 0.0, 1.0])?.into()")]
        #[pyi(python_type = "list[float]", default = "[0.0, 0.0, 0.0, 1.0]")]
        pub quat1: Py<PyList>,

        #[pyinit(default = "PyList::new(py, [0.0, 0.0, 0.0, 1.0])?.into()")]
        #[pyi(python_type = "list[float]", default = "[0.0, 0.0, 0.0, 1.0]")]
        pub quat2: Py<PyList>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::hlpb_data::OrientConstraintData)]
    #[pyrepr("ssbh_data_py.hlpb_data")]
    pub struct OrientConstraintData {
        pub name: String,

        pub parent_bone_name1: String,

        pub parent_bone_name2: String,

        pub source_bone_name: String,

        pub target_bone_name: String,

        pub unk_type: u32,

        #[pyi(python_type = "list[float]")]
        pub constraint_axes: Py<PyList>,

        #[pyinit(default = "PyList::new(py, [0.0, 0.0, 0.0, 1.0])?.into()")]
        #[pyi(python_type = "list[float]", default = "[0.0, 0.0, 0.0, 1.0]")]
        pub quat1: Py<PyList>,

        #[pyinit(default = "PyList::new(py, [0.0, 0.0, 0.0, 1.0])?.into()")]
        #[pyi(python_type = "list[float]", default = "[0.0, 0.0, 0.0, 1.0]")]
        pub quat2: Py<PyList>,

        #[pyinit(default = "PyList::new(py, [-180.0, -180.0, -180.0])?.into()")]
        #[pyi(python_type = "list[float]", default = "[-180.0, -180.0, -180.0]")]
        pub range_min: Py<PyList>,

        #[pyinit(default = "PyList::new(py, [180.0, 180.0, 180.0])?.into()")]
        #[pyi(python_type = "list[float]", default = "[180.0, 180.0, 180.0]")]
        pub range_max: Py<PyList>,
    }

    #[pyfunction]
    fn read_hlpb(py: Python, path: &str) -> PyResult<HlpbData> {
        ssbh_data::hlpb_data::HlpbData::from_file(path)
            .map_err(|e| HlpbDataError::new_err(format!("{}", e)))?
            .map_py(py)
    }
}

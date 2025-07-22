use pyo3::{create_exception, prelude::*};

create_exception!(ssbh_data_py, HlpbDataError, pyo3::exceptions::PyException);

#[pymodule]
pub mod hlpb_data {
    pub use super::*;

    use crate::{
        map_from_vector3, map_from_vector4, map_into_vector3, map_into_vector4, PyInit, PyRepr,
        Pyi, PyiMethods,
    };
    use map_py::{MapPy, TypedList};

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::hlpb_data::HlpbData)]
    #[pyrepr("ssbh_data_py.hlpb_data")]
    #[pyi(has_methods = true)]
    pub struct HlpbData {
        pub major_version: u16,
        pub minor_version: u16,
        pub aim_constraints: TypedList<AimConstraintData>,
        pub orient_constraints: TypedList<OrientConstraintData>,
    }

    #[pymethods]
    impl HlpbData {
        #[new]
        #[pyo3(signature = (major_version = 1, minor_version = 0))]
        fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
            Ok(HlpbData {
                major_version,
                minor_version,
                aim_constraints: TypedList::empty(py),
                orient_constraints: TypedList::empty(py),
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.clone()
                .map_py(py)?
                .write_to_file(path)
                .map_err(PyErr::from)
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

        #[pyinit(default = "vec![1.0, 0.0, 0.0].map_py(py)?")]
        #[pyi(default = "[1.0, 0.0, 0.0]")]
        #[map(from(map_from_vector3), into(map_into_vector3))]
        pub aim: TypedList<f32>,

        #[pyinit(default = "vec![0.0, 1.0, 0.0].map_py(py)?")]
        #[pyi(default = "[0.0, 1.0, 0.0]")]
        #[map(from(map_from_vector3), into(map_into_vector3))]
        pub up: TypedList<f32>,

        #[pyinit(default = "vec![0.0, 0.0, 0.0, 1.0].map_py(py)?")]
        #[pyi(default = "[0.0, 0.0, 0.0, 1.0]")]
        #[map(from(map_from_vector4), into(map_into_vector4))]
        pub quat1: TypedList<f32>,

        #[pyinit(default = "vec![0.0, 0.0, 0.0, 1.0].map_py(py)?")]
        #[pyi(default = "[0.0, 0.0, 0.0, 1.0]")]
        #[map(from(map_from_vector4), into(map_into_vector4))]
        pub quat2: TypedList<f32>,
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

        #[map(from(map_from_vector3), into(map_into_vector3))]
        pub constraint_axes: TypedList<f32>,

        #[pyinit(default = "vec![0.0, 0.0, 0.0, 1.0].map_py(py)?")]
        #[pyi(default = "[0.0, 0.0, 0.0, 1.0]")]
        #[map(from(map_from_vector4), into(map_into_vector4))]
        pub quat1: TypedList<f32>,

        #[pyinit(default = "vec![0.0, 0.0, 0.0, 1.0].map_py(py)?")]
        #[pyi(default = "[0.0, 0.0, 0.0, 1.0]")]
        #[map(from(map_from_vector4), into(map_into_vector4))]
        pub quat2: TypedList<f32>,

        #[pyinit(default = "vec![-180.0, -180.0, -180.0].map_py(py)?")]
        #[pyi(default = "[-180.0, -180.0, -180.0]")]
        #[map(from(map_from_vector3), into(map_into_vector3))]
        pub range_min: TypedList<f32>,

        #[pyinit(default = "vec![180.0, 180.0, 180.0].map_py(py)?")]
        #[pyi(default = "[180.0, 180.0, 180.0]")]
        #[map(from(map_from_vector3), into(map_into_vector3))]
        pub range_max: TypedList<f32>,
    }

    #[pyfunction]
    fn read_hlpb(py: Python, path: &str) -> PyResult<HlpbData> {
        ssbh_data::hlpb_data::HlpbData::from_file(path)
            .map_err(|e| HlpbDataError::new_err(format!("{}", e)))?
            .map_py(py)
    }
}

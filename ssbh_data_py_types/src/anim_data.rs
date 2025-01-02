use crate::python_enum;
use pyo3::{create_exception, prelude::*};

create_exception!(ssbh_data_py, AnimDataError, pyo3::exceptions::PyException);

python_enum!(
    GroupType,
    ssbh_data::anim_data::GroupType,
    AnimDataError,
    "ssbh_data_py.anim_data",
    Transform,
    Visibility,
    Material,
    Camera
);

#[pymodule]
pub mod anim_data {
    pub use super::*;

    use crate::{MapPy, PyInit, PyRepr, Pyi, PyiMethods};
    use pyo3::types::PyList;
    use ssbh_data::anim_data::TrackValues as TrackValuesRs;

    #[pymodule_export]
    pub use super::GroupType;

    #[pyclass]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::anim_data::AnimData)]
    #[pyrepr("ssbh_data_py.anim_data")]
    #[pyi(has_methods = true)]
    pub struct AnimData {
        #[pyo3(get, set)]
        pub major_version: u16,

        #[pyo3(get, set)]
        pub minor_version: u16,

        #[pyo3(get, set)]
        #[pyi(python_type = "list[GroupData]")]
        pub groups: Py<PyList>,

        #[pyo3(get, set)]
        pub final_frame_index: f32,
    }

    #[pyclass]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::GroupData)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct GroupData {
        #[pyo3(get, set)]
        pub group_type: GroupType,

        #[pyo3(get, set)]
        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[NodeData]")]
        pub nodes: Py<PyList>,
    }

    #[pyclass]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::NodeData)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct NodeData {
        #[pyo3(get, set)]
        pub name: String,

        #[pyo3(get, set)]
        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(default = "[]", python_type = "list[TrackData]")]
        pub tracks: Py<PyList>,
    }

    #[pyclass]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::TrackData)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct TrackData {
        #[pyo3(get, set)]
        pub name: String,

        #[pyo3(get, set)]
        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub compensate_scale: bool,

        #[pyo3(get, set)]
        #[pyinit(
            default = "TransformFlags { override_translation: false, override_rotation: false, override_scale: false, override_compensate_scale: false}"
        )]
        #[pyi(default = "TransformFlags()")]
        pub transform_flags: TransformFlags,

        // TODO: Does it make sense to use numpy here?
        #[pyo3(get, set)]
        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(
            default = "[]",
            python_type = "Union[list[UvTransform], list[Transform],
                      list[float], list[bool], list[int], list[list[float]]]"
        )]
        pub values: Py<PyList>,
    }

    #[pyclass]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::TransformFlags)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct TransformFlags {
        #[pyo3(get, set)]
        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub override_translation: bool,

        #[pyo3(get, set)]
        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub override_rotation: bool,

        #[pyo3(get, set)]
        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub override_scale: bool,

        #[pyo3(get, set)]
        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub override_compensate_scale: bool,
    }

    #[pymethods]
    impl AnimData {
        #[new]
        #[pyo3(signature = (major_version = 2, minor_version = 0))]
        fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
            Ok(Self {
                major_version,
                minor_version,
                groups: PyList::empty(py).into(),
                final_frame_index: 0.0,
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.map_py(py)?
                .write_to_file(path)
                .map_err(|e| AnimDataError::new_err(format!("{}", e)))
        }

        fn __repr__(&self) -> String {
            self.py_repr()
        }
    }

    impl PyiMethods for AnimData {
        fn pyi_methods() -> String {
            r#"    def __init__(
        self,
        major_version: int = 2,
        minor_version: int = 0,
    ) -> None: ...

    def save(self, path: str) -> None: ..."#
                .to_string()
        }
    }

    #[pyfunction]
    fn read_anim(py: Python, path: &str) -> PyResult<AnimData> {
        ssbh_data::anim_data::AnimData::from_file(path)
            .map_err(|e| AnimDataError::new_err(format!("{}", e)))?
            .map_py(py)
    }

    // TODO: Document what component counts are expected.
    #[pyclass]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::Transform)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct Transform {
        #[pyo3(get, set)]
        #[pyi(python_type = "list[float]")]
        pub scale: Py<PyList>,

        #[pyo3(get, set)]
        #[pyi(python_type = "list[float]")]
        pub rotation: Py<PyList>,

        #[pyo3(get, set)]
        #[pyi(python_type = "list[float]")]
        pub translation: Py<PyList>,
    }

    #[pyclass]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::UvTransform)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct UvTransform {
        #[pyo3(get, set)]
        pub scale_u: f32,

        #[pyo3(get, set)]
        pub scale_v: f32,

        #[pyo3(get, set)]
        pub rotation: f32,

        #[pyo3(get, set)]
        pub translate_u: f32,

        #[pyo3(get, set)]
        pub translate_v: f32,
    }

    impl MapPy<Py<PyList>> for TrackValuesRs {
        fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
            match self {
                TrackValuesRs::Transform(v) => v.map_py(py),
                TrackValuesRs::UvTransform(v) => v.map_py(py),
                TrackValuesRs::Float(v) => v.map_py(py),
                TrackValuesRs::PatternIndex(v) => v.map_py(py),
                TrackValuesRs::Boolean(v) => v.map_py(py),
                TrackValuesRs::Vector4(v) => v.map_py(py),
            }
        }
    }

    impl MapPy<TrackValuesRs> for Py<PyList> {
        fn map_py(&self, py: Python) -> PyResult<TrackValuesRs> {
            create_track_values_rs(py, self)
        }
    }

    pub fn create_track_values_rs(py: Python, values: &Py<PyList>) -> PyResult<TrackValuesRs> {
        // We don't know the type, so just try one until it works.
        // TODO: Clean up this code.
        values
            .extract::<Vec<bool>>(py)
            .map(TrackValuesRs::Boolean)
            .or_else(|_| {
                // Pattern index needs to come before float.
                // This avoids conflicts with integer literals being interpreted as floats.
                values
                    .extract::<Vec<u32>>(py)
                    .map(TrackValuesRs::PatternIndex)
            })
            .or_else(|_| values.extract::<Vec<f32>>(py).map(TrackValuesRs::Float))
            .or_else(|_| {
                values.extract::<Vec<[f32; 4]>>(py).map(|v| {
                    TrackValuesRs::Vector4(v.into_iter().map(ssbh_data::Vector4::from).collect())
                })
            })
            .or_else(|_| {
                let v = values.extract::<Vec<UvTransform>>(py)?;
                Ok(TrackValuesRs::UvTransform(
                    v.into_iter()
                        .map(|t| t.map_py(py))
                        .collect::<Result<Vec<_>, _>>()?,
                ))
            })
            .or_else(|_: PyErr| {
                let v = values.extract::<Vec<Transform>>(py)?;
                Ok(TrackValuesRs::Transform(
                    v.into_iter()
                        .map(|t| t.map_py(py))
                        .collect::<Result<Vec<_>, _>>()?,
                ))
            })
    }
}

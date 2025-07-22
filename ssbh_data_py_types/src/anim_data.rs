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

    use crate::{
        map_from_vector3, map_from_vector4, map_into_vector3, map_into_vector4, PyInit, PyRepr,
        Pyi, PyiMethods,
    };
    use map_py::{map_vec, MapPy, TypedList};
    use pyo3::types::PyList;
    use ssbh_data::anim_data::TrackValues as TrackValuesRs;

    #[pymodule_export]
    pub use super::GroupType;

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::anim_data::AnimData)]
    #[pyrepr("ssbh_data_py.anim_data")]
    #[pyi(has_methods = true)]
    pub struct AnimData {
        pub major_version: u16,
        pub minor_version: u16,
        pub groups: TypedList<GroupData>,
        pub final_frame_index: f32,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::GroupData)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct GroupData {
        pub group_type: GroupType,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub nodes: TypedList<NodeData>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::NodeData)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct NodeData {
        pub name: String,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub tracks: TypedList<TrackData>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::TrackData)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct TrackData {
        pub name: String,

        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub compensate_scale: bool,

        #[pyinit(
            default = "TransformFlags { override_translation: false, override_rotation: false, override_scale: false, override_compensate_scale: false}"
        )]
        #[pyi(default = "TransformFlags()")]
        pub transform_flags: TransformFlags,

        #[pyinit(default = "PyList::empty(py).into()")]
        #[pyi(
            default = "[]",
            python_type = "Union[list[UvTransform], list[Transform],
                      list[float], list[bool], list[int], list[list[float]]]"
        )]
        #[map(from(map_from_track_values), into(map_into_track_values))]
        pub values: Py<PyList>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::TransformFlags)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct TransformFlags {
        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub override_translation: bool,

        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub override_rotation: bool,

        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub override_scale: bool,

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
                groups: TypedList::empty(py),
                final_frame_index: 0.0,
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.clone()
                .map_py(py)?
                .write_to_file(path)
                .map_err(|e| AnimDataError::new_err(format!("{e}")))
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
            .map_err(|e| AnimDataError::new_err(format!("{e}")))?
            .map_py(py)
    }

    // TODO: Document what component counts are expected.
    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::Transform)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct Transform {
        #[map(from(map_from_vector3), into(map_into_vector3))]
        pub scale: TypedList<f32>,

        #[map(from(map_from_vector4), into(map_into_vector4))]
        pub rotation: TypedList<f32>,

        #[map(from(map_from_vector3), into(map_into_vector3))]
        pub translation: TypedList<f32>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::anim_data::UvTransform)]
    #[pyrepr("ssbh_data_py.anim_data")]
    pub struct UvTransform {
        pub scale_u: f32,
        pub scale_v: f32,
        pub rotation: f32,
        pub translate_u: f32,
        pub translate_v: f32,
    }

    fn map_from_track_values(value: TrackValuesRs, py: Python) -> PyResult<Py<PyList>> {
        match value {
            TrackValuesRs::Transform(v) => map_vec(v, py),
            TrackValuesRs::UvTransform(v) => map_vec(v, py),
            TrackValuesRs::Float(v) => map_vec(v, py),
            TrackValuesRs::PatternIndex(v) => map_vec(v, py),
            TrackValuesRs::Boolean(v) => map_vec(v, py),
            TrackValuesRs::Vector4(v) => {
                PyList::new(
                    py,
                    v.into_iter()
                        .map(|v| {
                            let u = map_from_vector4(v, py)?;
                            // TODO: avoid unwrap.
                            Ok(u.into_pyobject(py).unwrap())
                        })
                        .collect::<PyResult<Vec<_>>>()?,
                )
                .map(Into::into)
            }
        }
    }

    fn map_into_track_values(value: Py<PyList>, py: Python) -> PyResult<TrackValuesRs> {
        create_track_values_rs(py, &value)
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

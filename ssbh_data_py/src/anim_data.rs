use crate::{python_enum, MapPy, PyRepr, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data::anim_data::TrackValues as TrackValuesRs;
use ssbh_data::SsbhData;
use ssbh_data_py_derive::{MapPy, PyRepr, Pyi};

mod enums;

create_exception!(ssbh_data_py, AnimDataError, pyo3::exceptions::PyException);

pub fn anim_data(py: Python, module: &PyModule) -> PyResult<()> {
    let anim_data = PyModule::new(py, "anim_data")?;
    anim_data.add_class::<AnimData>()?;
    anim_data.add_class::<GroupData>()?;
    anim_data.add_class::<NodeData>()?;
    anim_data.add_class::<TrackData>()?;
    anim_data.add_class::<Transform>()?;
    anim_data.add_class::<UvTransform>()?;
    anim_data.add_class::<GroupType>()?;
    anim_data.add_class::<ScaleOptions>()?;

    anim_data.add_function(wrap_pyfunction!(read_anim, anim_data)?)?;

    module.add_submodule(anim_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.anim_data")]
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

#[pyclass(module = "ssbh_data_py.anim_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::anim_data::GroupData)]
#[pyrepr("ssbh_data_py.anim_data")]
#[pyi(has_methods = true)]
pub struct GroupData {
    #[pyo3(get, set)]
    pub group_type: GroupType,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[NodeData]")]
    pub nodes: Py<PyList>,
}

#[pymethods]
impl GroupData {
    #[new]
    fn new(py: Python, group_type: GroupType) -> PyResult<Self> {
        Ok(GroupData {
            group_type,
            nodes: PyList::empty(py).into(),
        })
    }
}

impl PyiMethods for GroupData {
    fn pyi_methods() -> String {
        "    def __init__(
        self,
        group_type: GroupType,
    ) -> None: ..."
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.anim_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::anim_data::NodeData)]
#[pyrepr("ssbh_data_py.anim_data")]
#[pyi(has_methods = true)]
pub struct NodeData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[TrackData]")]
    pub tracks: Py<PyList>,
}

#[pymethods]
impl NodeData {
    #[new]
    fn new(py: Python, name: String) -> PyResult<Self> {
        Ok(NodeData {
            name,
            tracks: PyList::empty(py).into(),
        })
    }
}

impl PyiMethods for NodeData {
    fn pyi_methods() -> String {
        "    def __init__(
        self,
        name: str,
    ) -> None: ..."
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.anim_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::anim_data::TrackData)]
#[pyrepr("ssbh_data_py.anim_data")]
#[pyi(has_methods = true)]
pub struct TrackData {
    #[pyo3(get, set)]
    pub name: String,

    // TODO: Does it make sense to use numpy here?
    #[pyo3(get, set)]
    #[pyi(
        python_type = "Union[list[UvTransform], list[Transform],
                  list[float], list[bool], list[int], list[list[float]]]"
    )]
    pub values: Py<PyList>, // TODO: Is inferring the value type the best option?

    #[pyo3(get, set)]
    pub scale_options: ScaleOptions,
}

#[pymethods]
impl TrackData {
    #[new]
    fn new(py: Python, name: String) -> PyResult<Self> {
        Ok(TrackData {
            name,
            values: PyList::empty(py).into(),
            scale_options: ScaleOptions {
                inherit_scale: false,
                compensate_scale: false,
            },
        })
    }
}

impl PyiMethods for TrackData {
    fn pyi_methods() -> String {
        "    def __init__(
        self,
        name: str,
    ) -> None: ..."
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.anim_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::anim_data::ScaleOptions)]
#[pyrepr("ssbh_data_py.anim_data")]
#[pyi(has_methods = true)]
pub struct ScaleOptions {
    #[pyo3(get, set)]
    pub inherit_scale: bool,

    #[pyo3(get, set)]
    pub compensate_scale: bool,
}

#[pymethods]
impl ScaleOptions {
    #[new]
    fn new(_py: Python) -> PyResult<Self> {
        Ok(ScaleOptions {
            inherit_scale: false,
            compensate_scale: false,
        })
    }
}

impl PyiMethods for ScaleOptions {
    fn pyi_methods() -> String {
        "    def __init__(self) -> None: ...".to_string()
    }
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
            final_frame_index: 0.0,
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py)?
            .write_to_file(path)
            .map_err(|e| AnimDataError::new_err(format!("{}", e)))
    }
}

impl PyiMethods for AnimData {
    fn pyi_methods() -> String {
        r#"    def __init__(
        self,
        major_version: int = ...,
        minor_version: int = ...,
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

python_enum!(
    GroupType,
    ssbh_data::anim_data::GroupType,
    AnimDataError,
    "ssbh_data_py.anim_data"
);

// TODO: Document what component counts are expected.
#[pyclass(module = "ssbh_data_py.anim_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::anim_data::Transform)]
#[pyrepr("ssbh_data_py.anim_data")]
#[pyi(has_methods = true)]
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

#[pymethods]
impl Transform {
    #[new]
    fn new(scale: Py<PyList>, rotation: Py<PyList>, translation: Py<PyList>) -> PyResult<Self> {
        Ok(Transform {
            scale,
            rotation,
            translation,
        })
    }
}

impl PyiMethods for Transform {
    fn pyi_methods() -> String {
        "    def __init__(
        self,
        scale: list[float],
        rotation: list[float],
        translation: list[float],
    ) -> None: ..."
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.anim_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::anim_data::UvTransform)]
#[pyrepr("ssbh_data_py.anim_data")]
#[pyi(has_methods = true)]
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

#[pymethods]
impl UvTransform {
    #[new]
    fn new(
        scale_u: f32,
        scale_v: f32,
        rotation: f32,
        translate_u: f32,
        translate_v: f32,
    ) -> PyResult<Self> {
        Ok(UvTransform {
            scale_u,
            scale_v,
            rotation,
            translate_u,
            translate_v,
        })
    }
}

impl PyiMethods for UvTransform {
    fn pyi_methods() -> String {
        "    def __init__(
        self,
        scale_u: float,
        scale_v: float,
        rotation: float,
        translate_u: float,
        translate_v: float
    ) -> None: ..."
            .to_string()
    }
}

// TODO: This is shared with other modules?
impl MapPy<ssbh_data::anim_data::Vector4> for Py<PyList> {
    fn map_py(&self, py: Python) -> PyResult<ssbh_data::anim_data::Vector4> {
        let values: [f32; 4] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<Py<PyList>> for ssbh_data::anim_data::Vector4 {
    fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
        Ok(PyList::new(py, self.to_array()).into())
    }
}

impl MapPy<PyObject> for ssbh_data::anim_data::Vector4 {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.to_array().into_py(py))
    }
}

impl MapPy<ssbh_data::anim_data::Vector4> for PyObject {
    fn map_py(&self, py: Python) -> PyResult<ssbh_data::anim_data::Vector4> {
        let values: [f32; 4] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<ssbh_data::anim_data::Vector3> for Py<PyList> {
    fn map_py(&self, py: Python) -> PyResult<ssbh_data::anim_data::Vector3> {
        let values: [f32; 3] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<Py<PyList>> for ssbh_data::anim_data::Vector3 {
    fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
        Ok(PyList::new(py, self.to_array()).into())
    }
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
        create_track_values_rs(py, self.as_ref(py))
    }
}

fn create_track_values_rs(py: Python, values: &PyList) -> PyResult<TrackValuesRs> {
    // We don't know the type, so just try one until it works.
    // TODO: Clean up this code.
    values
        .extract::<Vec<bool>>()
        .map(TrackValuesRs::Boolean)
        .or_else(|_| {
            // Pattern index needs to come before float.
            // This avoids conflicts with integer literals being interpreted as floats.
            values
                .extract::<Vec<u32>>()
                .map(TrackValuesRs::PatternIndex)
        })
        .or_else(|_| values.extract::<Vec<f32>>().map(TrackValuesRs::Float))
        .or_else(|_| {
            values.extract::<Vec<[f32; 4]>>().map(|v| {
                TrackValuesRs::Vector4(
                    v.into_iter()
                        .map(ssbh_data::anim_data::Vector4::from)
                        .collect(),
                )
            })
        })
        .or_else(|_| {
            let v = values.extract::<Vec<UvTransform>>()?;
            Ok(TrackValuesRs::UvTransform(
                v.into_iter()
                    .map(|t| t.map_py(py))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        })
        .or_else(|_: PyErr| {
            let v = values.extract::<Vec<Transform>>()?;
            Ok(TrackValuesRs::Transform(
                v.into_iter()
                    .map(|t| t.map_py(py))
                    .collect::<Result<Vec<_>, _>>()?,
            ))
        })
}

#[cfg(test)]
mod tests {
    use crate::{eval_python_code, run_python_code};
    use indoc::indoc;
    use ssbh_data::anim_data::{Vector3, Vector4};

    use super::*;

    #[test]
    fn create_anim_data() {
        run_python_code(indoc! {r#"
            a = ssbh_data_py.anim_data.AnimData()
            assert a.major_version == 2
            assert a.minor_version == 0
            assert a.groups == []
        "#})
        .unwrap();
    }

    #[test]
    fn create_group_data() {
        run_python_code(indoc! {r#"
            a = ssbh_data_py.anim_data.GroupData(ssbh_data_py.anim_data.GroupType.Transform)
            assert a.group_type.name == 'Transform'
            assert a.nodes == []
        "#})
        .unwrap();
    }

    #[test]
    fn create_node_data() {
        run_python_code(indoc! {r#"
            a = ssbh_data_py.anim_data.NodeData('abc')
            assert a.name == 'abc'
            assert a.tracks == []
        "#})
        .unwrap();
    }

    #[test]
    fn create_track_data() {
        run_python_code(indoc! {r#"
            a = ssbh_data_py.anim_data.TrackData('abc')
            assert a.name == 'abc'
            assert a.values == []
            assert a.scale_options.inherit_scale == False
            assert a.scale_options.compensate_scale == False
            assert a.scale_options.inherit_scale == False
            assert a.scale_options.compensate_scale == False
        "#})
        .unwrap();
    }

    #[test]
    fn create_scale_options() {
        run_python_code(indoc! {r#"
            o = ssbh_data_py.anim_data.ScaleOptions()
            assert o.inherit_scale == False
            assert o.compensate_scale == False
        "#})
        .unwrap();
    }

    #[test]
    fn create_transform() {
        run_python_code(indoc! {r#"
            t = ssbh_data_py.anim_data.Transform([1, 2, 3], [4, 5, 6, 7], [8, 9, 10])
            assert t.scale == [1, 2, 3]
            assert t.rotation == [4, 5, 6, 7]
            assert t.translation == [8, 9, 10]
        "#})
        .unwrap();
    }

    #[test]
    fn transform_repr() {
        // Check that repr can be used to construct the type.
        run_python_code(indoc! {r#"
            t = ssbh_data_py.anim_data.Transform([1, 2, 3], [4, 5, 6, 7], [8, 9, 10])
            s = repr(t)
            assert s == 'ssbh_data_py.anim_data.Transform([1, 2, 3], [4, 5, 6, 7], [8, 9, 10])'
            t2 = eval(s)
            assert t2.scale == [1, 2, 3]
            assert t2.rotation == [4, 5, 6, 7]
            assert t2.translation == [8, 9, 10]
        "#})
        .unwrap();
    }

    #[test]
    fn create_uv_transform() {
        run_python_code(indoc! {r#"
            t = ssbh_data_py.anim_data.UvTransform(1,2,3,4,5)
            assert t.scale_u == 1
            assert t.scale_v == 2
            assert t.rotation == 3
            assert t.translate_u == 4
            assert t.translate_v == 5
        "#})
        .unwrap();
    }

    #[test]
    fn uv_transform_repr() {
        // Check that repr can be used to construct the type.
        run_python_code(indoc! {r#"
            t = ssbh_data_py.anim_data.UvTransform(1,2,3,4,5)
            s = repr(t)
            assert s == 'ssbh_data_py.anim_data.UvTransform(1, 2, 3, 4, 5)'
            t2 = eval(s)
            assert t2.scale_u == 1
            assert t2.scale_v == 2
            assert t2.rotation == 3
            assert t2.translate_u == 4
            assert t2.translate_v == 5
        "#})
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn group_type_is_immutable() {
        run_python_code(indoc! {r#"
            g = ssbh_data_py.anim_data.GroupType.Transform
            g.name = 'abc'
            g.value = 4
        "#})
        .unwrap();
    }

    #[test]
    fn group_type_classattrs() {
        run_python_code(indoc! {r#"
            g = ssbh_data_py.anim_data.GroupType.Transform
            assert g.name == 'Transform' and g.value == 1

            g = ssbh_data_py.anim_data.GroupType.Visibility
            assert g.name == 'Visibility' and g.value == 2

            g = ssbh_data_py.anim_data.GroupType.Material
            assert g.name == 'Material' and g.value == 4

            g = ssbh_data_py.anim_data.GroupType.Camera
            assert g.name == 'Camera' and g.value == 5
        "#})
        .unwrap();
    }

    #[test]
    fn create_group_types_py() {
        let g: GroupType = ssbh_data::anim_data::GroupType::Transform.into();
        assert_eq!("Transform", g.name);
        assert_eq!(ssbh_data::anim_data::GroupType::Transform as u64, g.value);

        let g: GroupType = ssbh_data::anim_data::GroupType::Visibility.into();
        assert_eq!("Visibility", g.name);
        assert_eq!(ssbh_data::anim_data::GroupType::Visibility as u64, g.value);

        let g: GroupType = ssbh_data::anim_data::GroupType::Camera.into();
        assert_eq!("Camera", g.name);
        assert_eq!(ssbh_data::anim_data::GroupType::Camera as u64, g.value);

        let g: GroupType = ssbh_data::anim_data::GroupType::Material.into();
        assert_eq!("Material", g.name);
        assert_eq!(ssbh_data::anim_data::GroupType::Material as u64, g.value);
    }

    #[test]
    fn create_track_values_rs_floats() {
        eval_python_code("[0.5, 1, 3.4]", |py, x| {
            let data: &PyList = x.downcast().unwrap();
            assert_eq!(
                TrackValuesRs::Float(vec![0.5, 1.0, 3.4]),
                create_track_values_rs(py, data).unwrap()
            );
        });
    }

    #[test]
    fn create_track_values_rs_pattern_index() {
        eval_python_code("[0, 1, 2, 3]", |py, x| {
            let data: &PyList = x.downcast().unwrap();
            assert_eq!(
                TrackValuesRs::PatternIndex(vec![0, 1, 2, 3]),
                create_track_values_rs(py, data).unwrap()
            );
        });
    }

    #[test]
    fn create_track_values_rs_bool() {
        eval_python_code("[True, False, True]", |py, x| {
            let data: &PyList = x.downcast().unwrap();
            assert_eq!(
                TrackValuesRs::Boolean(vec![true, false, true]),
                create_track_values_rs(py, data).unwrap()
            );
        });
    }

    #[test]
    fn create_track_values_rs_vector4() {
        eval_python_code("[[1, 2, 3, 4], [0.5, 0.25, 0.3, 0.1]]", |py, x| {
            let data: &PyList = x.downcast().unwrap();
            assert_eq!(
                TrackValuesRs::Vector4(vec![
                    Vector4::new(1.0, 2.0, 3.0, 4.0),
                    Vector4::new(0.5, 0.25, 0.3, 0.1)
                ]),
                create_track_values_rs(py, data).unwrap()
            );
        });
    }

    #[test]
    fn create_track_values_rs_transform() {
        eval_python_code(
            indoc! {r#"
                [ssbh_data_py.anim_data.Transform([1, 2, 3], [4, 5, 6, 7], [1, 2, 3]), 
                 ssbh_data_py.anim_data.Transform(
                    scale=[0, 1, 2],
                    rotation=[1, 2, 3, 4],
                    translation=[9, 8, 0.4])]
            "#},
            |py, x| {
                let data: &PyList = x.downcast().unwrap();
                assert_eq!(
                    TrackValuesRs::Transform(vec![
                        ssbh_data::anim_data::Transform {
                            rotation: Vector4::new(4.0, 5.0, 6.0, 7.0),
                            translation: Vector3::new(1.0, 2.0, 3.0),
                            scale: Vector3::new(1.0, 2.0, 3.0),
                        },
                        ssbh_data::anim_data::Transform {
                            rotation: Vector4::new(1.0, 2.0, 3.0, 4.0),
                            translation: Vector3::new(9.0, 8.0, 0.4),
                            scale: Vector3::new(0.0, 1.0, 2.0),
                        }
                    ]),
                    create_track_values_rs(py, data).unwrap()
                );
            },
        );
    }
}

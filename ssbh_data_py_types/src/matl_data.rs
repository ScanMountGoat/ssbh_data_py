use crate::{python_enum, MapPy, PyInit, PyRepr, PyTypeString, Pyi, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};

mod enums;

create_exception!(ssbh_data_py, MatlDataError, pyo3::exceptions::PyException);

pub fn matl_data(py: Python, module: &PyModule) -> PyResult<()> {
    let matl_data = PyModule::new(py, "matl_data")?;
    // TODO: Automatically register classes?
    matl_data.add_class::<MatlData>()?;
    matl_data.add_class::<MatlEntryData>()?;

    matl_data.add_class::<ParamId>()?;
    matl_data.add_class::<BlendFactor>()?;
    matl_data.add_class::<FillMode>()?;
    matl_data.add_class::<CullMode>()?;
    matl_data.add_class::<WrapMode>()?;
    matl_data.add_class::<MinFilter>()?;
    matl_data.add_class::<MagFilter>()?;
    matl_data.add_class::<MaxAnisotropy>()?;

    matl_data.add_class::<BlendStateParam>()?;
    matl_data.add_class::<FloatParam>()?;
    matl_data.add_class::<BooleanParam>()?;
    matl_data.add_class::<Vector4Param>()?;
    matl_data.add_class::<RasterizerStateParam>()?;
    matl_data.add_class::<SamplerParam>()?;
    matl_data.add_class::<TextureParam>()?;
    matl_data.add_class::<UvTransformParam>()?;
    matl_data.add_class::<UvTransform>()?;

    matl_data.add_class::<RasterizerStateData>()?;
    matl_data.add_class::<SamplerData>()?;
    matl_data.add_class::<BlendStateData>()?;

    matl_data.add_function(wrap_pyfunction!(read_matl, matl_data)?)?;

    module.add_submodule(matl_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::MatlData)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct MatlData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[MatlEntryData]")]
    pub entries: Py<PyList>,
}

#[pymethods]
impl MatlData {
    #[new]
    #[pyo3(signature = (major_version = 1, minor_version = 6))]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(MatlData {
            major_version,
            minor_version,
            entries: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py, false)?
            .write_to_file(path)
            .map_err(|e| MatlDataError::new_err(format!("{}", e)))
    }
}

impl PyiMethods for MatlData {
    fn pyi_methods() -> String {
        r#"    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 6,
    ) -> None: ...
    
    def save(self, path: str) -> None: ..."#
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::matl_data::MatlEntryData)]
#[pyrepr("ssbh_data_py.matl_data")]
pub struct MatlEntryData {
    #[pyo3(get, set)]
    pub material_label: String,

    #[pyo3(get, set)]
    pub shader_label: String,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[BlendStateParam]")]
    pub blend_states: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[FloatParam]")]
    pub floats: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[BooleanParam]")]
    pub booleans: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[Vector4Param]")]
    pub vectors: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[RasterizerStateParam]")]
    pub rasterizer_states: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[SamplerParam]")]
    pub samplers: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[TextureParam]")]
    pub textures: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::empty(py).into()")]
    #[pyi(default = "[]", python_type = "list[UvTransformParam]")]
    pub uv_transforms: Py<PyList>,
}

macro_rules! param_new_impl {
    ($(($py_class:ty,$data:ty)),*) => {
        $(
            #[pymethods]
            impl $py_class {
                #[new]
                fn new(_py: Python, param_id: ParamId, data: $data) -> PyResult<Self> {
                    Ok(Self { param_id, data })
                }
            }

            // TODO: Find a better place to generate the methods.
            impl crate::PyiMethods for $py_class {
                fn pyi_methods() -> String {
                    format!(
r#"    def __init__(
        self,
        param_id: ParamId,
        data: {},
    ) -> None: ..."#, <$data>::py_type_string())
                }
            }
        )*
    };
}

// Define a Python class for each parameter type to avoid dealing with generics.
param_new_impl!(
    (BlendStateParam, BlendStateData),
    (FloatParam, f32),
    (BooleanParam, bool),
    (Vector4Param, PyObject),
    (RasterizerStateParam, RasterizerStateData),
    (SamplerParam, SamplerData),
    (TextureParam, String),
    (UvTransformParam, UvTransform)
);

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::BlendStateParam)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct BlendStateParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: BlendStateData,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::FloatParam)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct FloatParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: f32,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::BooleanParam)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct BooleanParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: bool,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::Vector4Param)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct Vector4Param {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[float]")]
    pub data: PyObject,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::RasterizerStateParam)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct RasterizerStateParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: RasterizerStateData,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::SamplerParam)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct SamplerParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: SamplerData,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::TextureParam)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct TextureParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: String,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::UvTransformParam)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct UvTransformParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: UvTransform,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::matl_data::UvTransform)]
#[pyrepr("ssbh_data_py.matl_data")]
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

python_enum!(
    ParamId,
    ssbh_data::matl_data::ParamId,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::matl_data::BlendStateData)]
#[pyrepr("ssbh_data_py.matl_data")]
pub struct BlendStateData {
    #[pyo3(get, set)]
    #[pyinit(default = "BlendFactor::one()")]
    #[pyi(default = "BlendFactor.One")]
    pub source_color: BlendFactor,

    #[pyo3(get, set)]
    #[pyinit(default = "BlendFactor::zero()")]
    #[pyi(default = "BlendFactor.Zero")]
    pub destination_color: BlendFactor,

    #[pyo3(get, set)]
    #[pyinit(default = "false")]
    #[pyi(default = "False")]
    pub alpha_sample_to_coverage: bool,
}

python_enum!(
    BlendFactor,
    ssbh_data::matl_data::BlendFactor,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::RasterizerStateData)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct RasterizerStateData {
    #[pyo3(get, set)]
    pub fill_mode: FillMode,

    #[pyo3(get, set)]
    pub cull_mode: CullMode,

    #[pyo3(get, set)]
    pub depth_bias: f32,
}

#[pymethods]
impl RasterizerStateData {
    #[new]
    fn new(_py: Python) -> PyResult<Self> {
        Ok(Self {
            fill_mode: FillMode::solid(),
            cull_mode: CullMode::back(),
            depth_bias: 0.0,
        })
    }
}

impl crate::PyiMethods for RasterizerStateData {
    fn pyi_methods() -> String {
        "    def __init__(self) -> None: ...".to_string()
    }
}

python_enum!(
    FillMode,
    ssbh_data::matl_data::FillMode,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

python_enum!(
    CullMode,
    ssbh_data::matl_data::CullMode,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::matl_data::SamplerData)]
#[pyrepr("ssbh_data_py.matl_data")]
#[pyi(has_methods = true)]
pub struct SamplerData {
    #[pyo3(get, set)]
    pub wraps: WrapMode,

    #[pyo3(get, set)]
    pub wrapt: WrapMode,

    #[pyo3(get, set)]
    pub wrapr: WrapMode,

    #[pyo3(get, set)]
    pub min_filter: MinFilter,

    #[pyo3(get, set)]
    pub mag_filter: MagFilter,

    #[pyi(python_type = "list[float]")]
    #[pyo3(get, set)]
    pub border_color: PyObject,

    #[pyo3(get, set)]
    pub lod_bias: f32,

    #[pyo3(get, set)]
    pub max_anisotropy: Option<MaxAnisotropy>,
}

// TODO: Is it worth having default parameterless constructors?
// This will cause increased breaking changes and potentially unwanted default values.
#[pymethods]
impl SamplerData {
    #[new]
    fn new(py: Python) -> PyResult<Self> {
        Ok(Self {
            wraps: WrapMode::clamptoedge(),
            wrapt: WrapMode::clamptoedge(),
            wrapr: WrapMode::clamptoedge(),
            min_filter: MinFilter::linearmipmaplinear(),
            mag_filter: MagFilter::linear(),
            border_color: PyList::new(py, [0.0; 4]).into(),
            lod_bias: 0.0,
            max_anisotropy: Some(MaxAnisotropy::two()),
        })
    }
}

impl crate::PyiMethods for SamplerData {
    fn pyi_methods() -> String {
        "    def __init__(self) -> None: ...".to_string()
    }
}

python_enum!(
    WrapMode,
    ssbh_data::matl_data::WrapMode,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

python_enum!(
    MinFilter,
    ssbh_data::matl_data::MinFilter,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

python_enum!(
    MagFilter,
    ssbh_data::matl_data::MagFilter,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

python_enum!(
    MaxAnisotropy,
    ssbh_data::matl_data::MaxAnisotropy,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

#[pyfunction]
fn read_matl(py: Python, path: &str) -> PyResult<MatlData> {
    ssbh_data::matl_data::MatlData::from_file(path)
        .map_err(|e| MatlDataError::new_err(format!("{}", e)))?
        .map_py(py, false)
}

use crate::{python_enum, MapPy, PyInit, PyRepr, PyTypeString, Pyi, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};

create_exception!(ssbh_data_py, MatlDataError, pyo3::exceptions::PyException);

pub fn matl_data(py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
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

    matl_data.add_function(wrap_pyfunction!(read_matl, &matl_data)?)?;

    module.add_submodule(&matl_data)?;
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
    "ssbh_data_py.matl_data",
    BlendState0,
    RasterizerState0,
    CustomVector8,
    Texture4,
    CustomVector0,
    CustomBoolean1,
    CustomVector13,
    CustomBoolean3,
    CustomBoolean4,
    Texture7,
    CustomVector14,
    CustomFloat8,
    Texture0,
    Texture6,
    CustomVector3,
    Texture5,
    CustomVector30,
    CustomBoolean2,
    CustomVector31,
    CustomBoolean11,
    Texture14,
    CustomVector27,
    Texture9,
    CustomVector29,
    CustomVector6,
    CustomVector11,
    CustomBoolean5,
    CustomBoolean12,
    CustomBoolean6,
    Texture2,
    Texture1,
    CustomVector7,
    CustomFloat1,
    Texture3,
    CustomFloat19,
    CustomVector18,
    CustomBoolean9,
    CustomVector42,
    CustomVector32,
    CustomBoolean7,
    CustomFloat4,
    CustomFloat10,
    Texture11,
    Texture16,
    CustomVector47,
    Texture10,
    CustomVector34,
    CustomFloat11,
    CustomFloat12,
    CustomVector35,
    CustomFloat6,
    CustomFloat18,
    CustomVector37,
    CustomVector38,
    CustomVector39,
    CustomVector19,
    CustomVector23,
    Texture13,
    CustomVector21,
    CustomBoolean0,
    CustomVector20,
    CustomBoolean10,
    CustomVector40,
    Texture12,
    CustomVector22,
    Texture8,
    CustomVector46,
    CustomFloat17,
    CustomVector24,
    CustomBoolean8,
    CustomVector33,
    CustomVector4,
    CustomFloat0,
    CustomVector1,
    CustomVector2,
    CustomVector5,
    CustomVector15,
    CustomVector16,
    CustomVector43,
    CustomVector44,
    CustomVector45,
    CustomVector9,
    CustomVector10,
    Diffuse,
    Specular,
    Ambient,
    BlendMap,
    Transparency,
    DiffuseMapLayer1,
    CosinePower,
    SpecularPower,
    Fresnel,
    Roughness,
    EmissiveScale,
    EnableDiffuse,
    EnableSpecular,
    EnableAmbient,
    DiffuseMapLayer2,
    EnableTransparency,
    EnableOpacity,
    EnableCosinePower,
    EnableSpecularPower,
    EnableFresnel,
    EnableRoughness,
    EnableEmissiveScale,
    WorldMatrix,
    ViewMatrix,
    ProjectionMatrix,
    WorldViewMatrix,
    ViewInverseMatrix,
    ViewProjectionMatrix,
    WorldViewProjectionMatrix,
    WorldInverseTransposeMatrix,
    DiffuseMap,
    SpecularMap,
    AmbientMap,
    EmissiveMap,
    SpecularMapLayer1,
    TransparencyMap,
    NormalMap,
    DiffuseCubeMap,
    ReflectionMap,
    ReflectionCubeMap,
    RefractionMap,
    AmbientOcclusionMap,
    LightMap,
    AnisotropicMap,
    RoughnessMap,
    ReflectionMask,
    OpacityMask,
    UseDiffuseMap,
    UseSpecularMap,
    UseAmbientMap,
    UseEmissiveMap,
    UseTranslucencyMap,
    UseTransparencyMap,
    UseNormalMap,
    UseDiffuseCubeMap,
    UseReflectionMap,
    UseReflectionCubeMap,
    UseRefractionMap,
    UseAmbientOcclusionMap,
    UseLightMap,
    UseAnisotropicMap,
    UseRoughnessMap,
    UseReflectionMask,
    UseOpacityMask,
    DiffuseSampler,
    SpecularSampler,
    NormalSampler,
    ReflectionSampler,
    SpecularMapLayer2,
    NormalMapLayer1,
    NormalMapBc5,
    NormalMapLayer2,
    RoughnessMapLayer1,
    RoughnessMapLayer2,
    UseDiffuseUvTransform1,
    UseDiffuseUvTransform2,
    UseSpecularUvTransform1,
    UseSpecularUvTransform2,
    UseNormalUvTransform1,
    UseNormalUvTransform2,
    ShadowDepthBias,
    ShadowMap0,
    ShadowMap1,
    ShadowMap2,
    ShadowMap3,
    ShadowMap4,
    ShadowMap5,
    ShadowMap6,
    ShadowMap7,
    CastShadow,
    ReceiveShadow,
    ShadowMapSampler,
    Texture15,
    Sampler0,
    Sampler1,
    Sampler2,
    Sampler3,
    Sampler4,
    Sampler5,
    Sampler6,
    Sampler7,
    Sampler8,
    Sampler9,
    Sampler10,
    Sampler11,
    Sampler12,
    Sampler13,
    Sampler14,
    Sampler15,
    CustomBuffer0,
    CustomBuffer1,
    CustomBuffer2,
    CustomBuffer3,
    CustomBuffer4,
    CustomBuffer5,
    CustomBuffer6,
    CustomBuffer7,
    CustomMatrix0,
    CustomMatrix1,
    CustomMatrix2,
    CustomMatrix3,
    CustomMatrix4,
    CustomMatrix5,
    CustomMatrix6,
    CustomMatrix7,
    CustomMatrix8,
    CustomMatrix9,
    CustomMatrix10,
    CustomMatrix11,
    CustomMatrix12,
    CustomMatrix13,
    CustomMatrix14,
    CustomMatrix15,
    CustomMatrix16,
    CustomMatrix17,
    CustomMatrix18,
    CustomMatrix19,
    CustomVector12,
    CustomVector17,
    CustomColor0,
    CustomColor1,
    CustomColor2,
    CustomColor3,
    CustomColor4,
    CustomColor5,
    CustomColor6,
    CustomColor7,
    CustomColor8,
    CustomColor9,
    CustomColor10,
    CustomColor11,
    CustomColor12,
    CustomColor13,
    CustomColor14,
    CustomColor15,
    CustomColor16,
    CustomColor17,
    CustomColor18,
    CustomColor19,
    CustomFloat2,
    CustomFloat3,
    CustomFloat5,
    CustomFloat7,
    CustomFloat9,
    CustomFloat13,
    CustomFloat14,
    CustomFloat15,
    CustomFloat16,
    CustomInteger0,
    CustomInteger1,
    CustomInteger2,
    CustomInteger3,
    CustomInteger4,
    CustomInteger5,
    CustomInteger6,
    CustomInteger7,
    CustomInteger8,
    CustomInteger9,
    CustomInteger10,
    CustomInteger11,
    CustomInteger12,
    CustomInteger13,
    CustomInteger14,
    CustomInteger15,
    CustomInteger16,
    CustomInteger17,
    CustomInteger18,
    CustomInteger19,
    CustomBoolean13,
    CustomBoolean14,
    CustomBoolean15,
    CustomBoolean16,
    CustomBoolean17,
    CustomBoolean18,
    CustomBoolean19,
    UvTransform0,
    UvTransform1,
    UvTransform2,
    UvTransform3,
    UvTransform4,
    UvTransform5,
    UvTransform6,
    UvTransform7,
    UvTransform8,
    UvTransform9,
    UvTransform10,
    UvTransform11,
    UvTransform12,
    UvTransform13,
    UvTransform14,
    UvTransform15,
    DiffuseUvTransform1,
    DiffuseUvTransform2,
    SpecularUvTransform1,
    SpecularUvTransform2,
    NormalUvTransform1,
    NormalUvTransform2,
    DiffuseUvTransform,
    SpecularUvTransform,
    NormalUvTransform,
    UseDiffuseUvTransform,
    UseSpecularUvTransform,
    UseNormalUvTransform,
    BlendState1,
    BlendState2,
    BlendState3,
    BlendState4,
    BlendState5,
    BlendState6,
    BlendState7,
    BlendState8,
    BlendState9,
    BlendState10,
    RasterizerState1,
    RasterizerState2,
    RasterizerState3,
    RasterizerState4,
    RasterizerState5,
    RasterizerState6,
    RasterizerState7,
    RasterizerState8,
    RasterizerState9,
    RasterizerState10,
    ShadowColor,
    EmissiveMapLayer1,
    EmissiveMapLayer2,
    AlphaTestFunc,
    AlphaTestRef,
    Texture17,
    Texture18,
    Texture19,
    Sampler16,
    Sampler17,
    Sampler18,
    Sampler19,
    CustomVector25,
    CustomVector26,
    CustomVector28,
    CustomVector36,
    CustomVector41,
    CustomVector48,
    CustomVector49,
    CustomVector50,
    CustomVector51,
    CustomVector52,
    CustomVector53,
    CustomVector54,
    CustomVector55,
    CustomVector56,
    CustomVector57,
    CustomVector58,
    CustomVector59,
    CustomVector60,
    CustomVector61,
    CustomVector62,
    CustomVector63,
    UseBaseColorMap,
    UseMetallicMap,
    BaseColorMap,
    BaseColorMapLayer1,
    MetallicMap,
    MetallicMapLayer1,
    DiffuseLightingAoOffset
);

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::matl_data::BlendStateData)]
#[pyrepr("ssbh_data_py.matl_data")]
pub struct BlendStateData {
    #[pyo3(get, set)]
    #[pyinit(default = "BlendFactor::One()")]
    #[pyi(default = "BlendFactor.One")]
    pub source_color: BlendFactor,

    #[pyo3(get, set)]
    #[pyinit(default = "BlendFactor::Zero()")]
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
    "ssbh_data_py.matl_data",
    Zero,
    One,
    SourceAlpha,
    DestinationAlpha,
    SourceColor,
    DestinationColor,
    OneMinusSourceAlpha,
    OneMinusDestinationAlpha,
    OneMinusSourceColor,
    OneMinusDestinationColor,
    SourceAlphaSaturate
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
            fill_mode: ssbh_data::matl_data::FillMode::Solid.into(),
            cull_mode: ssbh_data::matl_data::CullMode::Back.into(),
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
    "ssbh_data_py.matl_data",
    Line,
    Solid
);

python_enum!(
    CullMode,
    ssbh_data::matl_data::CullMode,
    MatlDataError,
    "ssbh_data_py.matl_data",
    Back,
    Front,
    Disabled
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
            wraps: ssbh_data::matl_data::WrapMode::ClampToEdge.into(),
            wrapt: ssbh_data::matl_data::WrapMode::ClampToEdge.into(),
            wrapr: ssbh_data::matl_data::WrapMode::ClampToEdge.into(),
            min_filter: ssbh_data::matl_data::MinFilter::LinearMipmapLinear.into(),
            mag_filter: ssbh_data::matl_data::MagFilter::Linear.into(),
            border_color: PyList::new(py, [0.0; 4])?.into(),
            lod_bias: 0.0,
            max_anisotropy: Some(ssbh_data::matl_data::MaxAnisotropy::Two.into()),
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
    "ssbh_data_py.matl_data",
    Repeat,
    ClampToEdge,
    MirroredRepeat,
    ClampToBorder
);

python_enum!(
    MinFilter,
    ssbh_data::matl_data::MinFilter,
    MatlDataError,
    "ssbh_data_py.matl_data",
    Nearest,
    LinearMipmapLinear,
    LinearMipmapLinear2
);

python_enum!(
    MagFilter,
    ssbh_data::matl_data::MagFilter,
    MatlDataError,
    "ssbh_data_py.matl_data",
    Nearest,
    Linear,
    Linear2
);

python_enum!(
    MaxAnisotropy,
    ssbh_data::matl_data::MaxAnisotropy,
    MatlDataError,
    "ssbh_data_py.matl_data",
    One,
    Two,
    Four,
    Eight,
    Sixteen
);

#[pyfunction]
fn read_matl(py: Python, path: &str) -> PyResult<MatlData> {
    ssbh_data::matl_data::MatlData::from_file(path)
        .map_err(|e| MatlDataError::new_err(format!("{}", e)))?
        .map_py(py, false)
}

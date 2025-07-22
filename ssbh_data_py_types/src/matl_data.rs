use crate::python_enum;
use pyo3::{create_exception, prelude::*};

create_exception!(ssbh_data_py, MatlDataError, pyo3::exceptions::PyException);

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

#[pymodule]
pub mod matl_data {
    pub use super::*;

    use crate::{map_from_color4f, map_from_vector4, map_into_color4f, map_into_vector4};
    use crate::{PyInit, PyRepr, PyTypeString, Pyi, PyiMethods};
    use map_py::MapPy;
    use map_py::TypedList;

    #[pymodule_export]
    pub use super::ParamId;

    #[pymodule_export]
    pub use super::BlendFactor;

    #[pymodule_export]
    pub use super::FillMode;

    #[pymodule_export]
    pub use super::CullMode;

    #[pymodule_export]
    pub use super::WrapMode;

    #[pymodule_export]
    pub use super::MinFilter;

    #[pymodule_export]
    pub use super::MagFilter;

    #[pymodule_export]
    pub use super::MaxAnisotropy;

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::MatlData)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct MatlData {
        pub major_version: u16,
        pub minor_version: u16,
        pub entries: TypedList<MatlEntryData>,
    }

    #[pymethods]
    impl MatlData {
        #[new]
        #[pyo3(signature = (major_version = 1, minor_version = 6))]
        fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
            Ok(MatlData {
                major_version,
                minor_version,
                entries: TypedList::empty(py),
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.clone()
                .map_py(py)?
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

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::matl_data::MatlEntryData)]
    #[pyrepr("ssbh_data_py.matl_data")]
    pub struct MatlEntryData {
        pub material_label: String,

        pub shader_label: String,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub blend_states: TypedList<BlendStateParam>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub floats: TypedList<FloatParam>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub booleans: TypedList<BooleanParam>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub vectors: TypedList<Vector4Param>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub rasterizer_states: TypedList<RasterizerStateParam>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub samplers: TypedList<SamplerParam>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub textures: TypedList<TextureParam>,

        #[pyinit(default = "TypedList::empty(py)")]
        #[pyi(default = "[]")]
        pub uv_transforms: TypedList<UvTransformParam>,
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
        (Vector4Param, TypedList<f32>),
        (RasterizerStateParam, RasterizerStateData),
        (SamplerParam, SamplerData),
        (TextureParam, String),
        (UvTransformParam, UvTransform)
    );

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::BlendStateParam)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct BlendStateParam {
        pub param_id: ParamId,

        pub data: BlendStateData,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::FloatParam)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct FloatParam {
        pub param_id: ParamId,

        pub data: f32,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::BooleanParam)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct BooleanParam {
        pub param_id: ParamId,

        pub data: bool,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::Vector4Param)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct Vector4Param {
        pub param_id: ParamId,

        #[map(from(map_from_vector4), into(map_into_vector4))]
        pub data: TypedList<f32>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::RasterizerStateParam)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct RasterizerStateParam {
        pub param_id: ParamId,

        pub data: RasterizerStateData,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::SamplerParam)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct SamplerParam {
        pub param_id: ParamId,

        pub data: SamplerData,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::TextureParam)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct TextureParam {
        pub param_id: ParamId,

        pub data: String,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::UvTransformParam)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct UvTransformParam {
        pub param_id: ParamId,

        pub data: UvTransform,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::matl_data::UvTransform)]
    #[pyrepr("ssbh_data_py.matl_data")]
    pub struct UvTransform {
        pub scale_u: f32,

        pub scale_v: f32,

        pub rotation: f32,

        pub translate_u: f32,

        pub translate_v: f32,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
    #[map(ssbh_data::matl_data::BlendStateData)]
    #[pyrepr("ssbh_data_py.matl_data")]
    pub struct BlendStateData {
        #[pyinit(default = "BlendFactor::One()")]
        #[pyi(default = "BlendFactor.One")]
        pub source_color: BlendFactor,

        #[pyinit(default = "BlendFactor::Zero()")]
        #[pyi(default = "BlendFactor.Zero")]
        pub destination_color: BlendFactor,

        #[pyinit(default = "false")]
        #[pyi(default = "False")]
        pub alpha_sample_to_coverage: bool,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::RasterizerStateData)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct RasterizerStateData {
        pub fill_mode: FillMode,

        pub cull_mode: CullMode,

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

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::matl_data::SamplerData)]
    #[pyrepr("ssbh_data_py.matl_data")]
    #[pyi(has_methods = true)]
    pub struct SamplerData {
        pub wraps: WrapMode,
        pub wrapt: WrapMode,
        pub wrapr: WrapMode,

        pub min_filter: MinFilter,
        pub mag_filter: MagFilter,

        #[map(from(map_from_color4f), into(map_into_color4f))]
        pub border_color: TypedList<f32>,

        pub lod_bias: f32,
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
                border_color: vec![0.0; 4].map_py(py)?,
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

    #[pyfunction]
    fn read_matl(py: Python, path: &str) -> PyResult<MatlData> {
        ssbh_data::matl_data::MatlData::from_file(path)
            .map_err(|e| MatlDataError::new_err(format!("{}", e)))?
            .map_py(py)
    }
}

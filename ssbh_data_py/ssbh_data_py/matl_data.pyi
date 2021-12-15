from typing import List, Tuple, Any, Optional


def read_matl(path: str) -> MatlData: ...


class MatlData:
    major_version: int
    minor_version: int
    entries: list[MatlEntryData]

    def __init__(
        self,
        major_version: int = ...,
        minor_version: int = ...,
    ) -> None: ...

    def save(self, path: str) -> None: ...


class MatlEntryData:
    material_label: str
    shader_label: str
    blend_states: list[BlendStateParam]
    floats: list[FloatParam]
    booleans: list[BooleanParam]
    vectors: list[Vector4Param]
    rasterizer_states: list[RasterizerStateParam]
    samplers: list[SamplerParam]
    textures: list[TextureParam]


class BlendStateParam:
    param_id: ParamId
    data: BlendStateData

    def __init__(
        self,
        param_id: ParamId = ...,
        data: BlendStateData = ...,
    ) -> None: ...


class FloatParam:
    param_id: ParamId
    data: float

    def __init__(
        self,
        param_id: ParamId = ...,
        data: float = ...,
    ) -> None: ...


class BooleanParam:
    param_id: ParamId
    data: bool

    def __init__(
        self,
        param_id: ParamId = ...,
        data: bool = ...,
    ) -> None: ...


class Vector4Param:
    param_id: ParamId
    data: list[float]

    def __init__(
        self,
        param_id: ParamId = ...,
        data: Any = ...,
    ) -> None: ...


class RasterizerStateParam:
    param_id: ParamId
    data: RasterizerStateData

    def __init__(
        self,
        param_id: ParamId = ...,
        data: RasterizerStateData = ...,
    ) -> None: ...


class SamplerParam:
    param_id: ParamId
    data: SamplerData

    def __init__(
        self,
        param_id: ParamId = ...,
        data: SamplerData = ...,
    ) -> None: ...


class TextureParam:
    param_id: ParamId
    data: str

    def __init__(
        self,
        param_id: ParamId = ...,
        data: str = ...,
    ) -> None: ...


class BlendStateData:
    source_color: BlendFactor
    destination_color: BlendFactor
    alpha_sample_to_coverage: bool

    def __init__(self) -> None: ...


class RasterizerStateData:
    fill_mode: FillMode
    cull_mode: CullMode
    depth_bias: float

    def __init__(self) -> None: ...


class SamplerData:
    wraps: WrapMode
    wrapt: WrapMode
    wrapr: WrapMode
    min_filter: MinFilter
    mag_filter: MagFilter
    border_color: list[float]
    lod_bias: float
    max_anisotropy: Optional[MaxAnisotropy]

    def __init__(self) -> None: ...


class ParamId:
    name: str
    value: int

    BlendState0: ParamId = ...
    RasterizerState0: ParamId = ...
    CustomVector8: ParamId = ...
    Texture4: ParamId = ...
    CustomVector0: ParamId = ...
    CustomBoolean1: ParamId = ...
    CustomVector13: ParamId = ...
    CustomBoolean3: ParamId = ...
    CustomBoolean4: ParamId = ...
    Texture7: ParamId = ...
    CustomVector14: ParamId = ...
    CustomFloat8: ParamId = ...
    Texture0: ParamId = ...
    Texture6: ParamId = ...
    CustomVector3: ParamId = ...
    Texture5: ParamId = ...
    CustomVector30: ParamId = ...
    CustomBoolean2: ParamId = ...
    CustomVector31: ParamId = ...
    CustomBoolean11: ParamId = ...
    Texture14: ParamId = ...
    CustomVector27: ParamId = ...
    Texture9: ParamId = ...
    CustomVector29: ParamId = ...
    CustomVector6: ParamId = ...
    CustomVector11: ParamId = ...
    CustomBoolean5: ParamId = ...
    CustomBoolean12: ParamId = ...
    CustomBoolean6: ParamId = ...
    Texture2: ParamId = ...
    Texture1: ParamId = ...
    CustomVector7: ParamId = ...
    CustomFloat1: ParamId = ...
    Texture3: ParamId = ...
    CustomFloat19: ParamId = ...
    CustomVector18: ParamId = ...
    CustomBoolean9: ParamId = ...
    CustomVector42: ParamId = ...
    CustomVector32: ParamId = ...
    CustomBoolean7: ParamId = ...
    CustomFloat4: ParamId = ...
    CustomFloat10: ParamId = ...
    Texture11: ParamId = ...
    Texture16: ParamId = ...
    CustomVector47: ParamId = ...
    Texture10: ParamId = ...
    CustomVector34: ParamId = ...
    CustomFloat11: ParamId = ...
    CustomFloat12: ParamId = ...
    CustomVector35: ParamId = ...
    CustomFloat6: ParamId = ...
    CustomFloat18: ParamId = ...
    CustomVector37: ParamId = ...
    CustomVector38: ParamId = ...
    CustomVector39: ParamId = ...
    CustomVector19: ParamId = ...
    CustomVector23: ParamId = ...
    Texture13: ParamId = ...
    CustomVector21: ParamId = ...
    CustomBoolean0: ParamId = ...
    CustomVector20: ParamId = ...
    CustomBoolean10: ParamId = ...
    CustomVector40: ParamId = ...
    Texture12: ParamId = ...
    CustomVector22: ParamId = ...
    Texture8: ParamId = ...
    CustomVector46: ParamId = ...
    CustomFloat17: ParamId = ...
    CustomVector24: ParamId = ...
    CustomBoolean8: ParamId = ...
    CustomVector33: ParamId = ...
    CustomVector4: ParamId = ...
    CustomFloat0: ParamId = ...
    CustomVector1: ParamId = ...
    CustomVector2: ParamId = ...
    CustomVector5: ParamId = ...
    CustomVector15: ParamId = ...
    CustomVector16: ParamId = ...
    CustomVector43: ParamId = ...
    CustomVector44: ParamId = ...
    CustomVector45: ParamId = ...
    CustomVector9: ParamId = ...
    CustomVector10: ParamId = ...
    Diffuse: ParamId = ...
    Specular: ParamId = ...
    Ambient: ParamId = ...
    BlendMap: ParamId = ...
    Transparency: ParamId = ...
    DiffuseMapLayer1: ParamId = ...
    CosinePower: ParamId = ...
    SpecularPower: ParamId = ...
    Fresnel: ParamId = ...
    Roughness: ParamId = ...
    EmissiveScale: ParamId = ...
    EnableDiffuse: ParamId = ...
    EnableSpecular: ParamId = ...
    EnableAmbient: ParamId = ...
    DiffuseMapLayer2: ParamId = ...
    EnableTransparency: ParamId = ...
    EnableOpacity: ParamId = ...
    EnableCosinePower: ParamId = ...
    EnableSpecularPower: ParamId = ...
    EnableFresnel: ParamId = ...
    EnableRoughness: ParamId = ...
    EnableEmissiveScale: ParamId = ...
    WorldMatrix: ParamId = ...
    ViewMatrix: ParamId = ...
    ProjectionMatrix: ParamId = ...
    WorldViewMatrix: ParamId = ...
    ViewInverseMatrix: ParamId = ...
    ViewProjectionMatrix: ParamId = ...
    WorldViewProjectionMatrix: ParamId = ...
    WorldInverseTransposeMatrix: ParamId = ...
    DiffuseMap: ParamId = ...
    SpecularMap: ParamId = ...
    AmbientMap: ParamId = ...
    EmissiveMap: ParamId = ...
    SpecularMapLayer1: ParamId = ...
    TransparencyMap: ParamId = ...
    NormalMap: ParamId = ...
    DiffuseCubeMap: ParamId = ...
    ReflectionMap: ParamId = ...
    ReflectionCubeMap: ParamId = ...
    RefractionMap: ParamId = ...
    AmbientOcclusionMap: ParamId = ...
    LightMap: ParamId = ...
    AnisotropicMap: ParamId = ...
    RoughnessMap: ParamId = ...
    ReflectionMask: ParamId = ...
    OpacityMask: ParamId = ...
    UseDiffuseMap: ParamId = ...
    UseSpecularMap: ParamId = ...
    UseAmbientMap: ParamId = ...
    UseEmissiveMap: ParamId = ...
    UseTranslucencyMap: ParamId = ...
    UseTransparencyMap: ParamId = ...
    UseNormalMap: ParamId = ...
    UseDiffuseCubeMap: ParamId = ...
    UseReflectionMap: ParamId = ...
    UseReflectionCubeMap: ParamId = ...
    UseRefractionMap: ParamId = ...
    UseAmbientOcclusionMap: ParamId = ...
    UseLightMap: ParamId = ...
    UseAnisotropicMap: ParamId = ...
    UseRoughnessMap: ParamId = ...
    UseReflectionMask: ParamId = ...
    UseOpacityMask: ParamId = ...
    DiffuseSampler: ParamId = ...
    SpecularSampler: ParamId = ...
    NormalSampler: ParamId = ...
    ReflectionSampler: ParamId = ...
    SpecularMapLayer2: ParamId = ...
    NormalMapLayer1: ParamId = ...
    NormalMapBc5: ParamId = ...
    NormalMapLayer2: ParamId = ...
    RoughnessMapLayer1: ParamId = ...
    RoughnessMapLayer2: ParamId = ...
    UseDiffuseUvTransform1: ParamId = ...
    UseDiffuseUvTransform2: ParamId = ...
    UseSpecularUvTransform1: ParamId = ...
    UseSpecularUvTransform2: ParamId = ...
    UseNormalUvTransform1: ParamId = ...
    UseNormalUvTransform2: ParamId = ...
    ShadowDepthBias: ParamId = ...
    ShadowMap0: ParamId = ...
    ShadowMap1: ParamId = ...
    ShadowMap2: ParamId = ...
    ShadowMap3: ParamId = ...
    ShadowMap4: ParamId = ...
    ShadowMap5: ParamId = ...
    ShadowMap6: ParamId = ...
    ShadowMap7: ParamId = ...
    CastShadow: ParamId = ...
    ReceiveShadow: ParamId = ...
    ShadowMapSampler: ParamId = ...
    Texture15: ParamId = ...
    Sampler0: ParamId = ...
    Sampler1: ParamId = ...
    Sampler2: ParamId = ...
    Sampler3: ParamId = ...
    Sampler4: ParamId = ...
    Sampler5: ParamId = ...
    Sampler6: ParamId = ...
    Sampler7: ParamId = ...
    Sampler8: ParamId = ...
    Sampler9: ParamId = ...
    Sampler10: ParamId = ...
    Sampler11: ParamId = ...
    Sampler12: ParamId = ...
    Sampler13: ParamId = ...
    Sampler14: ParamId = ...
    Sampler15: ParamId = ...
    CustomBuffer0: ParamId = ...
    CustomBuffer1: ParamId = ...
    CustomBuffer2: ParamId = ...
    CustomBuffer3: ParamId = ...
    CustomBuffer4: ParamId = ...
    CustomBuffer5: ParamId = ...
    CustomBuffer6: ParamId = ...
    CustomBuffer7: ParamId = ...
    CustomMatrix0: ParamId = ...
    CustomMatrix1: ParamId = ...
    CustomMatrix2: ParamId = ...
    CustomMatrix3: ParamId = ...
    CustomMatrix4: ParamId = ...
    CustomMatrix5: ParamId = ...
    CustomMatrix6: ParamId = ...
    CustomMatrix7: ParamId = ...
    CustomMatrix8: ParamId = ...
    CustomMatrix9: ParamId = ...
    CustomMatrix10: ParamId = ...
    CustomMatrix11: ParamId = ...
    CustomMatrix12: ParamId = ...
    CustomMatrix13: ParamId = ...
    CustomMatrix14: ParamId = ...
    CustomMatrix15: ParamId = ...
    CustomMatrix16: ParamId = ...
    CustomMatrix17: ParamId = ...
    CustomMatrix18: ParamId = ...
    CustomMatrix19: ParamId = ...
    CustomVector12: ParamId = ...
    CustomVector17: ParamId = ...
    CustomColor0: ParamId = ...
    CustomColor1: ParamId = ...
    CustomColor2: ParamId = ...
    CustomColor3: ParamId = ...
    CustomColor4: ParamId = ...
    CustomColor5: ParamId = ...
    CustomColor6: ParamId = ...
    CustomColor7: ParamId = ...
    CustomColor8: ParamId = ...
    CustomColor9: ParamId = ...
    CustomColor10: ParamId = ...
    CustomColor11: ParamId = ...
    CustomColor12: ParamId = ...
    CustomColor13: ParamId = ...
    CustomColor14: ParamId = ...
    CustomColor15: ParamId = ...
    CustomColor16: ParamId = ...
    CustomColor17: ParamId = ...
    CustomColor18: ParamId = ...
    CustomColor19: ParamId = ...
    CustomFloat2: ParamId = ...
    CustomFloat3: ParamId = ...
    CustomFloat5: ParamId = ...
    CustomFloat7: ParamId = ...
    CustomFloat9: ParamId = ...
    CustomFloat13: ParamId = ...
    CustomFloat14: ParamId = ...
    CustomFloat15: ParamId = ...
    CustomFloat16: ParamId = ...
    CustomInteger0: ParamId = ...
    CustomInteger1: ParamId = ...
    CustomInteger2: ParamId = ...
    CustomInteger3: ParamId = ...
    CustomInteger4: ParamId = ...
    CustomInteger5: ParamId = ...
    CustomInteger6: ParamId = ...
    CustomInteger7: ParamId = ...
    CustomInteger8: ParamId = ...
    CustomInteger9: ParamId = ...
    CustomInteger10: ParamId = ...
    CustomInteger11: ParamId = ...
    CustomInteger12: ParamId = ...
    CustomInteger13: ParamId = ...
    CustomInteger14: ParamId = ...
    CustomInteger15: ParamId = ...
    CustomInteger16: ParamId = ...
    CustomInteger17: ParamId = ...
    CustomInteger18: ParamId = ...
    CustomInteger19: ParamId = ...
    CustomBoolean13: ParamId = ...
    CustomBoolean14: ParamId = ...
    CustomBoolean15: ParamId = ...
    CustomBoolean16: ParamId = ...
    CustomBoolean17: ParamId = ...
    CustomBoolean18: ParamId = ...
    CustomBoolean19: ParamId = ...
    UvTransform0: ParamId = ...
    UvTransform1: ParamId = ...
    UvTransform2: ParamId = ...
    UvTransform3: ParamId = ...
    UvTransform4: ParamId = ...
    UvTransform5: ParamId = ...
    UvTransform6: ParamId = ...
    UvTransform7: ParamId = ...
    UvTransform8: ParamId = ...
    UvTransform9: ParamId = ...
    UvTransform10: ParamId = ...
    UvTransform11: ParamId = ...
    UvTransform12: ParamId = ...
    UvTransform13: ParamId = ...
    UvTransform14: ParamId = ...
    UvTransform15: ParamId = ...
    DiffuseUvTransform1: ParamId = ...
    DiffuseUvTransform2: ParamId = ...
    SpecularUvTransform1: ParamId = ...
    SpecularUvTransform2: ParamId = ...
    NormalUvTransform1: ParamId = ...
    NormalUvTransform2: ParamId = ...
    DiffuseUvTransform: ParamId = ...
    SpecularUvTransform: ParamId = ...
    NormalUvTransform: ParamId = ...
    UseDiffuseUvTransform: ParamId = ...
    UseSpecularUvTransform: ParamId = ...
    UseNormalUvTransform: ParamId = ...
    BlendState1: ParamId = ...
    BlendState2: ParamId = ...
    BlendState3: ParamId = ...
    BlendState4: ParamId = ...
    BlendState5: ParamId = ...
    BlendState6: ParamId = ...
    BlendState7: ParamId = ...
    BlendState8: ParamId = ...
    BlendState9: ParamId = ...
    BlendState10: ParamId = ...
    RasterizerState1: ParamId = ...
    RasterizerState2: ParamId = ...
    RasterizerState3: ParamId = ...
    RasterizerState4: ParamId = ...
    RasterizerState5: ParamId = ...
    RasterizerState6: ParamId = ...
    RasterizerState7: ParamId = ...
    RasterizerState8: ParamId = ...
    RasterizerState9: ParamId = ...
    RasterizerState10: ParamId = ...
    ShadowColor: ParamId = ...
    EmissiveMapLayer1: ParamId = ...
    EmissiveMapLayer2: ParamId = ...
    AlphaTestFunc: ParamId = ...
    AlphaTestRef: ParamId = ...
    Texture17: ParamId = ...
    Texture18: ParamId = ...
    Texture19: ParamId = ...
    Sampler16: ParamId = ...
    Sampler17: ParamId = ...
    Sampler18: ParamId = ...
    Sampler19: ParamId = ...
    CustomVector25: ParamId = ...
    CustomVector26: ParamId = ...
    CustomVector28: ParamId = ...
    CustomVector36: ParamId = ...
    CustomVector41: ParamId = ...
    CustomVector48: ParamId = ...
    CustomVector49: ParamId = ...
    CustomVector50: ParamId = ...
    CustomVector51: ParamId = ...
    CustomVector52: ParamId = ...
    CustomVector53: ParamId = ...
    CustomVector54: ParamId = ...
    CustomVector55: ParamId = ...
    CustomVector56: ParamId = ...
    CustomVector57: ParamId = ...
    CustomVector58: ParamId = ...
    CustomVector59: ParamId = ...
    CustomVector60: ParamId = ...
    CustomVector61: ParamId = ...
    CustomVector62: ParamId = ...
    CustomVector63: ParamId = ...
    UseBaseColorMap: ParamId = ...
    UseMetallicMap: ParamId = ...
    BaseColorMap: ParamId = ...
    BaseColorMapLayer1: ParamId = ...
    MetallicMap: ParamId = ...
    MetallicMapLayer1: ParamId = ...
    DiffuseLightingAoOffset: ParamId = ...


class FillMode:
    name: str
    value: int

    Line: FillMode = ...
    Solid: FillMode = ...


class CullMode:
    name: str
    value: int

    Back: CullMode = ...
    Front: CullMode = ...
    None: CullMode = ...


class BlendFactor:
    name: str
    value: int

    Zero: BlendFactor = ...
    One: BlendFactor = ...
    SourceAlpha: BlendFactor = ...
    DestinationAlpha: BlendFactor = ...
    SourceColor: BlendFactor = ...
    DestinationColor: BlendFactor = ...
    OneMinusSourceAlpha: BlendFactor = ...
    OneMinusDestinationAlpha: BlendFactor = ...
    OneMinusSourceColor: BlendFactor = ...
    OneMinusDestinationColor: BlendFactor = ...
    SourceAlphaSaturate: BlendFactor = ...


class WrapMode:
    name: str
    value: int

    Repeat: WrapMode = ...
    ClampToEdge: WrapMode = ...
    MirroredRepeat: WrapMode = ...
    ClampToBorder: WrapMode = ...


class MinFilter:
    name: str
    value: int

    Nearest: MinFilter = ...
    LinearMipmapLinear: MinFilter = ...
    LinearMipmapLinear2: MinFilter = ...


class MagFilter:
    name: str
    value: int

    Nearest: MagFilter = ...
    Linear: MagFilter = ...
    Linear2: MagFilter = ...


class MaxAnisotropy:
    name: str
    value: int

    One: MaxAnisotropy = ...
    Two: MaxAnisotropy = ...
    Four: MaxAnisotropy = ...
    Eight: MaxAnisotropy = ...
    Sixteen: MaxAnisotropy = ...

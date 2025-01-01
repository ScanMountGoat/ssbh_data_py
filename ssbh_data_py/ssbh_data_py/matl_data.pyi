# File automatically generated by build.rs.
# Changes made to this file will not be saved.
from typing import List, Tuple, Any, Optional, Union, ClassVar
import numpy


def read_matl(path: str) -> MatlData: ...


class MatlData:
    major_version: int
    minor_version: int
    entries: list[MatlEntryData]

    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 6,
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
    uv_transforms: list[UvTransformParam]

    def __init__(
        self,
        material_label: str,
        shader_label: str,
        blend_states: list[BlendStateParam] = [],
        floats: list[FloatParam] = [],
        booleans: list[BooleanParam] = [],
        vectors: list[Vector4Param] = [],
        rasterizer_states: list[RasterizerStateParam] = [],
        samplers: list[SamplerParam] = [],
        textures: list[TextureParam] = [],
        uv_transforms: list[UvTransformParam] = []
    ) -> None: ...


class BlendStateParam:
    param_id: ParamId
    data: BlendStateData

    def __init__(
        self,
        param_id: ParamId,
        data: BlendStateData,
    ) -> None: ...


class FloatParam:
    param_id: ParamId
    data: float

    def __init__(
        self,
        param_id: ParamId,
        data: float,
    ) -> None: ...


class BooleanParam:
    param_id: ParamId
    data: bool

    def __init__(
        self,
        param_id: ParamId,
        data: bool,
    ) -> None: ...


class Vector4Param:
    param_id: ParamId
    data: list[float]

    def __init__(
        self,
        param_id: ParamId,
        data: list[Any],
    ) -> None: ...


class RasterizerStateParam:
    param_id: ParamId
    data: RasterizerStateData

    def __init__(
        self,
        param_id: ParamId,
        data: RasterizerStateData,
    ) -> None: ...


class SamplerParam:
    param_id: ParamId
    data: SamplerData

    def __init__(
        self,
        param_id: ParamId,
        data: SamplerData,
    ) -> None: ...


class TextureParam:
    param_id: ParamId
    data: str

    def __init__(
        self,
        param_id: ParamId,
        data: str,
    ) -> None: ...


class UvTransformParam:
    param_id: ParamId
    data: UvTransform

    def __init__(
        self,
        param_id: ParamId,
        data: UvTransform,
    ) -> None: ...


class UvTransform:
    scale_u: float
    scale_v: float
    rotation: float
    translate_u: float
    translate_v: float

    def __init__(
        self,
        scale_u: float,
        scale_v: float,
        rotation: float,
        translate_u: float,
        translate_v: float
    ) -> None: ...


class BlendStateData:
    source_color: BlendFactor
    destination_color: BlendFactor
    alpha_sample_to_coverage: bool

    def __init__(
        self,
        source_color: BlendFactor = BlendFactor.One,
        destination_color: BlendFactor = BlendFactor.Zero,
        alpha_sample_to_coverage: bool = False
    ) -> None: ...


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

    BlendState0: ClassVar[ParamId]
    RasterizerState0: ClassVar[ParamId]
    CustomVector8: ClassVar[ParamId]
    Texture4: ClassVar[ParamId]
    CustomVector0: ClassVar[ParamId]
    CustomBoolean1: ClassVar[ParamId]
    CustomVector13: ClassVar[ParamId]
    CustomBoolean3: ClassVar[ParamId]
    CustomBoolean4: ClassVar[ParamId]
    Texture7: ClassVar[ParamId]
    CustomVector14: ClassVar[ParamId]
    CustomFloat8: ClassVar[ParamId]
    Texture0: ClassVar[ParamId]
    Texture6: ClassVar[ParamId]
    CustomVector3: ClassVar[ParamId]
    Texture5: ClassVar[ParamId]
    CustomVector30: ClassVar[ParamId]
    CustomBoolean2: ClassVar[ParamId]
    CustomVector31: ClassVar[ParamId]
    CustomBoolean11: ClassVar[ParamId]
    Texture14: ClassVar[ParamId]
    CustomVector27: ClassVar[ParamId]
    Texture9: ClassVar[ParamId]
    CustomVector29: ClassVar[ParamId]
    CustomVector6: ClassVar[ParamId]
    CustomVector11: ClassVar[ParamId]
    CustomBoolean5: ClassVar[ParamId]
    CustomBoolean12: ClassVar[ParamId]
    CustomBoolean6: ClassVar[ParamId]
    Texture2: ClassVar[ParamId]
    Texture1: ClassVar[ParamId]
    CustomVector7: ClassVar[ParamId]
    CustomFloat1: ClassVar[ParamId]
    Texture3: ClassVar[ParamId]
    CustomFloat19: ClassVar[ParamId]
    CustomVector18: ClassVar[ParamId]
    CustomBoolean9: ClassVar[ParamId]
    CustomVector42: ClassVar[ParamId]
    CustomVector32: ClassVar[ParamId]
    CustomBoolean7: ClassVar[ParamId]
    CustomFloat4: ClassVar[ParamId]
    CustomFloat10: ClassVar[ParamId]
    Texture11: ClassVar[ParamId]
    Texture16: ClassVar[ParamId]
    CustomVector47: ClassVar[ParamId]
    Texture10: ClassVar[ParamId]
    CustomVector34: ClassVar[ParamId]
    CustomFloat11: ClassVar[ParamId]
    CustomFloat12: ClassVar[ParamId]
    CustomVector35: ClassVar[ParamId]
    CustomFloat6: ClassVar[ParamId]
    CustomFloat18: ClassVar[ParamId]
    CustomVector37: ClassVar[ParamId]
    CustomVector38: ClassVar[ParamId]
    CustomVector39: ClassVar[ParamId]
    CustomVector19: ClassVar[ParamId]
    CustomVector23: ClassVar[ParamId]
    Texture13: ClassVar[ParamId]
    CustomVector21: ClassVar[ParamId]
    CustomBoolean0: ClassVar[ParamId]
    CustomVector20: ClassVar[ParamId]
    CustomBoolean10: ClassVar[ParamId]
    CustomVector40: ClassVar[ParamId]
    Texture12: ClassVar[ParamId]
    CustomVector22: ClassVar[ParamId]
    Texture8: ClassVar[ParamId]
    CustomVector46: ClassVar[ParamId]
    CustomFloat17: ClassVar[ParamId]
    CustomVector24: ClassVar[ParamId]
    CustomBoolean8: ClassVar[ParamId]
    CustomVector33: ClassVar[ParamId]
    CustomVector4: ClassVar[ParamId]
    CustomFloat0: ClassVar[ParamId]
    CustomVector1: ClassVar[ParamId]
    CustomVector2: ClassVar[ParamId]
    CustomVector5: ClassVar[ParamId]
    CustomVector15: ClassVar[ParamId]
    CustomVector16: ClassVar[ParamId]
    CustomVector43: ClassVar[ParamId]
    CustomVector44: ClassVar[ParamId]
    CustomVector45: ClassVar[ParamId]
    CustomVector9: ClassVar[ParamId]
    CustomVector10: ClassVar[ParamId]
    Diffuse: ClassVar[ParamId]
    Specular: ClassVar[ParamId]
    Ambient: ClassVar[ParamId]
    BlendMap: ClassVar[ParamId]
    Transparency: ClassVar[ParamId]
    DiffuseMapLayer1: ClassVar[ParamId]
    CosinePower: ClassVar[ParamId]
    SpecularPower: ClassVar[ParamId]
    Fresnel: ClassVar[ParamId]
    Roughness: ClassVar[ParamId]
    EmissiveScale: ClassVar[ParamId]
    EnableDiffuse: ClassVar[ParamId]
    EnableSpecular: ClassVar[ParamId]
    EnableAmbient: ClassVar[ParamId]
    DiffuseMapLayer2: ClassVar[ParamId]
    EnableTransparency: ClassVar[ParamId]
    EnableOpacity: ClassVar[ParamId]
    EnableCosinePower: ClassVar[ParamId]
    EnableSpecularPower: ClassVar[ParamId]
    EnableFresnel: ClassVar[ParamId]
    EnableRoughness: ClassVar[ParamId]
    EnableEmissiveScale: ClassVar[ParamId]
    WorldMatrix: ClassVar[ParamId]
    ViewMatrix: ClassVar[ParamId]
    ProjectionMatrix: ClassVar[ParamId]
    WorldViewMatrix: ClassVar[ParamId]
    ViewInverseMatrix: ClassVar[ParamId]
    ViewProjectionMatrix: ClassVar[ParamId]
    WorldViewProjectionMatrix: ClassVar[ParamId]
    WorldInverseTransposeMatrix: ClassVar[ParamId]
    DiffuseMap: ClassVar[ParamId]
    SpecularMap: ClassVar[ParamId]
    AmbientMap: ClassVar[ParamId]
    EmissiveMap: ClassVar[ParamId]
    SpecularMapLayer1: ClassVar[ParamId]
    TransparencyMap: ClassVar[ParamId]
    NormalMap: ClassVar[ParamId]
    DiffuseCubeMap: ClassVar[ParamId]
    ReflectionMap: ClassVar[ParamId]
    ReflectionCubeMap: ClassVar[ParamId]
    RefractionMap: ClassVar[ParamId]
    AmbientOcclusionMap: ClassVar[ParamId]
    LightMap: ClassVar[ParamId]
    AnisotropicMap: ClassVar[ParamId]
    RoughnessMap: ClassVar[ParamId]
    ReflectionMask: ClassVar[ParamId]
    OpacityMask: ClassVar[ParamId]
    UseDiffuseMap: ClassVar[ParamId]
    UseSpecularMap: ClassVar[ParamId]
    UseAmbientMap: ClassVar[ParamId]
    UseEmissiveMap: ClassVar[ParamId]
    UseTranslucencyMap: ClassVar[ParamId]
    UseTransparencyMap: ClassVar[ParamId]
    UseNormalMap: ClassVar[ParamId]
    UseDiffuseCubeMap: ClassVar[ParamId]
    UseReflectionMap: ClassVar[ParamId]
    UseReflectionCubeMap: ClassVar[ParamId]
    UseRefractionMap: ClassVar[ParamId]
    UseAmbientOcclusionMap: ClassVar[ParamId]
    UseLightMap: ClassVar[ParamId]
    UseAnisotropicMap: ClassVar[ParamId]
    UseRoughnessMap: ClassVar[ParamId]
    UseReflectionMask: ClassVar[ParamId]
    UseOpacityMask: ClassVar[ParamId]
    DiffuseSampler: ClassVar[ParamId]
    SpecularSampler: ClassVar[ParamId]
    NormalSampler: ClassVar[ParamId]
    ReflectionSampler: ClassVar[ParamId]
    SpecularMapLayer2: ClassVar[ParamId]
    NormalMapLayer1: ClassVar[ParamId]
    NormalMapBc5: ClassVar[ParamId]
    NormalMapLayer2: ClassVar[ParamId]
    RoughnessMapLayer1: ClassVar[ParamId]
    RoughnessMapLayer2: ClassVar[ParamId]
    UseDiffuseUvTransform1: ClassVar[ParamId]
    UseDiffuseUvTransform2: ClassVar[ParamId]
    UseSpecularUvTransform1: ClassVar[ParamId]
    UseSpecularUvTransform2: ClassVar[ParamId]
    UseNormalUvTransform1: ClassVar[ParamId]
    UseNormalUvTransform2: ClassVar[ParamId]
    ShadowDepthBias: ClassVar[ParamId]
    ShadowMap0: ClassVar[ParamId]
    ShadowMap1: ClassVar[ParamId]
    ShadowMap2: ClassVar[ParamId]
    ShadowMap3: ClassVar[ParamId]
    ShadowMap4: ClassVar[ParamId]
    ShadowMap5: ClassVar[ParamId]
    ShadowMap6: ClassVar[ParamId]
    ShadowMap7: ClassVar[ParamId]
    CastShadow: ClassVar[ParamId]
    ReceiveShadow: ClassVar[ParamId]
    ShadowMapSampler: ClassVar[ParamId]
    Texture15: ClassVar[ParamId]
    Sampler0: ClassVar[ParamId]
    Sampler1: ClassVar[ParamId]
    Sampler2: ClassVar[ParamId]
    Sampler3: ClassVar[ParamId]
    Sampler4: ClassVar[ParamId]
    Sampler5: ClassVar[ParamId]
    Sampler6: ClassVar[ParamId]
    Sampler7: ClassVar[ParamId]
    Sampler8: ClassVar[ParamId]
    Sampler9: ClassVar[ParamId]
    Sampler10: ClassVar[ParamId]
    Sampler11: ClassVar[ParamId]
    Sampler12: ClassVar[ParamId]
    Sampler13: ClassVar[ParamId]
    Sampler14: ClassVar[ParamId]
    Sampler15: ClassVar[ParamId]
    CustomBuffer0: ClassVar[ParamId]
    CustomBuffer1: ClassVar[ParamId]
    CustomBuffer2: ClassVar[ParamId]
    CustomBuffer3: ClassVar[ParamId]
    CustomBuffer4: ClassVar[ParamId]
    CustomBuffer5: ClassVar[ParamId]
    CustomBuffer6: ClassVar[ParamId]
    CustomBuffer7: ClassVar[ParamId]
    CustomMatrix0: ClassVar[ParamId]
    CustomMatrix1: ClassVar[ParamId]
    CustomMatrix2: ClassVar[ParamId]
    CustomMatrix3: ClassVar[ParamId]
    CustomMatrix4: ClassVar[ParamId]
    CustomMatrix5: ClassVar[ParamId]
    CustomMatrix6: ClassVar[ParamId]
    CustomMatrix7: ClassVar[ParamId]
    CustomMatrix8: ClassVar[ParamId]
    CustomMatrix9: ClassVar[ParamId]
    CustomMatrix10: ClassVar[ParamId]
    CustomMatrix11: ClassVar[ParamId]
    CustomMatrix12: ClassVar[ParamId]
    CustomMatrix13: ClassVar[ParamId]
    CustomMatrix14: ClassVar[ParamId]
    CustomMatrix15: ClassVar[ParamId]
    CustomMatrix16: ClassVar[ParamId]
    CustomMatrix17: ClassVar[ParamId]
    CustomMatrix18: ClassVar[ParamId]
    CustomMatrix19: ClassVar[ParamId]
    CustomVector12: ClassVar[ParamId]
    CustomVector17: ClassVar[ParamId]
    CustomColor0: ClassVar[ParamId]
    CustomColor1: ClassVar[ParamId]
    CustomColor2: ClassVar[ParamId]
    CustomColor3: ClassVar[ParamId]
    CustomColor4: ClassVar[ParamId]
    CustomColor5: ClassVar[ParamId]
    CustomColor6: ClassVar[ParamId]
    CustomColor7: ClassVar[ParamId]
    CustomColor8: ClassVar[ParamId]
    CustomColor9: ClassVar[ParamId]
    CustomColor10: ClassVar[ParamId]
    CustomColor11: ClassVar[ParamId]
    CustomColor12: ClassVar[ParamId]
    CustomColor13: ClassVar[ParamId]
    CustomColor14: ClassVar[ParamId]
    CustomColor15: ClassVar[ParamId]
    CustomColor16: ClassVar[ParamId]
    CustomColor17: ClassVar[ParamId]
    CustomColor18: ClassVar[ParamId]
    CustomColor19: ClassVar[ParamId]
    CustomFloat2: ClassVar[ParamId]
    CustomFloat3: ClassVar[ParamId]
    CustomFloat5: ClassVar[ParamId]
    CustomFloat7: ClassVar[ParamId]
    CustomFloat9: ClassVar[ParamId]
    CustomFloat13: ClassVar[ParamId]
    CustomFloat14: ClassVar[ParamId]
    CustomFloat15: ClassVar[ParamId]
    CustomFloat16: ClassVar[ParamId]
    CustomInteger0: ClassVar[ParamId]
    CustomInteger1: ClassVar[ParamId]
    CustomInteger2: ClassVar[ParamId]
    CustomInteger3: ClassVar[ParamId]
    CustomInteger4: ClassVar[ParamId]
    CustomInteger5: ClassVar[ParamId]
    CustomInteger6: ClassVar[ParamId]
    CustomInteger7: ClassVar[ParamId]
    CustomInteger8: ClassVar[ParamId]
    CustomInteger9: ClassVar[ParamId]
    CustomInteger10: ClassVar[ParamId]
    CustomInteger11: ClassVar[ParamId]
    CustomInteger12: ClassVar[ParamId]
    CustomInteger13: ClassVar[ParamId]
    CustomInteger14: ClassVar[ParamId]
    CustomInteger15: ClassVar[ParamId]
    CustomInteger16: ClassVar[ParamId]
    CustomInteger17: ClassVar[ParamId]
    CustomInteger18: ClassVar[ParamId]
    CustomInteger19: ClassVar[ParamId]
    CustomBoolean13: ClassVar[ParamId]
    CustomBoolean14: ClassVar[ParamId]
    CustomBoolean15: ClassVar[ParamId]
    CustomBoolean16: ClassVar[ParamId]
    CustomBoolean17: ClassVar[ParamId]
    CustomBoolean18: ClassVar[ParamId]
    CustomBoolean19: ClassVar[ParamId]
    UvTransform0: ClassVar[ParamId]
    UvTransform1: ClassVar[ParamId]
    UvTransform2: ClassVar[ParamId]
    UvTransform3: ClassVar[ParamId]
    UvTransform4: ClassVar[ParamId]
    UvTransform5: ClassVar[ParamId]
    UvTransform6: ClassVar[ParamId]
    UvTransform7: ClassVar[ParamId]
    UvTransform8: ClassVar[ParamId]
    UvTransform9: ClassVar[ParamId]
    UvTransform10: ClassVar[ParamId]
    UvTransform11: ClassVar[ParamId]
    UvTransform12: ClassVar[ParamId]
    UvTransform13: ClassVar[ParamId]
    UvTransform14: ClassVar[ParamId]
    UvTransform15: ClassVar[ParamId]
    DiffuseUvTransform1: ClassVar[ParamId]
    DiffuseUvTransform2: ClassVar[ParamId]
    SpecularUvTransform1: ClassVar[ParamId]
    SpecularUvTransform2: ClassVar[ParamId]
    NormalUvTransform1: ClassVar[ParamId]
    NormalUvTransform2: ClassVar[ParamId]
    DiffuseUvTransform: ClassVar[ParamId]
    SpecularUvTransform: ClassVar[ParamId]
    NormalUvTransform: ClassVar[ParamId]
    UseDiffuseUvTransform: ClassVar[ParamId]
    UseSpecularUvTransform: ClassVar[ParamId]
    UseNormalUvTransform: ClassVar[ParamId]
    BlendState1: ClassVar[ParamId]
    BlendState2: ClassVar[ParamId]
    BlendState3: ClassVar[ParamId]
    BlendState4: ClassVar[ParamId]
    BlendState5: ClassVar[ParamId]
    BlendState6: ClassVar[ParamId]
    BlendState7: ClassVar[ParamId]
    BlendState8: ClassVar[ParamId]
    BlendState9: ClassVar[ParamId]
    BlendState10: ClassVar[ParamId]
    RasterizerState1: ClassVar[ParamId]
    RasterizerState2: ClassVar[ParamId]
    RasterizerState3: ClassVar[ParamId]
    RasterizerState4: ClassVar[ParamId]
    RasterizerState5: ClassVar[ParamId]
    RasterizerState6: ClassVar[ParamId]
    RasterizerState7: ClassVar[ParamId]
    RasterizerState8: ClassVar[ParamId]
    RasterizerState9: ClassVar[ParamId]
    RasterizerState10: ClassVar[ParamId]
    ShadowColor: ClassVar[ParamId]
    EmissiveMapLayer1: ClassVar[ParamId]
    EmissiveMapLayer2: ClassVar[ParamId]
    AlphaTestFunc: ClassVar[ParamId]
    AlphaTestRef: ClassVar[ParamId]
    Texture17: ClassVar[ParamId]
    Texture18: ClassVar[ParamId]
    Texture19: ClassVar[ParamId]
    Sampler16: ClassVar[ParamId]
    Sampler17: ClassVar[ParamId]
    Sampler18: ClassVar[ParamId]
    Sampler19: ClassVar[ParamId]
    CustomVector25: ClassVar[ParamId]
    CustomVector26: ClassVar[ParamId]
    CustomVector28: ClassVar[ParamId]
    CustomVector36: ClassVar[ParamId]
    CustomVector41: ClassVar[ParamId]
    CustomVector48: ClassVar[ParamId]
    CustomVector49: ClassVar[ParamId]
    CustomVector50: ClassVar[ParamId]
    CustomVector51: ClassVar[ParamId]
    CustomVector52: ClassVar[ParamId]
    CustomVector53: ClassVar[ParamId]
    CustomVector54: ClassVar[ParamId]
    CustomVector55: ClassVar[ParamId]
    CustomVector56: ClassVar[ParamId]
    CustomVector57: ClassVar[ParamId]
    CustomVector58: ClassVar[ParamId]
    CustomVector59: ClassVar[ParamId]
    CustomVector60: ClassVar[ParamId]
    CustomVector61: ClassVar[ParamId]
    CustomVector62: ClassVar[ParamId]
    CustomVector63: ClassVar[ParamId]
    UseBaseColorMap: ClassVar[ParamId]
    UseMetallicMap: ClassVar[ParamId]
    BaseColorMap: ClassVar[ParamId]
    BaseColorMapLayer1: ClassVar[ParamId]
    MetallicMap: ClassVar[ParamId]
    MetallicMapLayer1: ClassVar[ParamId]
    DiffuseLightingAoOffset: ClassVar[ParamId]

    @staticmethod
    def from_value(value: int) -> Optional[ParamId]: ...

    @staticmethod
    def from_str(value: str) -> Optional[ParamId]: ...


class FillMode:
    name: str
    value: int

    Line: ClassVar[FillMode]
    Solid: ClassVar[FillMode]

    @staticmethod
    def from_value(value: int) -> Optional[FillMode]: ...

    @staticmethod
    def from_str(value: str) -> Optional[FillMode]: ...


class CullMode:
    name: str
    value: int

    Back: ClassVar[CullMode]
    Front: ClassVar[CullMode]
    Disabled: ClassVar[CullMode]

    @staticmethod
    def from_value(value: int) -> Optional[CullMode]: ...

    @staticmethod
    def from_str(value: str) -> Optional[CullMode]: ...


class BlendFactor:
    name: str
    value: int

    Zero: ClassVar[BlendFactor]
    One: ClassVar[BlendFactor]
    SourceAlpha: ClassVar[BlendFactor]
    DestinationAlpha: ClassVar[BlendFactor]
    SourceColor: ClassVar[BlendFactor]
    DestinationColor: ClassVar[BlendFactor]
    OneMinusSourceAlpha: ClassVar[BlendFactor]
    OneMinusDestinationAlpha: ClassVar[BlendFactor]
    OneMinusSourceColor: ClassVar[BlendFactor]
    OneMinusDestinationColor: ClassVar[BlendFactor]
    SourceAlphaSaturate: ClassVar[BlendFactor]

    @staticmethod
    def from_value(value: int) -> Optional[BlendFactor]: ...

    @staticmethod
    def from_str(value: str) -> Optional[BlendFactor]: ...


class WrapMode:
    name: str
    value: int

    Repeat: ClassVar[WrapMode]
    ClampToEdge: ClassVar[WrapMode]
    MirroredRepeat: ClassVar[WrapMode]
    ClampToBorder: ClassVar[WrapMode]

    @staticmethod
    def from_value(value: int) -> Optional[WrapMode]: ...

    @staticmethod
    def from_str(value: str) -> Optional[WrapMode]: ...


class MinFilter:
    name: str
    value: int

    Nearest: ClassVar[MinFilter]
    LinearMipmapLinear: ClassVar[MinFilter]
    LinearMipmapLinear2: ClassVar[MinFilter]

    @staticmethod
    def from_value(value: int) -> Optional[MinFilter]: ...

    @staticmethod
    def from_str(value: str) -> Optional[MinFilter]: ...


class MagFilter:
    name: str
    value: int

    Nearest: ClassVar[MagFilter]
    Linear: ClassVar[MagFilter]
    Linear2: ClassVar[MagFilter]

    @staticmethod
    def from_value(value: int) -> Optional[MagFilter]: ...

    @staticmethod
    def from_str(value: str) -> Optional[MagFilter]: ...


class MaxAnisotropy:
    name: str
    value: int

    One: ClassVar[MaxAnisotropy]
    Two: ClassVar[MaxAnisotropy]
    Four: ClassVar[MaxAnisotropy]
    Eight: ClassVar[MaxAnisotropy]
    Sixteen: ClassVar[MaxAnisotropy]

    @staticmethod
    def from_value(value: int) -> Optional[MaxAnisotropy]: ...

    @staticmethod
    def from_str(value: str) -> Optional[MaxAnisotropy]: ...

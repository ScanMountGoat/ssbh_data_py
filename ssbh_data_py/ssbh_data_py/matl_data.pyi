from typing import List, Tuple, Any, Optional


def read_matl(path: str) -> MatlData: ...


class MatlData:
    major_version: int
    minor_version: int
    entries: list[MatlEntryData]

    def save(self, path: str) -> None: ...

    def __init__(
        self,
        major_version: int = ...,
        minor_version: int = ...,
    ) -> None: ...


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

    def __init__(
        self,
        material_label: str,
        shader_label: str,
    ) -> None: ...


class ParamId:
    name: str
    value: int


class BlendStateParam:
    param_id: ParamId
    data: BlendStateData


class FloatParam:
    param_id: ParamId
    data: float


class BooleanParam:
    param_id: ParamId
    data: bool


class Vector4Param:
    param_id: ParamId
    data: list[float]


class RasterizerStateParam:
    param_id: ParamId
    data: RasterizerStateData


class SamplerParam:
    param_id: ParamId
    data: SamplerData


class TextureParam:
    param_id: ParamId
    data: str


class BlendStateData:
    source_color: BlendFactor
    destination_color: BlendFactor
    alpha_sample_to_coverage: bool


class RasterizerStateData:
    fill_mode: FillMode
    cull_mode: CullMode
    depth_bias: float


class SamplerData:
    wraps: WrapMode
    wrapt: WrapMode
    wrapr: WrapMode
    min_filter: MinFilter
    mag_filter: MagFilter
    border_color: list[float]
    lod_bias: float
    max_anisotropy: Optional[MaxAnisotropy]


class BlendFactor:
    name: str
    value: int


class WrapMode:
    name: str
    value: int


class MinFilter:
    name: str
    value: int


class MagFilter:
    name: str
    value: int


class MaxAnisotropy:
    name: str
    value: int


class FillMode:
    name: str
    value: int


class CullMode:
    name: str
    value: int

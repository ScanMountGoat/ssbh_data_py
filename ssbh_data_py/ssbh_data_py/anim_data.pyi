# File automatically generated by build.rs.
# Changes made to this file will not be saved.
from typing import List, Tuple, Any, Optional, Union, ClassVar


def read_anim(path: str) -> AnimData: ...


class AnimData:
    major_version: int
    minor_version: int
    groups: list[GroupData]
    final_frame_index: float

    def __init__(
        self,
        major_version: int = 2,
        minor_version: int = 0,
    ) -> None: ...

    def save(self, path: str) -> None: ...


class GroupData:
    group_type: GroupType
    nodes: list[NodeData]

    def __init__(
        self,
        group_type: GroupType,
        nodes: list[NodeData] = []
    ) -> None: ...


class GroupType:
    name: str
    value: int

    Transform: ClassVar[GroupType]
    Visibility: ClassVar[GroupType]
    Material: ClassVar[GroupType]
    Camera: ClassVar[GroupType]

    @staticmethod
    def from_value(value: int) -> Optional[GroupType]: ...

    @staticmethod
    def from_str(value: str) -> Optional[GroupType]: ...


class NodeData:
    name: str
    tracks: list[TrackData]

    def __init__(
        self,
        name: str,
        tracks: list[TrackData] = []
    ) -> None: ...


class TrackData:
    name: str
    values: Union[list[UvTransform], list[Transform],
                  list[float], list[bool], list[int], list[list[float]]]
    scale_options: ScaleOptions
    transform_flags: TransformFlags

    def __init__(
        self,
        name: str,
        values: Union[list[UvTransform], list[Transform],
                  list[float], list[bool], list[int], list[list[float]]] = [],
        scale_options: ScaleOptions = ScaleOptions(),
        transform_flags: TransformFlags = TransformFlags()
    ) -> None: ...


class ScaleOptions:
    inherit_scale: bool
    compensate_scale: bool

    def __init__(
        self,
        inherit_scale: bool = True,
        compensate_scale: bool = False
    ) -> None: ...


class Transform:
    scale: list[float]
    rotation: list[float]
    translation: list[float]

    def __init__(
        self,
        scale: list[float],
        rotation: list[float],
        translation: list[float]
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


class TransformFlags:
    override_translation: bool
    override_rotation: bool
    override_scale: bool

    def __init__(
        self,
        override_translation: bool = False,
        override_rotation: bool = False,
        override_scale: bool = False
    ) -> None: ...

from typing import List, Tuple, Any, Optional, Union


def read_anim(path: str) -> AnimData: ...


class AnimData:
    major_version: int
    minor_version: int
    groups: list[GroupData]

    def __init__(
        self,
        major_version: int = ...,
        minor_version: int = ...,
    ) -> None: ...

    def save(self, path: str) -> None: ...


class GroupData:
    group_type: GroupType
    nodes: list[NodeData]

    def __init__(
        self,
        group_type: GroupType,
    ) -> None: ...


class GroupType:
    name: str
    value: int

    Transform: GroupType = ...
    Visibility: GroupType = ...
    Material: GroupType = ...
    Camera: GroupType = ...


class NodeData:
    name: str
    tracks: list[TrackData]

    def __init__(
        self,
        name: str,
    ) -> None: ...


class TrackData:
    name: str
    values: Union[list[UvTransform], list[Transform],
                  list[float], list[bool], list[int], list[list[float]]]

    def __init__(
        self,
        name: str,
    ) -> None: ...


class Transform:
    scale: list[float]
    rotation: list[float]
    translation: list[float]
    compensate_scale: float

    def __init__(
        self,
        scale: list[float],
        rotation: list[float],
        translation: list[float],
        compensate_scale: float
    ) -> None: ...


class UvTransform:
    unk1: float
    unk2: float
    unk3: float
    unk4: float
    unk5: float

    def __init__(
        self,
        unk1: float,
        unk2: float,
        unk3: float,
        unk4: float,
        unk5: float
    ) -> None: ...

# File automatically generated by build.rs.
# Changes made to this file will not be saved.
from typing import List, Tuple, Any, Optional, Union, ClassVar


def read_mesh(path: str, use_numpy: bool = False) -> MeshData: ...


def transform_points(
    points: list[list[float]], transform: list[list[float]]) -> list[list[float]]: ...


def transform_vectors(
    points: list[list[float]], transform: list[list[float]]) -> list[list[float]]: ...


def calculate_smooth_normals(
    positions: list[list[float]], vertex_indices: list[int]) -> list[list[float]]: ...


def calculate_tangents_vec4(
    positions: list[list[float]], normals: list[list[float]], uvs: list[list[float]], vertex_indices: list[int]) -> list[list[float]]: ...


class MeshData:
    major_version: int
    minor_version: int
    objects: list[MeshObjectData]

    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 10,
    ) -> None: ...

    def save(self, path: str) -> None: ...


class MeshObjectData:
    name: str
    subindex: int
    parent_bone_name: str
    disable_depth_test: bool
    disable_depth_write: bool
    sort_bias: int
    vertex_indices: list[int]
    positions: list[AttributeData]
    normals: list[AttributeData]
    binormals: list[AttributeData]
    tangents: list[AttributeData]
    texture_coordinates: list[AttributeData]
    color_sets: list[AttributeData]
    bone_influences: list[BoneInfluence]

    def __init__(
        self,
        name: str,
        subindex: int,
        parent_bone_name: str = '',
        disable_depth_test: bool = False,
        disable_depth_write: bool = False,
        sort_bias: int = 0,
        vertex_indices: list[int] = [],
        positions: list[AttributeData] = [],
        normals: list[AttributeData] = [],
        binormals: list[AttributeData] = [],
        tangents: list[AttributeData] = [],
        texture_coordinates: list[AttributeData] = [],
        color_sets: list[AttributeData] = [],
        bone_influences: list[BoneInfluence] = []
    ) -> None: ...


class AttributeData:
    name: str
    data: list[list[float]]

    def __init__(
        self,
        name: str,
        data: list[list[float]] = []
    ) -> None: ...


class BoneInfluence:
    bone_name: str
    vertex_weights: list[VertexWeight]

    def __init__(
        self,
        bone_name: str,
        vertex_weights: list[VertexWeight]
    ) -> None: ...


class VertexWeight:
    vertex_index: int
    vertex_weight: float

    def __init__(
        self,
        vertex_index: int,
        vertex_weight: float
    ) -> None: ...

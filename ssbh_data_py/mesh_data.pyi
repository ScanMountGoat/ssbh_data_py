from typing import List, Tuple, Any


def read_mesh(path: str) -> MeshData: ...


class MeshData:
    major_version: int
    minor_version: int
    objects: list[MeshObjectData]
    def save(self, path: str) -> None: ...


class MeshObjectData:
    name: str
    sub_index: int
    parent_bone_name: str
    vertex_indices: list[int]
    positions: list[AttributeData]
    normals: list[AttributeData]
    binormals: list[AttributeData]
    tangents: list[AttributeData]
    texture_coordinates: list[AttributeData]
    color_sets: list[AttributeData]
    bone_influences: list[BoneInfluence]


class AttributeData:
    name: str
    data: list[list[float]]


class BoneInfluence:
    bone_name: str
    vertex_weights: list[VertexWeight]


class VertexWeight:
    vertex_index: int
    vertex_weight: float

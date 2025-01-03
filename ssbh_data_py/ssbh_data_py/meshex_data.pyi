# File automatically generated by build.rs.
# Changes made to this file will not be saved.
from typing import List, Tuple, Any, Optional, Union, ClassVar
import numpy


def read_meshex(path: str) -> MeshExData: ...


class MeshExData:
    mesh_object_groups: list[MeshObjectGroupData]

    def __init__(self) -> None: ...

    @staticmethod
    def from_mesh_objects(objects: list[MeshObjectData]) -> MeshExData: ...
    
    def save(self, path: str) -> None: ...


class MeshObjectGroupData:
    bounding_sphere: BoundingSphere
    mesh_object_name: str
    mesh_object_full_name: str
    entry_flags: list[EntryFlags]

    def __init__(
        self,
        bounding_sphere: BoundingSphere,
        mesh_object_name: str,
        mesh_object_full_name: str,
        entry_flags: list[EntryFlags]
    ) -> None: ...


class EntryFlags:
    draw_model: bool
    cast_shadow: bool

    def __init__(
        self,
        draw_model: bool,
        cast_shadow: bool
    ) -> None: ...


class BoundingSphere:
    center: list[float]
    radius: float

    def __init__(
        self,
        center: list[float],
        radius: float
    ) -> None: ...

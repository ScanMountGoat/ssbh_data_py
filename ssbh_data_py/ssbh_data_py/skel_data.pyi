# File automatically generated by build.rs.
# Changes made to this file will not be saved.
from typing import List, Tuple, Any, Optional, Union


def read_skel(path: str) -> SkelData: ...


def calculate_relative_transform(
    world_transform: list[list[float]],
    parent_world_transform: list[list[float]]) -> list[list[float]]: ...


class SkelData:
    major_version: int
    minor_version: int
    bones: list[BoneData]

    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 0,
    ) -> None: ...

    def save(self, path: str) -> None: ...

    def calculate_world_transform(
        self, bone: BoneData) -> list[list[float]]: ...


class BoneData:
    name: str
    transform: list[list[float]]
    parent_index: Optional[int]

    def __init__(
        self,
        name: str,
        transform: list[list[float]],
        parent_index: Optional[int]
    ) -> None: ...

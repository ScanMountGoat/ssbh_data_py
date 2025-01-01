# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## unreleased
### Changed
* Enabled numpy arrays by default for better performance and removed `use_numpy` argument from all relevant methods.

### Added
- Added support for Python 3.13 for PyPi builds.

## 0.8.4 - 2024-08-01
### Fixed
- Fixed an issue where `ssbh_data_py.mesh_ex_data.EntryFlags` could not be instantiated from Python.

## 0.8.3 - 2024-03-25
### Added
- Added field `ssbh_data_py.matl_data.MatlEntryData.uv_transforms`.
- Added type `ssbh_data_py.matl_data.UvTransformParam`.
- Added type `ssbh_data_py.matl_data.UvTransform`.

### Changed
- Updated ssbh_data to add read and save support for version 1.5 for `ssbh_data_py.matl_data.MatlData`.

## 0.8.2 - 2024-03-11
### Changed
- Updated ssbh_data.

### Added
- Added support for Python 3.11 and Python 3.12 for PyPi builds.

## 0.8.1 - 2023-05-09
### Fixed
- Updated ssbh_data to fix an issue where some compressed anims would not import/export correctly.

## 0.8.0 - 2023-04-26
### Added
- Added field `ssbh_data_py.anim_data.TransformFlags.override_compensate_scale`

### Changed
- Moved field `ssbh_data_py.anim_data.ScaleOptions.compensate_scale` to `ssbh_data_py.anim_data.TrackData.compensate_scale`
- Changed the type of `ssbh_data_py.meshex_data.MeshObjectGroupData.bounding_sphere` to `ssbh_data_py.meshex_data.BoundingSphere`

### Removed
- Removed `ssbh_data_py.anim_data.ScaleOptions`

## 0.7.1 - 2023-10-23
### Added
- Added support for nuhlpb files with the `ssbh_data_py.hlpb_data` submodule.
- Added field `ssbh_data_py.skel_data.BoneData.billboard_type` of type `BillboardType`.
- Added field `ssbh_data_py.anim_data.TrackData.transform_flags` of type `TransformFlags`.

### Changed
- Improved export times when using numpy arrays for mesh attribute data.
- Renamed `ssbh_data_py.MeshExError` to `ssbh_data_py.MeshExDataError`
- Renamed `ssbh_data_py.mesh_data.MeshObjectData.sub_index` to `ssbh_data_py.mesh_data.MeshObjectData.subindex`
- Renamed `ssbh_data_py.modl_data.ModlEntryData.mesh_object_sub_index` to `ssbh_data_py.modl_data.ModlEntryData.mesh_object_subindex`
- Adjusted matrices to use column-major order instead of row-major order.
- Renamed enum variant `ssbh_data_py.matl_data.CullMode.None` to `ssbh_data_py.matl_data.CullMode.Disabled`.

### Fixed
- Fixed comparison operators not working as expected for enum types. Enum variants are equal if their `value` field is equal.
- Fixed an issue where error types couldn't be matched on in exception handling like `except ssbh_data_py.MeshDataError as e:`.

## 0.6.2 - 2022-01-10
### Added
- Added support for numshexb files with the `ssbh_data_py.meshex_data` submodule.
- Added support for installing from PyPi for Apple M1 processors on Python 3.9.

### Changed
- Improved type hints.
- Changed classes to support specifying all fields at construction with clear defaults for optional fields.

### Removed
- Removed `ssbh_data_py.mesh_data.read_mesh_numpy`. Use `read_mesh(..., use_numpy=True)` instead.

## 0.5.3 - 2021-12-28
### Added
- Added `from_str` and `from_value` static methods to enum types.
- Added `ssbh_data_py.mesh_data.read_mesh_numpy`. This is an experimental API and may change in future releases. 

### Changed
- Improved type hints.

## 0.5.2 - 2021-12-16
### Added
- Added support for numatb files with the `ssbh_data_py.matl_data` submodule.

### Changed
- Improved the printed output of ssbh_data_py classes when calling `repr()`, `str()`, or `print()`.

### Removed
- Removed support for Python 3.6.

## 0.5.1 - 2021-11-12
### Added
- Added the `sort_bias`, `disable_depth_write`, and `disable_depth_test` fields to to `MeshObjectData`. These default to `0`, `False`, and `False`.
- Added support for adjb files with the `ssbh_data_py.adj_data` submodule.
- Added scale options to `TrackData`. This is accessible as `track.scale_options.inherit_scale` and 
`track.scale_options.compensate_scale`. 

### Changed
- Types will now display as being a member of their respective module rather than a member of "builtins" when debugging objects.
- Moved the compensate_scale field to `TrackData`. This now applies to all frames in the track.

## 0.4.0 - 2021-10-17
### Added
- Added `ssbh_data_py.mesh_data.calculate_smooth_normals` and `ssbh_data_py.mesh_data.calculate_tangents_vec4` functions from ssbh_data
- Added the final_frame_index field to AnimData. This should be set to `frame_count - 1` or `0` for empty animations.

### Changed
- Fields and function parameters that used to only accept lists of floats or ints now accept sequences such as tuples or numpy arrays. For example, `bone.transform = numpy.zeros((4,4))`, `mesh_object_data.vertex_indices = numpy.arange(12)`, and `attribute_data.data = [(1,0,0), (0,1,0)]` now work as expected without requiring any conversions.
- Improved the printed representation of `ssbh_data_py.anim_data.Transform` and `ssbh_data_py.anim_data.UvTransform`
- Improved the readability of Python exceptions triggered by panics in Rust code. 
- Renamed all UvTransform fields
- Changed the expected type from float to integer for Transform.compensate_scale

### Fixed
- Fixed an issue where the 4th value of `Transform.rotation` would sometimes be set to `NaN`.

## 0.3.4 - 2021-09-05
### Added
- Added type stub files (.pyi). This enables type hints and autocompletion in supported IDEs.

## 0.3.2 - 2021-09-02
### Added
- Added read only anim support in `ssbh_data_py.anim_data`. Exporting is planned for a future release.  

### Fixed
- Fixed import and export of version 1.8 and 1.9 mesh files. This mostly applies to games other than Smash Ultimate.  

## 0.3.0 - 2021-06-13
### Added
- Added `transform_points` and `transform_vectors` functions to `ssbh_data_py.mesh_data`
- Added `calculate_world_transform` method to `ssbh_data_py.skel_data.SkelData`
- Added `calculate_relative_transform` function to `ssbh_data_py.skel_data`
- Added skel support in `ssbh_data_py.skel_data`
- Added modl support in `ssbh_data_py.modl_data`
- Added support for creating new meshes and mesh objects to `ssbh_data_py.mesh_data`

### Changed
- Renamed `ssbh_data_py.mesh_data.Mesh` to `ssbh_data_py.mesh_data.MeshData` 

### Removed
- Removed `ssbh_data_py.skel_data.calculate_single_bind_transform`. Use `ssbh_data_py.skel_data.SkelData.calculate_world_transform` instead.

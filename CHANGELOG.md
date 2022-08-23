### unreleased
Changes:
- Added support for nuhlpb files with the `ssbh_data_py.hlpb_data` submodule.

Breaking Changes:
- Renamed `ssbh_data_py.mesh_data.MeshObjectData.subindex` to `ssbh_data_py.mesh_data.MeshObjectData.subindex`
- Renamed `ssbh_data_py.modl_data.ModlEntryData.mesh_object_subindex` to `ssbh_data_py.modl_data.ModlEntryData.mesh_object_subindex`
- Adjusted matrices to use column-major order instead of row-major order.

### 0.7.0
Changes:
- Fixed comparison operators not working as expected for enum types. Enum variants are equal if their `value` field is equal.
- Fixed an issue where error types couldn't be matched on in in exception handling like `except ssbh_data_py.MeshDataError as e:`.
- Added field `ssbh_data_py.skel_data.BoneData.billboard_type` of type `BillboardType`.
- Added field `ssbh_data_py.anim_data.TrackData.transform_flags` of type `TransformFlags`.

Breaking Changes:
- Renamed `ssbh_data_py.MeshExError` to `ssbh_data_py.MeshExDataError`

### 0.6.2
Changes:
- Added support for numshexb files with the `ssbh_data_py.meshex_data` submodule.
- Improved type hints.
- Changed classes to support specifying all fields at construction with clear defaults for optional fields.
- Added support for installing from PyPi for Apple M1 processors on Python 3.9.

Breaking Changes:
ssbh_data_py.mesh_data
- Removed `ssbh_data_py.mesh_data.read_mesh_numpy`. Use `read_mesh(..., use_numpy=True)` instead.

### 0.5.3
Changes:
- Added `from_str` and `from_value` static methods to enum types.
- Improved type hints.
- Added `ssbh_data_py.mesh_data.read_mesh_numpy`. This is an experimental API and may change in future releases. 

### 0.5.2
Changes:
- Added support for numatb files with the `ssbh_data_py.matl_data` submodule.
- Improved the printed output of ssbh_data_py classes when calling `repr()`, `str()`, or `print()`.
- Removed support for Python 3.6.

### 0.5.1
Changes:
- Added the `sort_bias`, `disable_depth_write`, and `disable_depth_test` fields to to `MeshObjectData`. These default to `0`, `False`, and `False`.
- Types will now display as being a member of their respective module rather than a member of "builtins" when debugging objects.
- Added support for adjb files with the `ssbh_data_py.adj_data` submodule.
- Added scale options to `TrackData`. This is accessible as `track.scale_options.inherit_scale` and 
`track.scale_options.compensate_scale`. 

Breaking Changes:
ssbh_data_py.anim_data
- Moved the compensate_scale field to `TrackData`. This now applies to all frames in the track.

### 0.4.0
Changes:
- Fields and function parameters that used to only accept lists of floats or ints now accept sequences such as tuples or numpy arrays. For example, `bone.transform = numpy.zeros((4,4))`, `mesh_object_data.vertex_indices = numpy.arange(12)`, and `attribute_data.data = [(1,0,0), (0,1,0)]` now work as expected without requiring any conversions.
- Improved the printed representation of `ssbh_data_py.anim_data.Transform` and `ssbh_data_py.anim_data.UvTransform`
- Added `ssbh_data_py.mesh_data.calculate_smooth_normals` and `ssbh_data_py.mesh_data.calculate_tangents_vec4` functions from ssbh_data
- Fixed an issue where the 4th value of `Transform.rotation` would sometimes be set to `NaN`.
- Improved the readability of Python exceptions triggered by panics in Rust code. 

Breaking Changes:
ssbh_data_py.anim_data
- Renamed all UvTransform fields
- Changed the expected type from float to integer for Transform.compensate_scale
- Added the final_frame_index field to AnimData. This should be set to `frame_count - 1` or `0` for empty animations.

### 0.3.4
Changes:
- Added type stub files (.pyi). This enables type hints and autocompletion in supported IDEs.

### 0.3.2
Changes:
- Added read only anim support in `ssbh_data_py.anim_data`. Exporting is planned for a future release.  
- Fixed import and export of version 1.8 and 1.9 mesh files. This mostly applies to games other than Smash Ultimate.  

### 0.3.0
Breaking Changes:
- Renamed `ssbh_data_py.mesh_data.Mesh` to `ssbh_data_py.mesh_data.MeshData` 
- Removed `ssbh_data_py.skel_data.calculate_single_bind_transform`.  
Use `ssbh_data_py.skel_data.SkelData.calculate_world_transform` instead.

Changes:
- Added `transform_points` and `transform_vectors` functions to `ssbh_data_py.mesh_data`
- Added `calculate_world_transform` method to `ssbh_data_py.skel_data.SkelData`
- Added `calculate_relative_transform` function to `ssbh_data_py.skel_data`
- Added skel support in `ssbh_data_py.skel_data`
- Added modl support in `ssbh_data_py.modl_data`
- Added support for creating new meshes and mesh objects to `ssbh_data_py.mesh_data`
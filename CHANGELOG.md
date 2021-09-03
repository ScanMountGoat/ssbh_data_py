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
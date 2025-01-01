# This script creates a mesh with a small cube parented to each of the bones defined in the skeleton file.
# The mesh objects are all single bound, so the cubes will render at the current location of the parent bone.
# Once proper materials are applied using a modl and matl file, the final result acts as a skeleton visualization.
# Usage: python skeleton_mesh.py model.nustkb model.numshb

import sys
import ssbh_data_py
import numpy

input_skel_path = sys.argv[1]
output_mesh_path = sys.argv[2]
output_modl_path = sys.argv[3]

# 3d points for a small cube centered at the origin.
scale = 0.25
points = numpy.array(
    [
        [-scale, -scale, -scale],
        [-scale, -scale, scale],
        [-scale, scale, scale],
        [scale, scale, -scale],
        [-scale, -scale, -scale],
        [-scale, scale, -scale],
        [scale, -scale, scale],
        [-scale, -scale, -scale],
        [scale, -scale, -scale],
        [scale, scale, -scale],
        [scale, -scale, -scale],
        [-scale, -scale, -scale],
        [-scale, -scale, -scale],
        [-scale, scale, scale],
        [-scale, scale, -scale],
        [scale, -scale, scale],
        [-scale, -scale, scale],
        [-scale, -scale, -scale],
        [-scale, scale, scale],
        [-scale, -scale, scale],
        [scale, -scale, scale],
        [scale, scale, scale],
        [scale, -scale, -scale],
        [scale, scale, -scale],
        [scale, -scale, -scale],
        [scale, scale, scale],
        [scale, -scale, scale],
        [scale, scale, scale],
        [scale, scale, -scale],
        [-scale, scale, -scale],
        [scale, scale, scale],
        [-scale, scale, -scale],
        [-scale, scale, scale],
        [scale, scale, scale],
        [-scale, scale, scale],
        [scale, -scale, scale],
    ],
    dtype=numpy.float32,
)

# Collect the bone names from the skel.
skel_data = ssbh_data_py.skel_data.read_skel(input_skel_path)
bone_names = [b.name for b in skel_data.bones]

# Create the model.numshb file.
mesh = ssbh_data_py.mesh_data.MeshData()
for i, bone_name in enumerate(bone_names):
    # Use the bone index as the name.
    # This makes the resulting mesh slightly smaller.
    # The bone names are unique, so we can use 0 for the index.
    mesh_object = ssbh_data_py.mesh_data.MeshObjectData(str(i), 0)

    # Parent the mesh object to the bone, so the object will move with the bone.
    mesh_object.parent_bone_name = bone_name

    # Don't add any vertex weights to ensure the mesh is single bound.
    mesh_object.bone_influences = []

    # Add the vertex position data.
    position0 = ssbh_data_py.mesh_data.AttributeData("Position0")
    position0.data = points
    mesh_object.positions = [position0]

    # Create additional attributes required by Smash Ultimate.
    # Normal and tangent generation is not currently supported by ssbh_data_py.
    normal0 = ssbh_data_py.mesh_data.AttributeData("Normal0")
    normal0.data = numpy.array(
        [[0.5, 0.5, 0.5, 0.5]] * len(points), dtype=numpy.float32
    )
    mesh_object.normals = [normal0]

    tangent0 = ssbh_data_py.mesh_data.AttributeData("Tangent0")
    tangent0.data = numpy.array(
        [[0.5, 0.5, 0.5, 1.0]] * len(points), dtype=numpy.float32
    )
    mesh_object.tangents = [tangent0]

    map1 = ssbh_data_py.mesh_data.AttributeData("map1")
    map1.data = numpy.array([[0.5, 0.5]] * len(points), dtype=numpy.float32)
    mesh_object.texture_coordinates = [map1]

    # Generate an index for each point.
    # This wastes memory by not reusing vertices but is fine for this small example.
    mesh_object.vertex_indices = numpy.arange(len(points), dtype=numpy.uint32)

    mesh.objects.append(mesh_object)

# Save the result.
mesh.save(output_mesh_path)

# Create the model.numdlb.
# The modl.animation_file_name can use the default value of None in most cases.
modl = ssbh_data_py.modl_data.ModlData()
modl.skeleton_file_name = "model.nusktb"
modl.material_file_names = ["model.numatb"]
modl.mesh_file_name = "model.numshb"
modl.entries = []
for i, bone_name in enumerate(bone_names):
    # Use the same naming conventions used for the mesh objects.
    # TODO: Creating custom matl files is not yet supported.
    entry = ssbh_data_py.modl_data.ModlEntryData(str(i), 0, "alp_mario_001")
    modl.entries.append(entry)

# Save the result.
modl.save(output_modl_path)

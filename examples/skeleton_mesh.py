# This script creates a mesh with a small cube parented to each of the bones defined in the skeleton JSON file.
# The mesh objects are all single bound, so the cubes will render at the current location of the parent bone.
# Once proper materials are applied using a modl and matl file, the final result acts as a skeleton visualization.

import sys
import json
import ssbh_data_py

skel_path = sys.argv[1]
mesh_path = sys.argv[2]

# 3d points for a small cube centered at the origin.
scale = 0.25
points = [
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
]

with open(skel_path, 'r') as skel_file:
    # Read the bone names from a JSON exported by ssbh_lib_json.
    # Reading skel data with ssbh_data_py is not currently supported.
    skel = json.loads(skel_file.read())
    bone_names = [b['name'] for b in skel['data']['Skel']['bone_entries']]

    mesh = ssbh_data_py.mesh_data.Mesh()
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
        position0 = ssbh_data_py.mesh_data.AttributeData('Position0')
        position0.data = points
        mesh_object.positions = [position0]

        # Create additional attributes required by Smash Ultimate.
        # Normal and tangent generation is not currently supported by ssbh_data_py.
        normal0 = ssbh_data_py.mesh_data.AttributeData('Normal0')
        normal0.data = [[0.5, 0.5, 0.5, 0.5]] * len(points)
        mesh_object.normals = [normal0]

        tangent0 = ssbh_data_py.mesh_data.AttributeData('Tangent0')
        tangent0.data = [[0.5, 0.5, 0.5, 1.0]] * len(points)
        mesh_object.tangents = [tangent0]

        map1 = ssbh_data_py.mesh_data.AttributeData('map1')
        map1.data = [[0.5, 0.5]] * len(points)
        mesh_object.texture_coordinates = [map1]

        # Generate an index for each point.
        # This wastes memory by not reusing vertices but is fine for this small example.
        mesh_object.vertex_indices = list(range(len(points)))

        mesh.objects.append(mesh_object)

    # Save the result.
    mesh.save(mesh_path)

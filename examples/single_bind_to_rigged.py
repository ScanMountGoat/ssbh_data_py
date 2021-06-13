# This script creates a new mesh with all single bound mesh objects converted to use regular vertex weights.
# Usage: python skeleton_mesh.py input.numshb output.numshb

import sys
import ssbh_data_py

input_mesh_path = sys.argv[1]
output_mesh_path = sys.argv[2]

# Open the model.numshb file.
mesh = ssbh_data_py.mesh_data.read_mesh(input_mesh_path)

for mesh_object in mesh.objects:
    if len(mesh_object.bone_influences) == 0:
        # There are no influences, so the object is bound to a parent bone.
        # Use the parent bone bone to create vertex weights.
        vertex_count = max(mesh_object.vertex_indices)
        vertex_weights = [ssbh_data_py.mesh_data.VertexWeight(i, 1.0) for i in range(vertex_count)]
        influence = ssbh_data_py.mesh_data.BoneInfluence(mesh_object.parent_bone_name, vertex_weights)

        # Adding an influence means the mesh object is no longer single bound to the parent.
        # This means that the mesh object will no longer appear at the same location as the parent bone.
        # TODO: Demonstrate how to transform the vertices so that they appear in the proper place.
        mesh_object.parent_bone_name = ''
        mesh_object.bone_influences.append(influence)

# Save the result.
mesh.save(output_mesh_path)

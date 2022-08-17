# This script creates a new mesh with parent bones converted to vertex skinning.
# Usage: python parent_bone_to_weights.py input.numshb input.nusktb output.numshb

import sys
import ssbh_data_py

mesh_path = sys.argv[1]
skel_path = sys.argv[2]
output_mesh_path = sys.argv[3]

# Open the model.numshb and model.nusktb files.
mesh = ssbh_data_py.mesh_data.read_mesh(mesh_path)
skel = ssbh_data_py.skel_data.read_skel(skel_path)

for mesh_object in mesh.objects:
    # There are no influences, so the object uses a parent bone.
    if len(mesh_object.bone_influences) == 0:
        # Use the parent bone to create vertex weights.
        vertex_count = len(mesh_object.positions[0].data)
        vertex_weights = [ssbh_data_py.mesh_data.VertexWeight(i, 1.0) for i in range(vertex_count)]
        influence = ssbh_data_py.mesh_data.BoneInfluence(mesh_object.parent_bone_name, vertex_weights)

        # Adding an influence means the mesh object is no longer single bound to the parent.
        # This means that the mesh object will no longer appear at the same location as the parent bone.
        mesh_object.bone_influences.append(influence)

        # Find the bone in the skeleton associated with this mesh object.
        parent_bone = None
        for bone in skel.bones:
            if bone.name == mesh_object.parent_bone_name:
                parent_bone = bone
                break

        if parent_bone is not None:
            # Manually apply the parent bone's transformation to the vertices.
            # This simulates the effect of parenting the mesh object to the bone.
            transform = skel.calculate_world_transform(parent_bone)

            # Transform the vertex positions.
            # Use the appropriate transform function to take into account translation.
            for position in mesh_object.positions:
                position.data = ssbh_data_py.mesh_data.transform_points(position.data, transform)

            # The mesh object may rotate, so transform the vectors as well.
            # This ensures the normals point in the correct direction and normal mapping works as expected.
            for normal in mesh_object.normals:
                normal.data = ssbh_data_py.mesh_data.transform_vectors(normal.data, transform)

            for tangent in mesh_object.tangents:
                tangent.data = ssbh_data_py.mesh_data.transform_vectors(tangent.data, transform)

            for binormal in mesh_object.binormals:
                binormal.data = ssbh_data_py.mesh_data.transform_vectors(binormal.data, transform)

        # The parent bone no longer needs to be set.
        mesh_object.parent_bone_name = ''


# Save the result.
mesh.save(output_mesh_path)

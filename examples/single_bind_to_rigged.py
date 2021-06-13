# This script creates a new mesh with all single bound mesh objects converted to use regular vertex weights.
# Usage: python skeleton_mesh.py input.numshb output.numshb

import sys
import ssbh_data_py

# This example requires numpy, which can be installed with pip.
import numpy as np

# Transform vectors or positions by converting to homogeneous coordinates.
# TODO: This could be handled by ssbh_data directly in the future.
def transform_vectors(vectors, transform, w):
    values = np.array(vectors)

    # The transform matrices are 4x4, so the values need 4 components.
    values_vec4 = np.zeros((values.shape[0], 4))
    values_vec4[:, :values.shape[1]] = values[:, :]

    # Set the 4th value to 1.0 or 0.0 for points or vectors.
    values_vec4[:, 3] = w

    transformed_values = values_vec4 @ transform

    return transformed_values[:, :values.shape[1]].tolist()


mesh_path = sys.argv[1]
skel_path = sys.argv[2]
output_mesh_path = sys.argv[3]

# Open the model.numshb and model.nusktb files.
mesh = ssbh_data_py.mesh_data.read_mesh(mesh_path)
skel = ssbh_data_py.skel_data.read_skel(skel_path)

for mesh_object in mesh.objects:
    if len(mesh_object.bone_influences) == 0:
        # There are no influences, so the object is bound to a parent bone.
        # Use the parent bone bone to create vertex weights.
        vertex_count = len(mesh_object.positions[0].data)
        vertex_weights = [ssbh_data_py.mesh_data.VertexWeight(i, 1.0) for i in range(vertex_count)]
        influence = ssbh_data_py.mesh_data.BoneInfluence(mesh_object.parent_bone_name, vertex_weights)

        # Find the bone in the skeleton associated with this mesh object.
        parent_bone = None
        for bone in skel.bones:
            if bone.name == mesh_object.parent_bone_name:
                parent_bone = bone
                break

        # Adding an influence means the mesh object is no longer single bound to the parent.
        # This means that the mesh object will no longer appear at the same location as the parent bone.
        mesh_object.bone_influences.append(influence)

        if parent_bone is not None:
            transform = np.array(skel.calculate_world_transform(parent_bone))

            # Manually apply the parent bone's transformation to the vertices.
            for position in mesh_object.positions:
                position.data = transform_vectors(position.data, transform, 1.0)

            # The mesh object may rotate, so transform the vectors as well.
            # This ensures the normals point in the correct direction and normal mapping works as expected.
            for normal in mesh_object.normals:
                normal.data = transform_vectors(normal.data, transform, 0.0)

            for tangent in mesh_object.tangents:
                tangent.data = transform_vectors(tangent.data, transform, 0.0)

            for binormal in mesh_object.binormals:
                binormal.data = transform_vectors(binormal.data, transform, 0.0)

        # The parent bone no longer needs to be set.
        mesh_object.parent_bone_name = ''


# Save the result.
mesh.save(output_mesh_path)

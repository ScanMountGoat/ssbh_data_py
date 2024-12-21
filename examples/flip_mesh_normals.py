# This script demonstrates how to flip normals and reverse winding order.
# Winding order primarily affects face culling.
# If normals on a model appear "flipped" or inside out, this script may help.
# Ideally, these should both be fixed in a 3D modeling program before exporting.
import ssbh_data_py

# It may not be necessary to reverse both normals and winding order in all cases.
reverse_winding_order = True
flip_normals = False

mesh = ssbh_data_py.mesh_data.read_mesh("model.numshb")
for object in mesh.objects:
    # Reverse the winding order of each triangle face.
    if reverse_winding_order:
        for i in range(0, len(object.vertex_indices), 3):
            [v0, v1, v2] = object.vertex_indices[i : i + 3]
            object.vertex_indices[i : i + 3] = [v0, v2, v1]

    # Flip the normals and related vectors.
    if flip_normals:
        for normal in object.normals:
            normal.data = [[-x, -y, -z, 0.0] for [x, y, z, _] in normal.data]
            # normal.data = [[-x, -y, -z] for [x, y, z] in normal.data] # 3-component normals

        # Some models may also have tangents and binormals (bitangents) to flip as well.
        for binormal in object.binormals:
            binormal.data = [[-x, -y, -z] for [x, y, z] in binormal.data]
            # binormal.data = [[-x, -y, -z, 0.0] for [x, y, z, _] in binormal.data] # 4-component binormals

        for tangent in object.tangents:
            tangent.data = [[-x, -y, -z, w] for [x, y, z, w] in tangent.data]
            # tangent.data = [[-x, -y, -z] for [x, y, z] in tangent.data] # 3-component tangents

mesh.save("model.numshb")

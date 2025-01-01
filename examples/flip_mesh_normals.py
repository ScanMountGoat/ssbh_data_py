# This script demonstrates how to flip normals and reverse winding order.
# Winding order primarily affects face culling.
# If normals on a model appear "flipped" or inside out, this script may help.
# Ideally, these should both be fixed in a 3D modeling program before exporting.
import ssbh_data_py
import numpy

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
            # Omit the 4th component to save 3-component normals
            normal.data = numpy.array(
                [[-x, -y, -z, 0.0] for [x, y, z, _] in normal.data], dtype=numpy.float32
            )

        # Some models may also have tangents and binormals (bitangents) to flip as well.
        for binormal in object.binormals:
            # Omit the 4th component to save 3-component binormals
            binormal.data = numpy.array(
                [[-x, -y, -z] for [x, y, z] in binormal.data], dtype=numpy.float32
            )

        for tangent in object.tangents:
            # Omit the 4th component to save 3-component tangents
            tangent.data = numpy.array(
                [[-x, -y, -z, w] for [x, y, z, w] in tangent.data], dtype=numpy.float32
            )

mesh.save("model.numshb")

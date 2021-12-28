# This script demonstrates how to combine two model.numshb files into a single file.
from ssbh_data_py import mesh_data

mesh_a = mesh_data.read_mesh("model_a.numshb")
mesh_b = mesh_data.read_mesh("model_b.numshb")

# Add all the mesh objects from B to A.
for mesh_b_object in mesh_b.objects:
    mesh_a.objects.append(mesh_b_object)

# ssbh_data_py will recalculate the bounding data and rebuild mesh buffers.
mesh_a.save("model_combined.numshb")
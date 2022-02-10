# This script demonstrates how to combine two model.numshb files and two model.numdlb files into a single file.
# This doesn't account for combining the matl files (.numatb).
import ssbh_data_py

modl_a = ssbh_data_py.modl_data.read_modl("model_a.numdlb")
modl_b = ssbh_data_py.modl_data.read_modl("model_b.numdlb")

mesh_a = ssbh_data_py.mesh_data.read_mesh("model_a.numshb")
mesh_b = ssbh_data_py.mesh_data.read_mesh("model_b.numshb")

# Append all of the mesh entries from B to A.
for b_modl_entry in modl_b.entries:
    modl_a.entries.append(b_modl_entry)

# Append all of the mesh objects from B to A.
for b_mesh_object in mesh_b.objects:
    mesh_a.objects.append(b_mesh_object)

modl_a.save("model_combined.numdlb")

# ssbh_data_py will recalculate the bounding data and rebuild mesh buffers.
mesh_a.save("model_combined.numshb")
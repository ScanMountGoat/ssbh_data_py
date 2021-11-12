import ssbh_data_py

# Read the source mesh file.
mesh = ssbh_data_py.mesh_data.read_mesh("model.numshb")

# Create an entry in the adjb file for each mesh object.
# In practice, only certain mesh objects will need an entry in the adjb.
adj = ssbh_data_py.adj_data.AdjData()
for i, mesh_object in enumerate(mesh.objects):
    adj.entries.append(ssbh_data_py.adj_data.AdjEntryData.from_mesh_object(i, mesh_object))

# Save the new adjb file.
adj.save("model.adjb")

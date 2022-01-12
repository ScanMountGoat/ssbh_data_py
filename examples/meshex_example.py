import ssbh_data_py

# Open the meshex from an .numshexb file.
meshex = ssbh_data_py.meshex_data.read_meshex("model.numshexb")

for group in meshex.mesh_object_groups:
    # Mesh objects with the same name are grouped together.
    # The full name includes ending tags like 'shape' or '_O_VIS'.
    print(f'Full Name: {group.mesh_object_full_name}, Name: {group.mesh_object_name}')

    # Bounding information is stored for the entire group.
    print(f'Bounding: {group.bounding_sphere}')

    # Each mesh object in the group has flags to control rendering.
    print(f'Mesh Object Entry Flags: {group.entry_flags}')
    print(f'Mesh Object Count: {len(group.entry_flags)}')

    print()

# Save any changes made to the meshex.
meshex.save("model.numshexb")

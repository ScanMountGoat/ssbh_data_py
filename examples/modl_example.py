import ssbh_data_py

# Open the modl from a .numdlb file.
modl = ssbh_data_py.modl_data.read_modl("model.numdlb")

for entry in modl.entries:
    # Each modl entry assigns a material to an object from a mesh file.
    mesh_object = f"{entry.mesh_object_name}{entry.mesh_object_subindex}"
    print(f"Mesh: {mesh_object}, Material: {entry.material_label}")

# Save any changes made to the modl.
modl.save("model.numdlb")

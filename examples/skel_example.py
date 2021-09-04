import ssbh_data_py

# Open the skel from a .nusktb file.
skel = ssbh_data_py.skel_data.read_skel("model.nusktb")
for b in skel.bones:
    # Each bone is identified by it's name.
    print(f'Bone: {b.name}')

    # Matrices are in row-major order.
    # The transform is relative to the parent bone's world transform.
    print(f'Transform: {b.transform}')

    # The world transform is the global transform of the bone.
    # This requires walking up the bone heirarchy and thus requires a function call.
    # Single bound mesh objects use the world transform as their transform.
    print(f'World Transform: {skel.calculate_world_transform(b)}')

    # Each bone may optionally have a parent.
    # Root bones like 'Trans' have no parent.
    if b.parent_index is not None:
        print(f'Parent: {skel.bones[b.parent_index].name}')
    else:
        print('Root Bone')

# Save any changes made to the skel.
skel.save("model.nusktb")

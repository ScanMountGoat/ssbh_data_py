import ssbh_data_py

# Open the skel from a .nusktb file.
skel = ssbh_data_py.skel_data.read_skel("model.nusktb")
for b in skel.bones:
    # Each bone is identified by it's name.
    print(f'Bone: {b.name}')

    # The bone's transform is a matrix in row-major order representing 
    # the transform of the bone relative to its parent.
    print(f'Transform: {b.transform}')

    # The world transform is the global transform of the bone.
    # This requires walking up the bone heirarchy and thus requires a function call.
    # Single bound mesh objects use the world transform as their transform.
    print(f'World Transform: {skel.calculate_world_transform(b)}')

    # Each bone may optionally have a parent.
    # Root bones like 'TransN' have no parent.
    if b.parent_index is not None:
        print(f'Parent: {skel.bones[b.parent_index].name}')
    else:
        print('Root Bone')

# Save any changes made to the skel.
skel.save("model.nusktb")
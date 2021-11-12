import ssbh_data_py

# Open the anim from a .nuanmb file.
anim = ssbh_data_py.anim_data.read_anim("model.nuanmb")

# Navigate the heirarchy.
for group in anim.groups:
    print(f'Group: {group.group_type.name}')

    for node in group.nodes:
        print(f'Node: {node.name}')

        for track in node.tracks:
            print(f'Track: {track.name}')

            # Scale settings are stored for the entire track.
            # These values only affect transform tracks.
            print(f'inherit_scale: {track.scale_options.inherit_scale}, compensate_scale: {track.scale_options.compensate_scale}')

            # This API may change slightly in the future.
            # There isn't a reliable way to know the type of the track values.
            # One option is to just assume the type and use a try statement.
            # Some common conventions for Smash Ultimate are given below.
            if track.name == 'Transform':
                # Transform
                v = track.values[0]
                print(v.scale, v.rotation, v.translation)
            elif track.name == 'Visibility' or 'CustomBoolean' in track.name:
                # bool
                print(track.values[0])
            elif 'CustomVector' in track.name:
                # A list of floats of the form [x, y, z, w]
                print(track.values[0])
            elif track.name.endswith('.PatternIndex'):
                # Pattern index uses unsigned integers.
                print(track.values[0])
            elif 'Texture' in track.name:
                # UVTransform
                v = track.values[0]
                print(v.scale_u, v.scale_v, v.rotation, v.translate_u, v.translate_v)
            else:
                print(f'Unknown type: {track.values[0]}')

            print()

# Save any changes made to the anim.
# Anim compression is lossy, so track values may change slightly on export.
# This mostly applies to transform or vector tracks.
anim.save("model.nuanmb")
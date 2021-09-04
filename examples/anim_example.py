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

            # This API may change slightly in the future.
            # There isn't a reliable way to know the type of the track values.
            # One option is to just assume the type and use a try statement.
            # Some common conventions for Smash Ultimate are given below.
            if track.name == 'Transform':
                # Transform
                v = track.values[0]
                print(v.scale, v.rotation, v.translation, v.compensate_scale)
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
                print(v.unk1, v.unk2, v.unk3, v.unk4, v.unk5)
            else:
                print(f'Unknown type: {track.values[0]}')

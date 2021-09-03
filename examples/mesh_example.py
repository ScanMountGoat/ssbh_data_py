import ssbh_data_py

# Open the mesh from a .numshb file.
mesh = ssbh_data_py.mesh_data.read_mesh("model.numshb")
for o in mesh.objects:
    # Print the mesh object's name (ex: 'bodyShape0' or 'bodyShape1')
    # Mesh objects with different names should have a different sub index.
    print(f'{o.name}{o.sub_index}')

    # The name of the bone used for single binding or '' otherwise.
    print(o.parent_bone_name)

    # The number of actual rendered vertices is based on the number of vertex indices.
    print(f'Vertex Count: {len(o.vertex_indices)}')
    # If the mesh has triangle data, the faces can be accessed by looping over the indices.
    for i in range(0, len(o.vertex_indices), 3):
        # Get the vertex indices for each vertex in the triangle.
        v0, v1, v2 = o.vertex_indices[i:i+3]

    # Mesh objects have their vertex data stored in various attributes.
    # If the mesh does not have data for that attribute, the list will be blank. 
    # Print the name of the attribute, which will likely be 'Position0' in this case.
    print(o.positions[0].name)

    # The vectors for the attribute data are stored in lists of floats. 
    # The number of elements in each vector depends on the attribute's data type.
    print(o.positions[0].data[0])

    # There may be multiple attributes of a given type, so iterate over all of them.
    for normal in o.normals:
        print(normal.name)

    # Binormals are used by certain games.
    # For Smash Ultimate, this will be an empty list.
    for binormal in o.binormals:
        print(binormal.name)

    for tangent in o.tangents:
        print(tangent.name)

    for texture_coordinate in o.texture_coordinates:
        print(texture_coordinate.name)
        # An example for how to flip the UVs vertically.
        texture_coordinate.data = [[u, 1.0 - v] for [u, v] in texture_coordinate.data]

    # Color sets are accessed as floating point values in the range 0.0 (black) to 0.5 (white).
    for color_set in o.color_sets:
        print(color_set.name)

        # Set the vertex color RGB to white but keep the original alpha.
        # Note that looping over the elements individually can be slow.
        # List comprehension will be faster in many cases.
        color_set.data = [[0.5, 0.5, 0.5, a] for [r, g, b, a] in color_set.data]

    # Add a new color set attribute with the appropriate number of data elements.
    # The process is similar for for adding positions, normals, etc. 
    new_color_set = ssbh_data_py.mesh_data.AttributeData('colorSet3')
    new_color_set.data = [[0.5, 0.5, 0.5, 0.5]] * len(o.positions[0].data)
    o.color_sets.append(new_color_set)

    # The vertex data is indexed, so use the vertex indices to get the actual rendered vertices.
    # The same vertex index can be used for all the attributes.
    for index in o.vertex_indices:
        print(o.positions[0].data[index], o.normals[0].data[index])

    for influence in o.bone_influences:
        # Vertex skinning information is stored separately for each bone.
        # Not all vertices will be influenced by each bone.
        print(influence.bone_name)

        # Vertex weights are assigned based on the vertex_index value.
        # The vertex index corresponds to the values in o.vertex_indices.
        print(influence.vertex_weights[0].vertex_index, influence.vertex_weights[0].vertex_weight)

# Save any changes made to the mesh.
mesh.save("model.numshb")
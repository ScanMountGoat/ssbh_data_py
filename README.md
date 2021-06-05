# ssbh_data_py
Python bindings to the [ssbh_data](https://github.com/ultimate-research/ssbh_lib) Rust library. ssbh_data_py provides a higher level abstraction over 
the binary data stored in SSBH files such as a "model.numshb". Currently, the library only supports edits to existing files. Creating meshes from scratch or adding new mesh objects is not currently supported.

Report bugs and request new features in [issues](https://github.com/ScanMountGoat/ssbh_data_py/issues). Check the requirements before reporting on any installation issues.

## Installing
Installing: `pip install ssbh_data_py`  
Updating: `pip install ssbh_data_py --upgrade`.  
The minimum supported pip version is 20.3. 

## Requirements
The package is available on [PyPi](https://pypi.org/project/ssbh_data_py/) for Python 3.6, 3.7, 3.8, and 3.9 for the latest versions of Windows, Linux, and Mac OS. The supported Linux distributions are Debian 11+, Fedora 34+, Mageia 8+, Photon OS 3.0 with updates, and Ubuntu 21.04+. See the [manylinux](https://github.com/pypa/manylinux) repo under the `many_linux_x_y` section for details on supported Linux distributions.

## Mesh Example
For additional python samples, see the [examples](https://github.com/ScanMountGoat/ssbh_data_py/tree/main/examples).
```python
import ssbh_data_py.mesh_data

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
    # This works for positions, normals, etc. 
    new_color_set = ssbh_data_py.mesh_data.AttributeData('colorSet3')
    new_color_set.data = [[0.5, 0.5, 0.5, 0.5]] * len(o.positions[0].data)
    o.color_sets.append(new_color_set)

    for influence in o.bone_influences:
        # Vertex skinning information is stored separately for each bone.
        # Not all vertices will be influenced by each bone.
        print(influence.bone_name)

        # Vertex weights are assigned based on the vertex_index value.
        # The vertex index corresponds to the values in o.vertex_indices.
        print(influence.vertex_weights[0].vertex_index, influence.vertex_weights[0].vertex_weight)

# Save any changes made to the mesh.
mesh.save("model.numshb")
```

## Building
`cargo build --release` builds the `ssbh_data_py` module that can be imported into Python. Depending on the platform, it may be necessary to rename the file. See the [PyO3 page](https://github.com/PyO3/pyo3) for details. The builds published to PyPi are built using [Maturin](https://github.com/PyO3/maturin).

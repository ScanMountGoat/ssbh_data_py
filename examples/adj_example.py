import ssbh_data_py

# Open the adj from an .adjb file.
adj = ssbh_data_py.adj_data.read_adj("model.adjb")

for entry in adj.entries:
    # Each entry corresponds to a mesh object with the given index.
    print(f"Mesh object index: {entry.mesh_object_index}")

    # The vertex_adjacency contains vertex indices from up to 9 adjacent faces.
    # This doesn't include the shared vertex itself.
    # For triangle faces, this means each face consists of two indices.
    # Values of -1 indicate unused indices.
    # Find the adjacency information for the 4th vertex (index 3).
    vertex_index = 3
    start = vertex_index * 18
    adjacent_vertices = entry.vertex_adjacency[start : start + 18]
    print(f"Vertex: {vertex_index}, Adjacent: {adjacent_vertices}")
    print()

# Save any changes made to the adjb.
adj.save("model.adjb")

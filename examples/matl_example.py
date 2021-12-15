import ssbh_data_py

# Open the matl from a .numatb file.
matl = ssbh_data_py.matl_data.read_matl("model.numatb")

for entry in matl.entries:
    # Each matl entry corresponds to a material. 
    # Materials define some of the inputs for the shader used to render a model.
    print(f'Material: {entry.material_label}, Shader: {entry.shader_label}')
    print(entry.blend_states)
    print(entry.floats)
    print(entry.booleans)
    print(entry.vectors)
    print(entry.rasterizer_states)
    print(entry.samplers)
    print(entry.textures)
    print()

    # Add a new parameter.
    param = ssbh_data_py.matl_data.FloatParam(ssbh_data_py.matl_data.ParamId.CustomFloat10, 0.9)
    entry.floats.append(param)

    # Some parameter types contain multiple fields.
    sampler = ssbh_data_py.matl_data.SamplerData()
    sampler.wraps = ssbh_data_py.matl_data.WrapMode.ClampToEdge 
    sampler.wrapt = ssbh_data_py.matl_data.WrapMode.Repeat 
    param = ssbh_data_py.matl_data.SamplerParam(ssbh_data_py.matl_data.ParamId.Sampler0, sampler)
    entry.samplers.append(param)


# Save any changes made to the matl.
matl.save("model.numatb")
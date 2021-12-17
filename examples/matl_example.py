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

    param = ssbh_data_py.matl_data.SamplerParam(ssbh_data_py.matl_data.ParamId.Sampler1, sampler)
    entry.samplers.append(param)

    # There will likely only be one blend state, so replace instead of appending.
    blend_state = ssbh_data_py.matl_data.BlendStateData()
    blend_state.source_color = ssbh_data_py.matl_data.BlendFactor.SourceAlpha
    blend_state.destination_color = ssbh_data_py.matl_data.BlendFactor.OneMinusSourceAlpha

    param = ssbh_data_py.matl_data.BlendStateParam(ssbh_data_py.matl_data.ParamId.BlendState0, blend_state)
    entry.blend_states = [param]

    # There will likely only be one rasterizer state, so replace instead of appending.
    rasterizer_state = ssbh_data_py.matl_data.RasterizerStateData()
    rasterizer_state.fill_mode = ssbh_data_py.matl_data.FillMode.Line

    param = ssbh_data_py.matl_data.RasterizerStateParam(ssbh_data_py.matl_data.ParamId.RasterizerState0, rasterizer_state)
    entry.rasterizer_states = [param]


# Save any changes made to the matl.
matl.save("model.numatb")
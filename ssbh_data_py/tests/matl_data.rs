use indoc::indoc;
use ssbh_data_py::run_python_code;

#[test]
fn read_matl() {
    // Test exceptions.
    run_python_code(indoc! {r#"
        try:
            ssbh_data_py.matl_data.read_matl("invalid")
        except ssbh_data_py.MatlDataError as e:
            assert True
    "#})
    .unwrap();
}

#[test]
fn create_matl() {
    run_python_code(indoc! {r#"
        m = ssbh_data_py.matl_data.MatlData(3, 4)
        assert m.major_version == 3
        assert m.minor_version == 4
        assert m.entries == []

        m = ssbh_data_py.matl_data.MatlData(1)
        assert m.major_version == 1
        assert m.minor_version == 6

        m = ssbh_data_py.matl_data.MatlData()
        assert m.major_version == 1
        assert m.minor_version == 6
    "#})
    .unwrap();
}

#[test]
fn create_matl_entry() {
    run_python_code(indoc! {r#"
        m = ssbh_data_py.matl_data.MatlEntryData("a", "b")
        assert m.material_label == "a"
        assert m.shader_label == "b"
        assert m.blend_states == []
        assert m.floats == []
        assert m.booleans == []
        assert m.vectors == []
        assert m.rasterizer_states == []
        assert m.samplers == []
        assert m.textures == []
    "#})
    .unwrap();
}

// Test the enum implementations here since methods are generated in the build script.
#[test]
fn cull_mode_enum_repr() {
    run_python_code(indoc! {r#"
        assert repr(ssbh_data_py.matl_data.CullMode.Back) == '<CullMode.Back: 0>'
        assert repr(ssbh_data_py.matl_data.CullMode.Front) == '<CullMode.Front: 1>'
        assert repr(ssbh_data_py.matl_data.CullMode.Disabled) == '<CullMode.Disabled: 2>'
    "#})
    .unwrap();
}

#[test]
fn cull_mode_enum_richcmp() {
    // The ordering should be defined over the values.
    run_python_code(indoc! {r#"
        assert ssbh_data_py.matl_data.CullMode.Back == ssbh_data_py.matl_data.CullMode.Back
        assert ssbh_data_py.matl_data.CullMode.Back != ssbh_data_py.matl_data.CullMode.Front

        assert ssbh_data_py.matl_data.CullMode.Disabled >= ssbh_data_py.matl_data.CullMode.Disabled
        assert ssbh_data_py.matl_data.CullMode.Disabled >= ssbh_data_py.matl_data.CullMode.Front
        assert ssbh_data_py.matl_data.CullMode.Disabled > ssbh_data_py.matl_data.CullMode.Front

        assert ssbh_data_py.matl_data.CullMode.Back <= ssbh_data_py.matl_data.CullMode.Back
        assert ssbh_data_py.matl_data.CullMode.Back <= ssbh_data_py.matl_data.CullMode.Front
        assert ssbh_data_py.matl_data.CullMode.Back < ssbh_data_py.matl_data.CullMode.Front
    "#})
    .unwrap();
}

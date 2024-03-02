use indoc::indoc;
use ssbh_data_py_types::run_python_code;

#[test]
fn read_modl() {
    // Test exceptions.
    run_python_code(indoc! {r#"
        try:
            ssbh_data_py.modl_data.read_modl("invalid")
        except ssbh_data_py.ModlDataError as e:
            assert True
    "#})
    .unwrap();
}

#[test]
fn create_modl() {
    run_python_code(indoc! {r#"
        m = ssbh_data_py.modl_data.ModlData(3, 4)
        assert m.major_version == 3
        assert m.minor_version == 4
        assert m.model_name == ""
        assert m.skeleton_file_name == ""
        assert m.material_file_names == []
        assert m.animation_file_name == None
        assert m.mesh_file_name == ""
        assert m.entries == []

        m = ssbh_data_py.modl_data.ModlData(3)
        assert m.major_version == 3
        assert m.minor_version == 7

        m = ssbh_data_py.modl_data.ModlData()
        assert m.major_version == 1
        assert m.minor_version == 7
    "#})
    .unwrap();
}

#[test]
fn create_modl_entry() {
    run_python_code(indoc! {r#"
        m = ssbh_data_py.modl_data.ModlEntryData("a", 7, "b")
        assert m.mesh_object_name == "a"
        assert m.mesh_object_subindex == 7
        assert m.material_label == "b"
    "#})
    .unwrap();
}

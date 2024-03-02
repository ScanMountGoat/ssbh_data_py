use indoc::indoc;
use ssbh_data_py_types::run_python_code;

#[test]
fn read_meshex() {
    // Test exceptions.
    run_python_code(indoc! {r#"
        try:
            ssbh_data_py.meshex_data.read_meshex("invalid")
        except ssbh_data_py.MeshExDataError as e:
            assert True
    "#})
    .unwrap();
}

#[test]
fn create_meshex() {
    run_python_code(indoc! {r#"
        m = ssbh_data_py.meshex_data.MeshExData()
        assert m.mesh_object_groups == []
    "#})
    .unwrap();
}

#[test]
fn create_mesh_object_group() {
    run_python_code(indoc! {r#"
        sphere = ssbh_data_py.meshex_data.BoundingSphere([1, 2, 3], 4)
        m = ssbh_data_py.meshex_data.MeshObjectGroupData(sphere, "a", "a_VIS", [])
        assert m.bounding_sphere.center == [1.0, 2.0, 3.0]
        assert m.bounding_sphere.radius == 4.0
        assert m.mesh_object_name == "a"
        assert m.mesh_object_full_name == "a_VIS"
        assert m.entry_flags == []
    "#})
    .unwrap();
}

#[test]
fn create_meshex_from_objects() {
    run_python_code(indoc! {r#"
        ssbh_data_py.meshex_data.MeshExData.from_mesh_objects([])
        ssbh_data_py.meshex_data.MeshExData.from_mesh_objects([
            ssbh_data_py.mesh_data.MeshObjectData('a', 0),
            ssbh_data_py.mesh_data.MeshObjectData('a', 1)
        ])
    "#})
    .unwrap();
}

use indoc::indoc;
use ssbh_data_py_types::{run_python_code, run_python_code_numpy};

#[test]
fn read_adj() {
    // Test exceptions.
    run_python_code(indoc! {r#"
            try:
                ssbh_data_py.adj_data.read_adj("invalid")
            except ssbh_data_py.AdjDataError as e:
                assert True
        "#})
    .unwrap();
}

#[test]
fn create_adj() {
    run_python_code(indoc! {r#"
            a = ssbh_data_py.adj_data.AdjData()
            assert a.entries == []
        "#})
    .unwrap();
}

#[test]
fn create_adj_entry() {
    run_python_code(indoc! {r#"
            e = ssbh_data_py.adj_data.AdjEntryData(3)
            assert e.mesh_object_index == 3
            assert e.vertex_adjacency == []
        "#})
    .unwrap();
}

#[test]
fn vertex_adjacency_tuples() {
    run_python_code(indoc! {r#"
            e = ssbh_data_py.adj_data.AdjEntryData(3)
            assert e.mesh_object_index == 3
            e.vertex_adjacency = (-1, 3, 7)
            assert list(e.vertex_adjacency) == [-1, 3, 7]
        "#})
    .unwrap();
}

#[test]
fn vertex_adjacency_numpy() {
    run_python_code_numpy(indoc! {r#"
            e = ssbh_data_py.adj_data.AdjEntryData(3)
            assert e.mesh_object_index == 3
            e.vertex_adjacency = np.array([-1, 3, 7])
            assert e.vertex_adjacency.tolist() == [-1, 3, 7]
        "#})
    .unwrap();
}

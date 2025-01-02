use indoc::indoc;
use ssbh_data_py::run_python_code;

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
            assert e.vertex_adjacency.tolist() == []
        "#})
    .unwrap();
}

#[test]
fn vertex_adjacency_numpy() {
    run_python_code(indoc! {r#"
            e = ssbh_data_py.adj_data.AdjEntryData(3)
            assert e.mesh_object_index == 3
            e.vertex_adjacency = numpy.array([-1, 3, 7], dtype=numpy.int16)
            assert e.vertex_adjacency.tolist() == [-1, 3, 7]
        "#})
    .unwrap();
}

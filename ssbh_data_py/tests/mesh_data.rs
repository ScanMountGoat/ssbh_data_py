use indoc::indoc;
use pyo3::prelude::*;
use ssbh_data::mesh_data::VectorData;
use ssbh_data_py_types::{eval_python_code, run_python_code, MapPy};

#[test]
fn read_mesh() {
    // Test exceptions.
    run_python_code(indoc! {r#"
        try:
            ssbh_data_py.mesh_data.read_mesh("invalid")
        except ssbh_data_py.MeshDataError as e:
            assert True
    "#})
    .unwrap();
}

#[test]
fn create_mesh() {
    run_python_code(indoc! {r#"
        m = ssbh_data_py.mesh_data.MeshData(3, 4)
        assert m.major_version == 3
        assert m.minor_version == 4

        m = ssbh_data_py.mesh_data.MeshData(3)
        assert m.major_version == 3
        assert m.minor_version == 10

        m = ssbh_data_py.mesh_data.MeshData()
        assert m.major_version == 1
        assert m.minor_version == 10
    "#})
    .unwrap();
}

#[test]
fn create_modify_mesh_object() {
    run_python_code(indoc! {r#"
        m = ssbh_data_py.mesh_data.MeshObjectData("abc", 1)
        assert m.name == "abc"
        assert m.subindex == 1
        assert m.parent_bone_name == ""
        assert m.vertex_indices.tolist() == []
        assert m.positions == []
        assert m.normals == []
        assert m.binormals == []
        assert m.tangents == []
        assert m.texture_coordinates == []
        assert m.color_sets == []
        assert m.bone_influences == []

        m.vertex_indices = np.array([1, 2, 3], dtype=np.uint32)
        assert m.vertex_indices.tolist() == [1,2,3]
    "#})
    .unwrap();
}

#[test]
fn mesh_object_vertex_indices_ndarray() {
    run_python_code(indoc! {r#"
        m = ssbh_data_py.mesh_data.MeshObjectData("abc", 1)
        m.vertex_indices = np.array([1, 2, 3], dtype=np.uint32)
        assert m.vertex_indices.tolist() == [1,2,3]
    "#})
    .unwrap();
}

#[test]
fn create_modify_attribute_data() {
    run_python_code(indoc! {r#"
        a = ssbh_data_py.mesh_data.AttributeData("abc")
        assert a.name == "abc"
        assert a.data.tolist() == []

        a.name = "def"
        a.data = np.array([[1.0, 2.0]], dtype=np.float32)
        assert a.name == "def"
        assert a.data.tolist() == [[1.0, 2.0]]

        # Test mutability for nested types.
        a.data[0] = [2.5, 3.5]
        assert a.data.tolist() == [[2.5, 3.5]]
    "#})
    .unwrap();
}

#[test]
fn create_modify_vertex_weight() {
    run_python_code(indoc! {r#"
        v = ssbh_data_py.mesh_data.VertexWeight(1, 0.5)
        assert v.vertex_index == 1
        assert v.vertex_weight == 0.5

        v.vertex_index = 0
        v.vertex_weight = 0.0
        assert v.vertex_index == 0
        assert v.vertex_weight == 0.0
    "#})
    .unwrap();
}

#[test]
fn create_modify_bone_influence() {
    run_python_code(indoc! {r#"
        b = ssbh_data_py.mesh_data.BoneInfluence("abc", [])
        assert b.bone_name == "abc"
        assert b.vertex_weights == []

        b.bone_name = "def"
        b.vertex_weights = [ssbh_data_py.mesh_data.VertexWeight(1, 0.5)]
        assert b.bone_name == "def"
        assert len(b.vertex_weights) == 1
        assert b.vertex_weights[0].vertex_weight == 0.5

        # Test mutability for nested types.
        b.vertex_weights[0].vertex_index = 2
        assert b.vertex_weights[0].vertex_index == 2
    "#})
    .unwrap();
}

#[test]
fn vector2_from_ndarray() {
    eval_python_code(
        "np.array([[0.0, 1.0], [2.0, 3.0]], dtype=np.float32)",
        |py, x| {
            let x = x.downcast::<numpy::PyArray2<f32>>().unwrap();
            let value = x.as_unbound().map_py(py).unwrap();
            assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
        },
    );
}

#[test]
fn vector3_from_ndarray() {
    eval_python_code(
        "np.array([[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]], dtype=np.float32)",
        |py, x| {
            let x = x.downcast::<numpy::PyArray2<f32>>().unwrap();
            let value = x.as_unbound().map_py(py).unwrap();
            assert_eq!(
                VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
                value
            );
        },
    );
}

#[test]
fn vector4_from_ndarray() {
    eval_python_code(
        "np.array([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]], dtype=np.float32)",
        |py, x| {
            let x = x.downcast::<numpy::PyArray2<f32>>().unwrap();
            let value = x.as_unbound().map_py(py).unwrap();
            assert_eq!(
                VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
                value
            );
        },
    );
}

#[test]
#[should_panic]
fn vector_from_5x5_ndarray() {
    // Vector5 is not a valid variant.
    eval_python_code("np.zeros((5,5), dtype=np.float32)", |py, x| {
        let x = x.downcast::<numpy::PyArray2<f32>>().unwrap();
        let _: VectorData = x.as_unbound().map_py(py).unwrap();
    });
}

#[test]
#[ignore]
#[should_panic]
fn vector_from_empty_ndarray() {
    // TODO: How to infer the type when there are no elements?
    eval_python_code("np.array()", |py, x| {
        let x = x.downcast::<numpy::PyArray1<f32>>().unwrap();
        let _ = x.as_unbound().map_py(py).unwrap();
    });
}

#[test]
fn transform_points_ndarray() {
    run_python_code(indoc! {r#"
        points = np.array([[1,2,3],[4,5,6]], dtype=np.float32)
        transform = np.array([
            [1,0,0,0],
            [0,1,0,0],
            [0,0,1,0],
            [-1,-2,-3,1]
        ], dtype=np.float32)
        transformed = ssbh_data_py.mesh_data.transform_points(points, transform)
        assert transformed.tolist() == [[0,0,0],[3,3,3]]
    "#})
    .unwrap();
}

#[test]
fn transform_vectors_ndarray() {
    run_python_code(indoc! {r#"
        points = np.array([[1,2,3],[4,5,6]], dtype=np.float32)
        transform = np.array([
            [1,0,0,0],
            [0,1,0,0],
            [0,0,1,0],
            [-1,-2,-3,1]
        ], dtype=np.float32)
        transformed = ssbh_data_py.mesh_data.transform_vectors(points, transform)
        assert transformed.tolist() == [[1,2,3],[4,5,6]]
    "#})
    .unwrap();
}

#[test]
fn calculate_smooth_normals_ndarray() {
    run_python_code(indoc! {r#"
        ssbh_data_py.mesh_data.calculate_smooth_normals(np.zeros((12,4), dtype=np.float32), np.arange(12, dtype=np.uint32))
    "#})
    .unwrap();
}

#[test]
fn calculate_tangents_vec4_ndarray() {
    run_python_code(indoc! {r#"
        ssbh_data_py.mesh_data.calculate_tangents_vec4(np.zeros((12,4), dtype=np.float32), np.zeros((12,4), dtype=np.float32), np.zeros((12,2), dtype=np.float32), np.arange(12, dtype=np.uint32))
    "#}).unwrap();
}

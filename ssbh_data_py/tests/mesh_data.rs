use indoc::indoc;
use pyo3::PyObject;
use ssbh_data::mesh_data::VectorData;
use ssbh_data_py_types::MapPy;
use ssbh_data_py_types::{
    eval_python_code, eval_python_code_numpy, run_python_code, run_python_code_numpy,
};

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
        assert m.vertex_indices == []
        assert m.positions == []
        assert m.normals == []
        assert m.binormals == []
        assert m.tangents == []
        assert m.texture_coordinates == []
        assert m.color_sets == []
        assert m.bone_influences == []

        m.vertex_indices = [1, 2, 3]
        assert m.vertex_indices == [1,2,3]
    "#})
    .unwrap();
}

#[test]
fn mesh_object_vertex_indices_ndarray() {
    run_python_code_numpy(indoc! {r#"
        m = ssbh_data_py.mesh_data.MeshObjectData("abc", 1)
        m.vertex_indices = np.array([1, 2, 3])
        assert m.vertex_indices.tolist() == [1,2,3]
    "#})
    .unwrap();
}

#[test]
fn create_modify_attribute_data() {
    run_python_code(indoc! {r#"
        a = ssbh_data_py.mesh_data.AttributeData("abc")
        assert a.name == "abc"
        assert a.data == []

        a.name = "def"
        a.data = [[1.0, 2.0]]
        assert a.name == "def"
        assert a.data == [[1.0, 2.0]]

        # Test mutability for nested types.
        a.data[0][1] = 0.3
        assert a.data == [[1.0, 0.3]]
        a.data[0] = [2.5, 3.5]
        assert a.data == [[2.5, 3.5]]
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
#[should_panic]
fn vector2_from_pylist_invalid_type() {
    eval_python_code("[[0, 1], [2, 'a']]", |py, x| {
        let _: VectorData = PyObject::from(x).map_py(py, false).unwrap();
    });
}

#[test]
#[should_panic]
fn vector2_from_pylist_invalid_component_count() {
    eval_python_code("[[0.0, 1.0], [2.0]]", |py, x| {
        let _: VectorData = PyObject::from(x).map_py(py, false).unwrap();
    });
}

#[test]
fn vector2_from_pylist_ints() {
    eval_python_code("[[0, 1], [2, 3]]", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
    });
}

#[test]
fn vector2_from_ndarray_ints() {
    eval_python_code_numpy("np.array([[0, 1], [2, 3]],dtype=np.int8)", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
    });
}

#[test]
fn vector2_from_pylist() {
    eval_python_code("[[0.0, 1.0], [2.0, 3.0]]", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
    });
}

#[test]
fn vector2_from_tuples() {
    eval_python_code("[(0.0, 1.0), (2.0, 3.0)]", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
    });
}

#[test]
fn vector2_from_ndarray() {
    eval_python_code_numpy("np.array([[0.0, 1.0], [2.0, 3.0]])", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(VectorData::Vector2(vec![[0.0, 1.0], [2.0, 3.0]]), value);
    });
}

#[test]
fn vector3_from_pylist() {
    eval_python_code("[[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(
            VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
            value
        );
    });
}

#[test]
fn vector3_from_tuples() {
    eval_python_code("[(0.0, 1.0, 2.0), (3.0, 4.0, 5.0)]", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(
            VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
            value
        );
    });
}

#[test]
fn vector3_from_ndarray() {
    eval_python_code_numpy("np.array([[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]])", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(
            VectorData::Vector3(vec![[0.0, 1.0, 2.0], [3.0, 4.0, 5.0]]),
            value
        );
    });
}

#[test]
fn vector4_from_pylist() {
    eval_python_code("[[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(
            VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
            value
        );
    });
}

#[test]
fn vector4_from_tuples() {
    eval_python_code("[(0.0, 1.0, 2.0, 3.0), (4.0, 5.0, 6.0, 7.0)]", |py, x| {
        let value = PyObject::from(x).map_py(py, false).unwrap();
        assert_eq!(
            VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
            value
        );
    });
}

#[test]
fn vector4_from_ndarray() {
    eval_python_code_numpy(
        "np.array([[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]])",
        |py, x| {
            let value = PyObject::from(x).map_py(py, false).unwrap();
            assert_eq!(
                VectorData::Vector4(vec![[0.0, 1.0, 2.0, 3.0], [4.0, 5.0, 6.0, 7.0]]),
                value
            );
        },
    );
}

#[test]
#[should_panic]
fn vector_from_5x5_pylist() {
    // Vector5 is not a valid variant.
    eval_python_code("[[1.0,2.0,3.0,4.0,5.0]]", |py, x| {
        let _: VectorData = PyObject::from(x).map_py(py, false).unwrap();
    });
}

#[test]
#[should_panic]
fn vector_from_5x5_ndarray() {
    // Vector5 is not a valid variant.
    eval_python_code_numpy("np.zeros((5,5))", |py, x| {
        let _: VectorData = PyObject::from(x).map_py(py, false).unwrap();
    });
}

#[test]
#[ignore]
#[should_panic]
fn vector_from_empty_pylist() {
    // TODO: How to infer the type when there are no elements?
    eval_python_code("[]", |py, x| {
        let _: VectorData = PyObject::from(x).map_py(py, false).unwrap();
    });
}

#[test]
#[ignore]
#[should_panic]
fn vector_from_empty_ndarray() {
    // TODO: How to infer the type when there are no elements?
    eval_python_code_numpy("np.array()", |py, x| {
        let _: VectorData = PyObject::from(x).map_py(py, false).unwrap();
    });
}

#[test]
fn transform_points_pylist() {
    run_python_code(indoc! {r#"
            points = [[1,2,3],[4,5,6]]
            transform = [
                [1,0,0,0],
                [0,1,0,0],
                [0,0,1,0],
                [-1,-2,-3,1]
            ]
            transformed = ssbh_data_py.mesh_data.transform_points(points, transform)
            assert transformed == [[0,0,0],[3,3,3]]
        "#})
    .unwrap();
}

#[test]
fn transform_points_tuple() {
    run_python_code(indoc! {r#"
            points = ((1,2,3),(4,5,6))
            transform = (
                (1,0,0,0),
                (0,1,0,0),
                (0,0,1,0),
                (-1,-2,-3,1)
            )
            transformed = ssbh_data_py.mesh_data.transform_points(points, transform)
            assert transformed == [[0,0,0],[3,3,3]]
        "#})
    .unwrap();
}

#[test]
fn transform_points_ndarray() {
    run_python_code_numpy(indoc! {r#"
        points = np.array([[1,2,3],[4,5,6]])
        transform = np.array([
            [1,0,0,0],
            [0,1,0,0],
            [0,0,1,0],
            [-1,-2,-3,1]
        ])
        transformed = ssbh_data_py.mesh_data.transform_points(points, transform)
        assert transformed == [[0,0,0],[3,3,3]]
    "#})
    .unwrap();
}

#[test]
fn transform_vectors_pylist() {
    run_python_code(indoc! {r#"
        points = [[1,2,3],[4,5,6]]
        transform = [
            [1,0,0,0],
            [0,1,0,0],
            [0,0,1,0],
            [-1,-2,-3,1]
        ]
        transformed = ssbh_data_py.mesh_data.transform_vectors(points, transform)
        assert transformed == [[1,2,3],[4,5,6]]
    "#})
    .unwrap();
}

#[test]
fn transform_vectors_tuple() {
    run_python_code(indoc! {r#"
        points = ((1,2,3),(4,5,6))
        transform = (
            (1,0,0,0),
            (0,1,0,0),
            (0,0,1,0),
            (-1,-2,-3,1)
        )
        transformed = ssbh_data_py.mesh_data.transform_vectors(points, transform)
        assert transformed == [[1,2,3],[4,5,6]]
    "#})
    .unwrap();
}

#[test]
fn transform_vectors_ndarray() {
    run_python_code_numpy(indoc! {r#"
        points = np.array([[1,2,3],[4,5,6]])
        transform = np.array([
            [1,0,0,0],
            [0,1,0,0],
            [0,0,1,0],
            [-1,-2,-3,1]
        ])
        transformed = ssbh_data_py.mesh_data.transform_vectors(points, transform)
        assert transformed == [[1,2,3],[4,5,6]]
    "#})
    .unwrap();
}

#[test]
fn calculate_smooth_normals_pylist() {
    run_python_code(indoc! {r#"
        ssbh_data_py.mesh_data.calculate_smooth_normals([[0,0,0]]*36, list(range(36)))
    "#})
    .unwrap();
}

#[test]
fn calculate_smooth_normals_tuple() {
    run_python_code(indoc! {r#"
        ssbh_data_py.mesh_data.calculate_smooth_normals(((0,0,0),(1,1,1),(2,2,2)), (0,1,2))
    "#})
    .unwrap();
}

#[test]
fn calculate_smooth_normals_ndarray() {
    run_python_code_numpy(indoc! {r#"
        ssbh_data_py.mesh_data.calculate_smooth_normals(np.zeros((12,4)), np.arange(12))
    "#})
    .unwrap();
}

#[test]
fn calculate_tangents_vec4_pylist() {
    run_python_code(indoc! {r#"
        ssbh_data_py.mesh_data.calculate_tangents_vec4([[0,0,0],[1,1,1],[2,2,2]], [[0,0,0],[1,1,1],[2,2,2]], [[0,0],[1,1],[2,2]], [0,1,2])
    "#})
    .unwrap();
}

#[test]
fn calculate_tangents_vec4_tuple() {
    run_python_code(indoc! {r#"
        ssbh_data_py.mesh_data.calculate_tangents_vec4(((0,0,0),(1,1,1),(2,2,2)), ((0,0,0),(1,1,1),(2,2,2)), ((0,0),(1,1),(2,2)), (0,1,2))
    "#}).unwrap();
}

#[test]
fn calculate_tangents_vec4_ndarray() {
    run_python_code_numpy(indoc! {r#"
        ssbh_data_py.mesh_data.calculate_tangents_vec4(np.zeros((12,4)), np.zeros((12,4)), np.zeros((12,2)), np.arange(12))
    "#}).unwrap();
}

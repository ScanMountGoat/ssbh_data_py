use indoc::indoc;
use ssbh_data_py_types::run_python_code;

#[test]
fn read_skel() {
    // Test exceptions.
    run_python_code(indoc! {r#"
        try:
            ssbh_data_py.skel_data.read_skel("invalid")
        except ssbh_data_py.SkelDataError as e:
            assert True
    "#})
    .unwrap();
}

#[test]
fn create_skel() {
    run_python_code(indoc! {r#"
        s = ssbh_data_py.skel_data.SkelData()
        assert s.major_version == 1
        assert s.minor_version == 0
        assert s.bones == []
    "#})
    .unwrap();
}

#[test]
fn create_bone_data_numpy() {
    run_python_code(indoc! {r#"
        b = ssbh_data_py.skel_data.BoneData("abc", np.zeros((4,4), dtype=np.float32), 5)
        assert b.name == "abc"
        assert b.transform.tolist() == [[0,0,0,0]]*4
        assert b.parent_index == 5

        b = ssbh_data_py.skel_data.BoneData("abc", np.ones((4,4), dtype=np.float32), None)
        assert b.name == "abc"
        assert b.transform.tolist() == [[1,1,1,1]]*4
        assert b.parent_index == None
        # Test mutability.
        b.transform[1][2] = 3
        assert b.transform[1].tolist() == [1,1,3,1]
    "#})
    .unwrap();
}

#[test]
fn calculate_world_transform_no_parent() {
    run_python_code(indoc! {r#"
        s = ssbh_data_py.skel_data.SkelData()
        b0 = ssbh_data_py.skel_data.BoneData("b0", np.zeros((4,4), dtype=np.float32), None)
        b1 = ssbh_data_py.skel_data.BoneData("b1", np.ones((4,4), dtype=np.float32), None)
        s.bones = [b0, b1]

        assert s.calculate_world_transform(b1).tolist() == b1.transform.tolist()
    "#})
    .unwrap();
}

#[test]
fn calculate_world_transform_with_parent() {
    run_python_code(indoc! {r#"
        s = ssbh_data_py.skel_data.SkelData()
        b0 = ssbh_data_py.skel_data.BoneData("b0", np.ones((4,4), dtype=np.float32), None)
        b1 = ssbh_data_py.skel_data.BoneData("b0", np.ones((4,4), dtype=np.float32) * 2, 0)
        s.bones = [b0, b1]

        assert s.calculate_world_transform(b1).tolist() == [[8,8,8,8]]*4
    "#})
    .unwrap();
}

#[test]
fn calculate_world_transform_with_parent_ndarray() {
    run_python_code(indoc! {r#"
        s = ssbh_data_py.skel_data.SkelData()
        b0 = ssbh_data_py.skel_data.BoneData("b0", np.ones((4,4), dtype=np.float32), None)
        b1 = ssbh_data_py.skel_data.BoneData("b0", np.ones((4,4), dtype=np.float32)*2, 0)
        s.bones = [b0, b1]

        assert s.calculate_world_transform(b1).tolist() == [[8,8,8,8]]*4
    "#})
    .unwrap();
}

#[test]
fn calculate_relative_transform_with_parent() {
    run_python_code(indoc! {r#"
        world_transform = np.array([
            [2, 0, 0, 0],
            [0, 4, 0, 0],
            [0, 0, 8, 0],
            [0, 0, 0, 1],
        ], dtype=np.float32)
        parent_world_transform = np.array([
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [1, 2, 3, 1],
        ], dtype=np.float32)
        relative_transform = np.array([
            [2.0, 0, 0, 0],
            [0, 4, 0, 0],
            [0, 0, 8, 0],
            [-2, -8, -24, 1],
        ], dtype=np.float32)
        assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, parent_world_transform).tolist() == relative_transform.tolist()
    "#})
    .unwrap();
}

#[test]
fn calculate_relative_transform_no_parent() {
    run_python_code(indoc! {r#"
        world_transform = np.array([
            [0, 1, 2, 3],
            [4, 5, 6, 7],
            [8, 9, 10, 11],
            [12, 13, 14, 15],
        ], dtype=np.float32)
        assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None).tolist() == world_transform.tolist()
    "#})
    .unwrap();
}

#[test]
fn calculate_relative_transform_no_parent_ndarray() {
    // TODO: This can also return a numpy array in the future.
    run_python_code(indoc! {r#"
        world_transform = np.array([
            [0, 1, 2, 3],
            [4, 5, 6, 7],
            [8, 9, 10, 11],
            [12, 13, 14, 15],
        ], dtype=np.float32)
        assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None).tolist() == world_transform.tolist()
    "#})
    .unwrap();
}

#[test]
fn calculate_relative_transform_no_parent_tuple() {
    // Tuples should be treated like sequences.
    run_python_code(indoc! {r#"
        world_transform = np.array([
            (0, 1, 2, 3),
            (4, 5, 6, 7),
            (8, 9, 10, 11),
            (12, 13, 14, 15),
        ], dtype=np.float32)
        expected = np.array([
            [0, 1, 2, 3],
            [4, 5, 6, 7],
            [8, 9, 10, 11],
            [12, 13, 14, 15],
        ], dtype=np.float32)
        assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None).tolist() == expected.tolist()
    "#})
    .unwrap();
}

use indoc::indoc;
use ssbh_data_py_types::{run_python_code, run_python_code_numpy};

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
fn create_bone_data() {
    // TODO: Fix assertions to compare enums.
    // This may require implementing __richcmp__.
    run_python_code(indoc! {r#"
        b = ssbh_data_py.skel_data.BoneData("abc", [[0,0,0,0]]*4, 5, ssbh_data_py.skel_data.BillboardType.YAxisViewPlaneAligned)
        assert b.name == "abc"
        assert b.transform == [[0,0,0,0]]*4
        assert b.parent_index == 5
        assert b.billboard_type == ssbh_data_py.skel_data.BillboardType.YAxisViewPlaneAligned

        b = ssbh_data_py.skel_data.BoneData("abc", [[1,1,1,1]]*4, None)
        assert b.name == "abc"
        assert b.transform == [[1,1,1,1]]*4
        assert b.parent_index == None
        assert b.billboard_type == ssbh_data_py.skel_data.BillboardType.Disabled
        # Test mutability.
        b.transform[1][2] = 3
        assert b.transform[1] == [1,1,3,1]
    "#})
    .unwrap();
}

#[test]
fn create_bone_data_tuples() {
    run_python_code(indoc! {r#"
        billboard = ssbh_data_py.skel_data.BillboardType.YAxisViewPlaneAligned
        b = ssbh_data_py.skel_data.BoneData("abc", [(0,0,0,0)]*4, 5, billboard)
        assert b.name == "abc"
        assert b.transform == [(0,0,0,0)]*4
        assert b.parent_index == 5

        b = ssbh_data_py.skel_data.BoneData("abc", [(1,1,1,1)]*4)
        assert b.name == "abc"
        assert b.transform == [(1,1,1,1)]*4
        assert b.parent_index == None
    "#})
    .unwrap();
}

#[test]
fn create_bone_data_numpy() {
    run_python_code_numpy(indoc! {r#"
        b = ssbh_data_py.skel_data.BoneData("abc", np.zeros((4,4)), 5)
        assert b.name == "abc"
        assert b.transform.tolist() == [[0,0,0,0]]*4
        assert b.parent_index == 5

        b = ssbh_data_py.skel_data.BoneData("abc", np.ones((4,4)), None)
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
        b0 = ssbh_data_py.skel_data.BoneData("b0", [[0,0,0,0]]*4, None)
        b1 = ssbh_data_py.skel_data.BoneData("b1", [[1,1,1,1]]*4, None)
        s.bones = [b0, b1]

        assert s.calculate_world_transform(b1) == b1.transform
    "#})
    .unwrap();
}

#[test]
fn calculate_world_transform_with_parent() {
    run_python_code(indoc! {r#"
        s = ssbh_data_py.skel_data.SkelData()
        b0 = ssbh_data_py.skel_data.BoneData("b0", [[1,1,1,1]]*4, None)
        b1 = ssbh_data_py.skel_data.BoneData("b0", [[2,2,2,2]]*4, 0)
        s.bones = [b0, b1]

        assert s.calculate_world_transform(b1) == [[8,8,8,8]]*4
    "#})
    .unwrap();
}

#[test]
fn calculate_world_transform_with_parent_ndarray() {
    // TODO: This can also return a numpy array in the future.
    run_python_code_numpy(indoc! {r#"
        s = ssbh_data_py.skel_data.SkelData()
        b0 = ssbh_data_py.skel_data.BoneData("b0", np.ones((4,4)), None)
        b1 = ssbh_data_py.skel_data.BoneData("b0", np.ones((4,4))*2, 0)
        s.bones = [b0, b1]

        assert s.calculate_world_transform(b1) == [[8,8,8,8]]*4
    "#})
    .unwrap();
}

#[test]
fn calculate_relative_transform_with_parent() {
    run_python_code(indoc! {r#"
        world_transform = [
            [2, 0, 0, 0],
            [0, 4, 0, 0],
            [0, 0, 8, 0],
            [0, 0, 0, 1],
        ]
        parent_world_transform = [
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [1, 2, 3, 1],
        ]
        relative_transform = [
            [2.0, 0, 0, 0],
            [0, 4, 0, 0],
            [0, 0, 8, 0],
            [-2, -8, -24, 1],
        ]
        assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, parent_world_transform) == relative_transform
    "#})
    .unwrap();
}

#[test]
fn calculate_relative_transform_no_parent() {
    run_python_code(indoc! {r#"
        world_transform = [
            [0, 1, 2, 3],
            [4, 5, 6, 7],
            [8, 9, 10, 11],
            [12, 13, 14, 15],
        ]
        assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None) == world_transform
    "#})
    .unwrap();
}

#[test]
fn calculate_relative_transform_no_parent_ndarray() {
    // TODO: This can also return a numpy array in the future.
    run_python_code_numpy(indoc! {r#"
        world_transform = np.array([
            [0, 1, 2, 3],
            [4, 5, 6, 7],
            [8, 9, 10, 11],
            [12, 13, 14, 15],
        ])
        assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None) == world_transform.tolist()
    "#})
    .unwrap();
}

#[test]
fn calculate_relative_transform_no_parent_tuple() {
    // Tuples should be treated like sequences.
    run_python_code(indoc! {r#"
        world_transform = [
            (0, 1, 2, 3),
            (4, 5, 6, 7),
            (8, 9, 10, 11),
            (12, 13, 14, 15),
        ]
        expected = [
            [0, 1, 2, 3],
            [4, 5, 6, 7],
            [8, 9, 10, 11],
            [12, 13, 14, 15],
        ]
        assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None) == expected
    "#})
    .unwrap();
}

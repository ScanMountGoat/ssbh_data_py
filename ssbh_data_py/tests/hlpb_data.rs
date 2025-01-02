use indoc::indoc;
use ssbh_data_py::run_python_code;

#[test]
fn read_hlpb() {
    // Test exceptions.
    run_python_code(indoc! {r#"
        try:
            ssbh_data_py.hlpb_data.read_hlpb("invalid")
        except ssbh_data_py.HlpbDataError as e:
            assert True
    "#})
    .unwrap();
}

#[test]
fn create_hlpb() {
    run_python_code(indoc! {r#"
        h = ssbh_data_py.hlpb_data.HlpbData(3, 4)
        assert h.major_version == 3
        assert h.minor_version == 4
        assert h.aim_constraints == []
        assert h.orient_constraints == []

        m = ssbh_data_py.hlpb_data.HlpbData(3)
        assert m.major_version == 3
        assert m.minor_version == 0

        m = ssbh_data_py.hlpb_data.HlpbData()
        assert m.major_version == 1
        assert m.minor_version == 0
    "#})
    .unwrap();
}

// TODO: Test orient and aim constraint constructors.
// TODO: Add defaults for fields like range min, range max, etc?
#[test]
fn create_aim_constraint() {
    run_python_code(indoc! {r#"
        a = ssbh_data_py.hlpb_data.AimConstraintData('a', 'p1', 'p2', 't1', 't2')
        assert a.name == 'a'
        assert a.aim_bone_name1 == 'p1'
        assert a.aim_bone_name2 == 'p2'
        assert a.target_bone_name1 == 't1'
        assert a.target_bone_name2 == 't2'
        assert a.aim_type1 == 'DEFAULT'
        assert a.aim_type2 == 'DEFAULT'
        assert a.unk1 == 0
        assert a.unk2 == 0
        assert a.aim == [1, 0, 0]
        assert a.up == [0, 1, 0]
        assert a.quat1 == [0, 0, 0, 1]
        assert a.quat2 == [0, 0, 0, 1]
    "#})
    .unwrap();
}

#[test]
fn create_orient_constraint() {
    run_python_code(indoc! {r#"
        o = ssbh_data_py.hlpb_data.OrientConstraintData('a', 'p1', 'p2', 's', 't', 1, [0.1, 0.2, 0.3])
        assert o.name == 'a'
        assert o.parent_bone_name1 == 'p1'
        assert o.parent_bone_name2 == 'p2'
        assert o.source_bone_name == 's'
        assert o.target_bone_name == 't'
        assert o.unk_type == 1
        assert o.constraint_axes == [0.1, 0.2, 0.3]
        assert o.quat1 == [0, 0, 0, 1]
        assert o.quat2 == [0, 0, 0, 1]
        assert o.range_min == [-180, -180, -180]
        assert o.range_max == [180, 180, 180]
    "#})
    .unwrap();
}

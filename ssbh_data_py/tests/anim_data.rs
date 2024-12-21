use indoc::indoc;
use pyo3::{prelude::*, types::PyList};
use ssbh_data::{Vector3, Vector4};
use ssbh_data_py_types::{
    anim_data::{create_track_values_rs, GroupType},
    eval_python_code, run_python_code,
};

#[test]
fn read_anim() {
    // Test exceptions.
    run_python_code(indoc! {r#"
            try:
                ssbh_data_py.anim_data.read_anim("invalid")
            except ssbh_data_py.AnimDataError as e:
                assert True
        "#})
    .unwrap();
}

#[test]
fn create_anim_data() {
    run_python_code(indoc! {r#"
            a = ssbh_data_py.anim_data.AnimData()
            assert a.major_version == 2
            assert a.minor_version == 0
            assert a.groups == []
        "#})
    .unwrap();
}

#[test]
fn create_group_data() {
    run_python_code(indoc! {r#"
            a = ssbh_data_py.anim_data.GroupData(ssbh_data_py.anim_data.GroupType.Transform)
            assert a.group_type.name == 'Transform'
            assert a.nodes == []
        "#})
    .unwrap();
}

#[test]
fn create_node_data() {
    run_python_code(indoc! {r#"
            a = ssbh_data_py.anim_data.NodeData('abc')
            assert a.name == 'abc'
            assert a.tracks == []
        "#})
    .unwrap();
}

#[test]
fn create_track_data() {
    run_python_code(indoc! {r#"
            a = ssbh_data_py.anim_data.TrackData('abc')
            assert a.name == 'abc'
            assert a.values == []
            assert a.compensate_scale == False
        "#})
    .unwrap();
}

#[test]
fn create_transform_flags() {
    run_python_code(indoc! {r#"
            f = ssbh_data_py.anim_data.TransformFlags()
            assert f.override_translation == False
            assert f.override_rotation == False
            assert f.override_scale == False
            assert f.override_compensate_scale == False
        "#})
    .unwrap();
}

#[test]
fn create_transform() {
    run_python_code(indoc! {r#"
            t = ssbh_data_py.anim_data.Transform([1, 2, 3], [4, 5, 6, 7], [8, 9, 10])
            assert t.scale == [1, 2, 3]
            assert t.rotation == [4, 5, 6, 7]
            assert t.translation == [8, 9, 10]
        "#})
    .unwrap();
}

#[test]
fn transform_repr() {
    // Check that repr can be used to construct the type.
    run_python_code(indoc! {r#"
            t = ssbh_data_py.anim_data.Transform([1, 2, 3], [4, 5, 6, 7], [8, 9, 10])
            s = repr(t)
            assert s == 'ssbh_data_py.anim_data.Transform([1, 2, 3], [4, 5, 6, 7], [8, 9, 10])'
            t2 = eval(s)
            assert t2.scale == [1, 2, 3]
            assert t2.rotation == [4, 5, 6, 7]
            assert t2.translation == [8, 9, 10]
        "#})
    .unwrap();
}

#[test]
fn create_uv_transform() {
    run_python_code(indoc! {r#"
            t = ssbh_data_py.anim_data.UvTransform(1,2,3,4,5)
            assert t.scale_u == 1
            assert t.scale_v == 2
            assert t.rotation == 3
            assert t.translate_u == 4
            assert t.translate_v == 5
        "#})
    .unwrap();
}

#[test]
fn uv_transform_repr() {
    // Check that repr can be used to construct the type.
    run_python_code(indoc! {r#"
            t = ssbh_data_py.anim_data.UvTransform(1,2,3,4,5)
            s = repr(t)
            assert s == 'ssbh_data_py.anim_data.UvTransform(1, 2, 3, 4, 5)'
            t2 = eval(s)
            assert t2.scale_u == 1
            assert t2.scale_v == 2
            assert t2.rotation == 3
            assert t2.translate_u == 4
            assert t2.translate_v == 5
        "#})
    .unwrap();
}

#[test]
#[should_panic]
fn group_type_is_immutable() {
    run_python_code(indoc! {r#"
            g = ssbh_data_py.anim_data.GroupType.Transform
            g.name = 'abc'
            g.value = 4
        "#})
    .unwrap();
}

#[test]
fn group_type_classattrs() {
    run_python_code(indoc! {r#"
            g = ssbh_data_py.anim_data.GroupType.Transform
            assert g.name == 'Transform' and g.value == 1

            g = ssbh_data_py.anim_data.GroupType.Visibility
            assert g.name == 'Visibility' and g.value == 2

            g = ssbh_data_py.anim_data.GroupType.Material
            assert g.name == 'Material' and g.value == 4

            g = ssbh_data_py.anim_data.GroupType.Camera
            assert g.name == 'Camera' and g.value == 5
        "#})
    .unwrap();
}

#[test]
fn create_group_types_py() {
    let g: GroupType = ssbh_data::anim_data::GroupType::Transform.into();
    assert_eq!("Transform", g.name);
    assert_eq!(ssbh_data::anim_data::GroupType::Transform as u64, g.value);

    let g: GroupType = ssbh_data::anim_data::GroupType::Visibility.into();
    assert_eq!("Visibility", g.name);
    assert_eq!(ssbh_data::anim_data::GroupType::Visibility as u64, g.value);

    let g: GroupType = ssbh_data::anim_data::GroupType::Camera.into();
    assert_eq!("Camera", g.name);
    assert_eq!(ssbh_data::anim_data::GroupType::Camera as u64, g.value);

    let g: GroupType = ssbh_data::anim_data::GroupType::Material.into();
    assert_eq!("Material", g.name);
    assert_eq!(ssbh_data::anim_data::GroupType::Material as u64, g.value);
}

#[test]
fn create_track_values_rs_floats() {
    eval_python_code("[0.5, 1, 3.4]", |py, x| {
        let data = x.downcast::<PyList>().unwrap().as_unbound();
        assert_eq!(
            ssbh_data::anim_data::TrackValues::Float(vec![0.5, 1.0, 3.4]),
            create_track_values_rs(py, data).unwrap()
        );
    });
}

#[test]
fn create_track_values_rs_pattern_index() {
    eval_python_code("[0, 1, 2, 3]", |py, x| {
        let data = x.downcast::<PyList>().unwrap().as_unbound();
        assert_eq!(
            ssbh_data::anim_data::TrackValues::PatternIndex(vec![0, 1, 2, 3]),
            create_track_values_rs(py, data).unwrap()
        );
    });
}

#[test]
fn create_track_values_rs_bool() {
    eval_python_code("[True, False, True]", |py, x| {
        let data = x.downcast::<PyList>().unwrap().as_unbound();
        assert_eq!(
            ssbh_data::anim_data::TrackValues::Boolean(vec![true, false, true]),
            create_track_values_rs(py, data).unwrap()
        );
    });
}

#[test]
fn create_track_values_rs_vector4() {
    eval_python_code("[[1, 2, 3, 4], [0.5, 0.25, 0.3, 0.1]]", |py, x| {
        let data = x.downcast::<PyList>().unwrap().as_unbound();
        assert_eq!(
            ssbh_data::anim_data::TrackValues::Vector4(vec![
                Vector4::new(1.0, 2.0, 3.0, 4.0),
                Vector4::new(0.5, 0.25, 0.3, 0.1)
            ]),
            create_track_values_rs(py, data).unwrap()
        );
    });
}

#[test]
fn create_track_values_rs_transform() {
    eval_python_code(
        indoc! {r#"
                [ssbh_data_py.anim_data.Transform([1, 2, 3], [4, 5, 6, 7], [1, 2, 3]), 
                 ssbh_data_py.anim_data.Transform(
                    scale=[0, 1, 2],
                    rotation=[1, 2, 3, 4],
                    translation=[9, 8, 0.4])]
            "#},
        |py, x| {
            let data = x.downcast::<PyList>().unwrap().as_unbound();
            assert_eq!(
                ssbh_data::anim_data::TrackValues::Transform(vec![
                    ssbh_data::anim_data::Transform {
                        rotation: Vector4::new(4.0, 5.0, 6.0, 7.0),
                        translation: Vector3::new(1.0, 2.0, 3.0),
                        scale: Vector3::new(1.0, 2.0, 3.0),
                    },
                    ssbh_data::anim_data::Transform {
                        rotation: Vector4::new(1.0, 2.0, 3.0, 4.0),
                        translation: Vector3::new(9.0, 8.0, 0.4),
                        scale: Vector3::new(0.0, 1.0, 2.0),
                    }
                ]),
                create_track_values_rs(py, data).unwrap()
            );
        },
    );
}

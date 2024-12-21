use indoc::indoc;
use pyo3::{create_exception, prelude::*, types::IntoPyDict};
use ssbh_data_py_types::{python_enum, MapPy};
use strum::{Display, FromRepr};

fn run_test_python(code: &str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "test_module").unwrap();
        module.add_class::<TestEnumPy>().unwrap();
        let ctx = [("test_module", module)].into_py_dict(py).unwrap();
        py.run(&std::ffi::CString::new(code).unwrap(), None, Some(&ctx))
    })
}

create_exception!(ssbh_data_py, TestError, pyo3::exceptions::PyException);

#[derive(Display, FromRepr, Clone, Copy)]
enum TestEnumRs {
    A = 2,
    B = 7,
    C = 4,
}

python_enum!(TestEnumPy, TestEnumRs, TestError, "module_name", A, B, C);

#[test]
fn python_enum_conversions() {
    let e: TestEnumPy = TestEnumRs::A.into();
    assert_eq!("A", e.name);
    assert_eq!(2, e.value);

    let e: TestEnumPy = TestEnumRs::B.into();
    assert_eq!("B", e.name);
    assert_eq!(7, e.value);

    let e: TestEnumPy = TestEnumRs::C.into();
    assert_eq!("C", e.name);
    assert_eq!(4, e.value);
}

#[test]
fn python_enum_name_value() {
    run_test_python(indoc! {r#"
        t = test_module.TestEnumPy.A
        assert t.name == 'A' and t.value == 2

        t = test_module.TestEnumPy.B
        assert t.name == 'B' and t.value == 7

        t = test_module.TestEnumPy.C
        assert t.name == 'C' and t.value == 4
    "#})
    .unwrap();
}

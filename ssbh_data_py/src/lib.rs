use pyo3::{prelude::*, types::PyList};

#[cfg(test)]
use pyo3::types::IntoPyDict;

// External crates won't depend on ssbh_data_py, so just make everything public for convenience.
pub mod adj_data;
pub mod anim_data;
pub mod matl_data;
pub mod mesh_data;
pub mod meshex_data;
pub mod modl_data;
pub mod skel_data;

mod pyi;
pub use pyi::*;

mod map_py;
pub use map_py::*;

mod repr;
pub use repr::*;

#[pymodule]
fn ssbh_data_py(py: Python, module: &PyModule) -> PyResult<()> {
    crate::mesh_data::mesh_data(py, module)?;
    crate::modl_data::modl_data(py, module)?;
    crate::skel_data::skel_data(py, module)?;
    crate::anim_data::anim_data(py, module)?;
    crate::adj_data::adj_data(py, module)?;
    crate::matl_data::matl_data(py, module)?;
    crate::meshex_data::meshex_data(py, module)?;
    Ok(())
}

pub(crate) fn create_py_list_from_slice<T: IntoPy<U> + Copy, U: ToPyObject>(
    py: Python,
    elements: &[T],
) -> Py<PyList> {
    PyList::new(py, elements.iter().map(|m| m.into_py(py))).into()
}

#[macro_export]
macro_rules! python_enum {
    ($ty_py:ident, $ty_rs:ty, $ty_err:ty, $module:literal) => {
        // TODO: Change this to be a proper Python enum once supported by PyO3.
        // Try to match the interface from here: https://docs.python.org/3/library/enum.html
        #[pyclass(module = $module)]
        #[derive(Debug, Clone)]
        pub struct $ty_py {
            #[pyo3(get)]
            pub name: String,

            #[pyo3(get)]
            pub value: u64,
        }

        impl From<$ty_rs> for $ty_py {
            fn from(group_type: $ty_rs) -> Self {
                Self {
                    name: group_type.to_string(),
                    value: group_type as u64,
                }
            }
        }

        impl MapPy<$ty_rs> for $ty_py {
            fn map_py(&self, _py: Python, _use_numpy: bool) -> PyResult<$ty_rs> {
                <$ty_rs>::from_repr(self.value as usize).ok_or(<$ty_err>::new_err(format!(
                    "{} is not a supported variant.",
                    self.value
                )))
            }
        }

        impl MapPy<$ty_py> for $ty_rs {
            fn map_py(&self, _py: Python, _use_numpy: bool) -> PyResult<$ty_py> {
                Ok((*self).into())
            }
        }

        impl crate::PyTypeString for $ty_py {
            fn py_type_string() -> String {
                stringify!($ty_py).to_string()
            }
        }

        impl crate::PyRepr for $ty_py {
            fn py_repr(&self) -> String {
                // Match the behavior of Python's Enum class.
                format!("<{}.{}: {}>", stringify!($ty_py), self.name, self.value)
            }
        }

        #[pyproto]
        impl pyo3::PyObjectProtocol for $ty_py {
            fn __repr__(&self) -> String {
                self.py_repr()
            }
        }
    };
}

#[cfg(test)]
fn run_python_code(code: &str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);
        py.run(code, None, Some(ctx))
    })
}

#[cfg(test)]
fn run_python_code_numpy(code: &str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, module).unwrap();

        // This requires numpy to be in the current Python environment.
        // This may require some configuration to run tests with github actions.
        let ctx = [
            ("ssbh_data_py", module),
            ("numpy", PyModule::import(py, "numpy").unwrap()),
        ]
        .into_py_dict(py);

        py.run(code, None, Some(ctx))
    })
}

#[cfg(test)]
fn eval_python_code<F: Fn(Python, &PyAny)>(code: &str, f: F) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py);

        let result = py.eval(code, None, Some(ctx)).unwrap();
        f(py, result);
    })
}

#[cfg(test)]
fn eval_python_code_numpy<F: Fn(Python, &PyAny)>(code: &str, f: F) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, module).unwrap();

        // This requires numpy to be in the current Python environment.
        // This may require some configuration to run tests with github actions.
        let ctx = [
            ("ssbh_data_py", module),
            ("numpy", PyModule::import(py, "numpy").unwrap()),
        ]
        .into_py_dict(py);

        let result = py.eval(code, None, Some(ctx)).unwrap();
        f(py, result);
    });
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use pyo3::create_exception;
    use strum::{Display, FromRepr};

    use super::*;
    
    fn run_test_python(code: &str) -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let module = PyModule::new(py, "test_module").unwrap();
            module.add_class::<TestEnumPy>().unwrap();
            let ctx = [("test_module", module)].into_py_dict(py);
            py.run(code, None, Some(ctx))
        })
    }

    create_exception!(ssbh_data_py, TestError, pyo3::exceptions::PyException);

    #[derive(Display, FromRepr, Clone, Copy)]
    enum TestEnumRs {
        A = 2,
        B = 7,
        C = 4,
    }

    python_enum!(TestEnumPy, TestEnumRs, TestError, "module_name");

    #[pymethods]
    impl TestEnumPy {
        #[classattr]
        #[pyo3(name = "A")]
        fn a() -> TestEnumPy {
            TestEnumRs::A.into()
        }

        #[classattr]
        #[pyo3(name = "B")]
        fn b() -> TestEnumPy {
            TestEnumRs::B.into()
        }

        #[classattr]
        #[pyo3(name = "C")]
        fn c() -> TestEnumPy {
            TestEnumRs::C.into()
        }
    }

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

    #[test]
    fn python_enum_repr() {
        run_test_python(indoc! {r#"
            assert repr(test_module.TestEnumPy.A) == '<TestEnumPy.A: 2>'
            assert repr(test_module.TestEnumPy.B) == '<TestEnumPy.B: 7>'
            assert repr(test_module.TestEnumPy.C) == '<TestEnumPy.C: 4>'
        "#})
        .unwrap();
    }
}

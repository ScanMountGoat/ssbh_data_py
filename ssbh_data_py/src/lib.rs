use pyo3::{prelude::*, types::PyList};

#[cfg(test)]
use pyo3::types::IntoPyDict;

mod adj_data;
mod anim_data;
mod matl_data;
mod mesh_data;
mod modl_data;
mod skel_data;

#[pymodule]
fn ssbh_data_py(py: Python, module: &PyModule) -> PyResult<()> {
    crate::mesh_data::mesh_data(py, module)?;
    crate::modl_data::modl_data(py, module)?;
    crate::skel_data::skel_data(py, module)?;
    crate::anim_data::anim_data(py, module)?;
    crate::adj_data::adj_data(py, module)?;
    crate::matl_data::matl_data(py, module)?;
    Ok(())
}

fn create_py_list_from_slice<T: IntoPy<U> + Copy, U: ToPyObject>(
    py: Python,
    elements: &[T],
) -> Py<PyList> {
    PyList::new(py, elements.iter().map(|m| m.into_py(py))).into()
}

// Define a mapping between types.
// This allows for deriving the Python <-> Rust conversion.
// TODO: It may be possible to use PyO3 for this in the future.
// The derive macro is mainly to automate mapping field names.
trait MapPy<T> {
    fn map_py(&self, py: Python) -> PyResult<T>;
}

// We want a conversion from Vec<T> -> Py<PyList>.
// We can't implement ToPyObject for ssbh_lib types in ssbh_data_py.
// Use MapPy<PyObject> instead to utilize the ssbh_data -> ssbh_data_py conversion.
impl<T: MapPy<PyObject>> MapPy<Py<PyList>> for Vec<T> {
    fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
        Ok(PyList::new(
            py,
            self.iter()
                .map(|e| e.map_py(py))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .into())
    }
}

// Similarly, we need to define a conversion from Py<PyList> -> Vec<T>.
// The element type of a PyList is PyAny, so we can use a mapping from PyObject (Py<PyAny>) to T.
impl<T> MapPy<Vec<T>> for Py<PyList>
where
    PyObject: MapPy<T>,
{
    fn map_py(&self, py: Python) -> PyResult<Vec<T>> {
        self.as_ref(py)
            .iter()
            .map(|e| PyObject::from(e).map_py(py))
            .collect::<Result<Vec<_>, _>>()
    }
}

// Implement for primitive types.
macro_rules! map_py_impl {
    ($($t:ty),*) => {
        $(
            impl MapPy<$t> for $t {
                fn map_py(&self, _py: Python) -> PyResult<$t> {
                    Ok(self.clone())
                }
            }

            // Define the Rust <-> Python conversion to support the Vec <-> PyList conversion.
            impl MapPy<PyObject> for $t {
                fn map_py(
                    &self,
                    py: Python,
                ) -> PyResult<PyObject> {
                    Ok(self.into_py(py))
                }
            }

            impl MapPy<$t> for PyObject {
                fn map_py(&self, py: Python) -> PyResult<$t> {
                    self.extract(py)
                }
            }
        )*
    }
}

map_py_impl!(bool, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, f32, f64, String);

macro_rules! map_py_pyobject_impl {
    ($($t:ty),*) => {
        $(
            impl MapPy<$t> for PyObject {
                fn map_py(&self, py: Python) -> PyResult<$t> {
                    self.extract(py)
                }
            }
        )*
    }
}

// TODO: Derive this?
map_py_pyobject_impl!([[f32; 4]; 4]);
impl MapPy<PyObject> for [[f32; 4]; 4] {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(create_py_list_from_slice(py, self).into())
    }
}

map_py_pyobject_impl!(Vec<u32>);
impl MapPy<PyObject> for Vec<u32> {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(create_py_list_from_slice(py, self).into())
    }
}

map_py_pyobject_impl!(Vec<i16>);
impl MapPy<PyObject> for Vec<i16> {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(create_py_list_from_slice(py, self).into())
    }
}

impl<T: MapPy<U>, U> MapPy<Option<U>> for Option<T> {
    fn map_py(&self, py: Python) -> PyResult<Option<U>> {
        match self {
            Some(x) => Ok(Some(x.map_py(py)?)),
            None => Ok(None),
        }
    }
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

            // TODO: Customize this data type?
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
            fn map_py(&self, _py: Python) -> PyResult<$ty_rs> {
                <$ty_rs>::from_repr(self.value as usize).ok_or(<$ty_err>::new_err(format!(
                    "{} is not a supported variant.",
                    self.value
                )))
            }
        }

        impl MapPy<$ty_py> for $ty_rs {
            fn map_py(&self, _py: Python) -> PyResult<$ty_py> {
                Ok((*self).into())
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
    use crate::{eval_python_code, run_python_code};
    use indoc::indoc;
    use pyo3::create_exception;
    use ssbh_data::anim_data::{Vector3, Vector4};
    use strum::{Display, FromRepr};

    use super::*;

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
    fn python_enum_stuff() {
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

    fn run_test_python(code: &str) -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let module = PyModule::new(py, "test_module").unwrap();
            module.add_class::<TestEnumPy>().unwrap();
            let ctx = [("test_module", module)].into_py_dict(py);
            py.run(code, None, Some(ctx))
        })
    }
}

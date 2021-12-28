use pyo3::{prelude::*, types::PyList};

/// A trait to make it easier to implement the __repr__ method.
pub trait PyRepr {
    fn py_repr(&self) -> String;
}

macro_rules! py_repr_impl {
    ($($ty:ty),*) => {
        $(
            impl PyRepr for $ty {
                fn py_repr(&self) -> String {
                    format!("{}", self)
                }
            }
        )*
    };
}

py_repr_impl!(u16, u32, u64, usize, i16, i32, i64, f32);

impl PyRepr for bool {
    fn py_repr(&self) -> String {
        // Python capitalizes boolean literals.
        if *self {
            "True".to_string()
        } else {
            "False".to_string()
        }
    }
}

impl PyRepr for String {
    fn py_repr(&self) -> String {
        // Python uses single quotes instead of Rust's double quotes.
        format!("'{}'", self)
    }
}

// Simply calling Display produces the correct formatting.
// The tests below check that this doesn't break.
impl PyRepr for Py<PyList> {
    fn py_repr(&self) -> String {
        format!("{}", self)
    }
}

impl PyRepr for PyObject {
    fn py_repr(&self) -> String {
        format!("{}", self)
    }
}

impl<T: PyRepr> PyRepr for Option<T> {
    fn py_repr(&self) -> String {
        self.as_ref()
            .map(|t| t.py_repr())
            .unwrap_or_else(|| "None".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pyo3::types::IntoPyDict;
    use ssbh_data_py_derive::PyRepr;

    fn run_test_python(code: &str) -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let module = PyModule::new(py, "test_module").unwrap();
            module.add_class::<TestClass>().unwrap();
            module.add_class::<TestClassInner>().unwrap();
            let ctx = [("test_module", module)].into_py_dict(py);
            py.run(code, None, Some(ctx))
        })
    }

    #[pyclass()]
    #[derive(Debug, Clone, PyRepr)]
    #[pyrepr("test_module")]
    pub struct TestClass {
        #[pyo3(get, set)]
        pub a: f32,

        #[pyo3(get, set)]
        pub b: Py<PyList>,

        #[pyo3(get, set)]
        pub c: PyObject,

        #[pyo3(get, set)]
        pub d: String,
    }

    #[pyclass()]
    #[derive(Debug, Clone, PyRepr)]
    #[pyrepr("test_module")]
    pub struct TestClassInner {
        #[pyo3(get, set)]
        pub a: usize,

        #[pyo3(get, set)]
        pub b: bool,
    }

    #[pymethods]
    impl TestClassInner {
        #[new]
        fn new(_py: Python, a: usize, b: bool) -> PyResult<Self> {
            Ok(Self { a, b })
        }
    }

    #[pymethods]
    impl TestClass {
        #[new]
        fn new(_py: Python, a: f32, b: Py<PyList>, c: PyObject, d: String) -> PyResult<Self> {
            Ok(Self { a, b, c, d })
        }
    }

    #[test]
    fn test_class_repr() {
        // Check the repr is called correctly for inner types.
        run_test_python(indoc! {r#"
            inner = test_module.TestClassInner(3, False)
            outer = test_module.TestClass(0.5, [inner], inner, 'hello')
            assert repr(outer) == "test_module.TestClass(0.5, [test_module.TestClassInner(3, False)], test_module.TestClassInner(3, False), 'hello')"
        "#})
        .unwrap();
    }

    #[test]
    fn test_class_repr_eval() {
        // Check that repr can be used to construct the type.
        run_test_python(indoc! {r#"
            inner = test_module.TestClassInner(3, False)
            outer = test_module.TestClass(0.5, [inner], inner, 'hello')
            outer_new = eval(repr(outer))
        
            assert outer_new.a == 0.5

            assert len(outer_new.b) == 1
            assert outer_new.b[0].a == 3
            assert outer_new.b[0].b == False

            assert outer_new.c.a == 3
            assert outer_new.c.b == False
            
            assert outer_new.d == 'hello'
        "#})
        .unwrap();
    }
}

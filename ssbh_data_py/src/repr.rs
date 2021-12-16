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
                    format!("{:?}", self)
                }
            }
        )*
    };
}

py_repr_impl!(bool, u16, u32, u64, usize, i16, i32, i64, String, f32);

// TODO: Investigate why this works to just use the Display implementation.
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

impl <T: PyRepr> PyRepr for Option<T> {
    fn py_repr(&self) -> String {
        self.as_ref().map(|t| t.py_repr()).unwrap_or("None".to_string())
    }
}
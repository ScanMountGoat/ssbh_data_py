use map_py::TypedList;
use pyo3::{prelude::*, PyTypeInfo};

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
impl<T: PyTypeInfo> PyRepr for Py<T> {
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

impl<T> PyRepr for TypedList<T> {
    fn py_repr(&self) -> String {
        self.list.py_repr()
    }
}

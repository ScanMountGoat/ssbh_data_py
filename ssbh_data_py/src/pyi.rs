use pyo3::{prelude::*, types::PyList};

/// A trait for generating a type's corresponding Python class for Python type stub files (.pyi).
pub trait Pyi: PyiClass {
    fn pyi() -> String {
        // TODO: Combine class attributes, methods, etc
        Self::pyi_class()
    }
}

pub trait PyiClass {
    fn pyi_class() -> String;
}

/// A trait for defining the corresponding python type for a Rust type for Python type stub files (.pyi).
pub trait PyTypeString {
    fn py_type_string() -> String;
}

macro_rules! py_type_string_primitive_impl {
    ($py:literal, $($rs:ty),*) => {
        $(
            impl PyTypeString for $rs {
                fn py_type_string() -> String {
                    $py.to_string()
                }
            }
        )*
    };
}

py_type_string_primitive_impl!("bool", bool);
py_type_string_primitive_impl!("str", &str, String);
py_type_string_primitive_impl!("float", f32, f64);
py_type_string_primitive_impl!("int", u8, u16, u32, u64);

impl<T: PyTypeString> PyTypeString for Option<T> {
    fn py_type_string() -> String {
        format!("Optional[{}]", T::py_type_string())
    }
}

// This will likely be set manually using the Pyi derive helper attribute.
// Defaulting to "Any" essentially disables type checking.
impl PyTypeString for Py<PyList> {
    fn py_type_string() -> String {
        "list[Any]".to_string()
    }
}

impl PyTypeString for PyObject {
    fn py_type_string() -> String {
        "Any".to_string()
    }
}

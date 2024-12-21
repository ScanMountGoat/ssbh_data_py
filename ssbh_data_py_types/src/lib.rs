use pyo3::{
    prelude::*,
    types::{IntoPyDict, PyList},
};

// External crates won't depend on ssbh_data_py, so just make everything public for convenience.
pub mod adj_data;
pub mod anim_data;
pub mod hlpb_data;
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
pub use ssbh_data_py_derive::{MapPy, PyInit, PyRepr, Pyi};

pub fn ssbh_data_py(py: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    crate::adj_data::adj_data(py, module)?;
    crate::anim_data::anim_data(py, module)?;
    crate::hlpb_data::hlpb_data(py, module)?;
    crate::matl_data::matl_data(py, module)?;
    crate::mesh_data::mesh_data(py, module)?;
    crate::meshex_data::meshex_data(py, module)?;
    crate::modl_data::modl_data(py, module)?;
    crate::skel_data::skel_data(py, module)?;

    module.add("AdjDataError", py.get_type::<adj_data::AdjDataError>())?;
    module.add("AnimDataError", py.get_type::<anim_data::AnimDataError>())?;
    module.add("HlpbDataError", py.get_type::<hlpb_data::HlpbDataError>())?;
    module.add("MatlDataError", py.get_type::<matl_data::MatlDataError>())?;
    module.add("MeshDataError", py.get_type::<mesh_data::MeshDataError>())?;
    module.add(
        "MeshExDataError",
        py.get_type::<meshex_data::MeshExDataError>(),
    )?;
    module.add("ModlDataError", py.get_type::<modl_data::ModlDataError>())?;
    module.add("SkelDataError", py.get_type::<skel_data::SkelDataError>())?;

    Ok(())
}

pub(crate) fn create_py_list_from_slice<T, U>(py: Python, elements: &[T]) -> PyResult<Py<PyList>>
where
    T: IntoPy<U> + Copy,
    for<'a> U: IntoPyObject<'a>,
{
    PyList::new(py, elements.iter().map(|m| m.into_py(py))).map(Into::into)
}

#[macro_export]
macro_rules! python_enum {
    ($ty_py:ident, $ty_rs:ty, $ty_err:ty, $module:literal, $( $i:ident ),+) => {
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

        // TODO: match the variants instead?
        impl From<$ty_rs> for $ty_py {
            fn from(value: $ty_rs) -> Self {
                Self {
                    name: value.to_string(),
                    value: value as u64,
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

        impl $crate::PyTypeString for $ty_py {
            fn py_type_string() -> String {
                stringify!($ty_py).to_string()
            }
        }

        impl $crate::PyRepr for $ty_py {
            fn py_repr(&self) -> String {
                // Match the behavior of Python's Enum class.
                format!("<{}.{}: {}>", stringify!($ty_py), self.name, self.value)
            }
        }

        #[pymethods]
        impl $ty_py {
            fn __repr__(&self) -> String {
                <Self as $crate::PyRepr>::py_repr(self)
            }

            fn __richcmp__(&self, other: Self, op: pyo3::basic::CompareOp) -> PyResult<bool> {
                match op {
                    pyo3::basic::CompareOp::Lt => Ok(self.value < other.value),
                    pyo3::basic::CompareOp::Le => Ok(self.value <= other.value),
                    pyo3::basic::CompareOp::Eq => Ok(self.value == other.value),
                    pyo3::basic::CompareOp::Ne => Ok(self.value != other.value),
                    pyo3::basic::CompareOp::Gt => Ok(self.value > other.value),
                    pyo3::basic::CompareOp::Ge => Ok(self.value >= other.value),
                }
            }

            // The function name casing should match the variant name.
            $(
                #[allow(non_snake_case)]
                #[classattr]
                pub fn $i() -> $ty_py {
                    <$ty_rs>::$i.into()
                }
            )*
        }

        impl $crate::PyiClass for $ty_py {
            fn pyi_class() -> String {
                format!("class {}:\n    name: str\n    value: int", stringify!($ty_py))
            }
        }

        impl $crate::PyiMethods for $ty_py {
            fn pyi_methods() -> String {
                let mut out = String::new();

                $(
                    out += &format!("    {}: ClassVar[{}]\n", stringify!($i), stringify!($ty_py));
                )*
                out += "\n";

                out += "    @staticmethod\n";
                out += &format!("    def from_value(value: int) -> Optional[{}]: ...\n\n", stringify!($ty_py));

                out += "    @staticmethod\n";
                out += &format!("    def from_str(value: str) -> Optional[{}]: ...", stringify!($ty_py));

                out
            }
        }
    };
}

pub fn run_python_code(code: &str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py).unwrap();
        py.run(&std::ffi::CString::new(code).unwrap(), None, Some(&ctx))
    })
}

pub fn run_python_code_numpy(code: &str) -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();

        // This requires numpy to be in the current Python environment.
        // This may require some configuration to run tests with github actions.
        let ctx = [
            ("ssbh_data_py", module),
            ("np", PyModule::import(py, "numpy").unwrap()),
        ]
        .into_py_dict(py)
        .unwrap();

        py.run(&std::ffi::CString::new(code).unwrap(), None, Some(&ctx))
    })
}

pub fn eval_python_code<F: Fn(Python, Bound<'_, PyAny>)>(code: &str, f: F) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();
        let ctx = [("ssbh_data_py", module)].into_py_dict(py).unwrap();

        let result = py
            .eval(&std::ffi::CString::new(code).unwrap(), None, Some(&ctx))
            .unwrap();
        f(py, result);
    })
}

pub fn eval_python_code_numpy<F: Fn(Python, Bound<'_, PyAny>)>(code: &str, f: F) {
    pyo3::prepare_freethreaded_python();
    Python::with_gil(|py| {
        let module = PyModule::new(py, "ssbh_data_py").unwrap();
        ssbh_data_py(py, &module).unwrap();

        // This requires numpy to be in the current Python environment.
        // This may require some configuration to run tests with github actions.
        let ctx = [
            ("ssbh_data_py", module),
            ("np", PyModule::import(py, "numpy").unwrap()),
        ]
        .into_py_dict(py)
        .unwrap();

        let result = py
            .eval(&std::ffi::CString::new(code).unwrap(), None, Some(&ctx))
            .unwrap();
        f(py, result);
    });
}

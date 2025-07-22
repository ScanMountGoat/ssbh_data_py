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
use map_py::{MapPy, TypedList};
pub use pyi::*;

mod repr;
use pyo3::prelude::*;
pub use repr::*;
pub use ssbh_data_py_derive::{PyInit, PyRepr, Pyi};

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

        impl ::map_py::MapPy<$ty_rs> for $ty_py {
            fn map_py(self, _py: Python) -> PyResult<$ty_rs> {
                <$ty_rs>::from_repr(self.value as usize).ok_or(<$ty_err>::new_err(format!(
                    "{} is not a supported variant.",
                    self.value
                )))
            }
        }

        impl ::map_py::MapPy<$ty_py> for $ty_rs {
            fn map_py(self, _py: Python) -> PyResult<$ty_py> {
                Ok(self.into())
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

fn map_from_vector3(value: ssbh_data::Vector3, py: Python) -> PyResult<TypedList<f32>> {
    vec![value.x, value.y, value.z].map_py(py)
}

fn map_into_vector3(value: TypedList<f32>, py: Python) -> PyResult<ssbh_data::Vector3> {
    let values: [f32; 3] = value.list.extract(py)?;
    let [x, y, z] = values;
    Ok(ssbh_data::Vector3 { x, y, z })
}

fn map_from_vector4(value: ssbh_data::Vector4, py: Python) -> PyResult<TypedList<f32>> {
    vec![value.x, value.y, value.z, value.w].map_py(py)
}

fn map_into_vector4(value: TypedList<f32>, py: Python) -> PyResult<ssbh_data::Vector4> {
    let values: [f32; 4] = value.list.extract(py)?;
    let [x, y, z, w] = values;
    Ok(ssbh_data::Vector4 { x, y, z, w })
}

fn map_from_color4f(value: ssbh_data::Color4f, py: Python) -> PyResult<TypedList<f32>> {
    vec![value.r, value.g, value.b, value.a].map_py(py)
}

fn map_into_color4f(value: TypedList<f32>, py: Python) -> PyResult<ssbh_data::Color4f> {
    let values: [f32; 4] = value.list.extract(py)?;
    let [r, g, b, a] = values;
    Ok(ssbh_data::Color4f { r, g, b, a })
}

use crate::create_py_list_from_slice;
use num_traits::AsPrimitive;
use numpy::{ndarray::Dim, PyArray, PyArray2, PyArrayMethods, ToPyArray};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyList};

// Define a mapping between types.
// This allows for deriving the Python <-> Rust conversion.
// The derive macro is mainly to automate mapping field names.
pub trait MapPy<T> {
    fn map_py(&self, py: Python) -> PyResult<T>;
}

// We want a conversion from Vec<T> -> Py<PyList>.
// We can't implement ToPyObject for ssbh_lib types in ssbh_data_py.
// Use MapPy<PyObject> instead to utilize the ssbh_data -> ssbh_data_py conversion.
impl<T: MapPy<PyObject>> MapPy<Py<PyList>> for Vec<T> {
    fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
        // TODO: Faster to check for numpy array.
        PyList::new(
            py,
            self.iter()
                .map(|e| e.map_py(py))
                .collect::<Result<Vec<_>, _>>()?,
        )
        .map(Into::into)
    }
}

// Similarly, we need to define a conversion from Py<PyList> -> Vec<T>.
// The element type of a PyList is PyAny, so we can use a mapping from PyObject (Py<PyAny>) to T.
impl<T> MapPy<Vec<T>> for Py<PyList>
where
    PyObject: MapPy<T>,
{
    fn map_py(&self, py: Python) -> PyResult<Vec<T>> {
        self.bind(py)
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

impl MapPy<[[f32; 4]; 4]> for PyObject {
    fn map_py(&self, py: Python) -> PyResult<[[f32; 4]; 4]> {
        // TODO: Faster to check for numpy array.
        self.extract::<[[f32; 4]; 4]>(py)
    }
}

impl MapPy<PyObject> for [[f32; 4]; 4] {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(PyArray::from_slice(py, self.as_flattened())
            .readwrite()
            .reshape((4, 4))
            .unwrap()
            .into_any()
            .into())
    }
}

map_py_pyobject_impl!(Vec<u32>);
impl MapPy<PyObject> for Vec<u32> {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.to_pyarray(py).into_any().into())
    }
}

map_py_pyobject_impl!(Vec<i16>);
impl MapPy<PyObject> for Vec<i16> {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.to_pyarray(py).into_any().into())
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

impl MapPy<ssbh_data::Color4f> for PyObject {
    fn map_py(&self, py: Python) -> PyResult<ssbh_data::Color4f> {
        let [r, g, b, a] = self.extract::<[f32; 4]>(py)?;
        Ok(ssbh_data::Color4f { r, g, b, a })
    }
}

impl MapPy<PyObject> for ssbh_data::Color4f {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        PyList::new(py, [self.r, self.g, self.b, self.a]).map(Into::into)
    }
}

impl MapPy<ssbh_data::Vector4> for Py<PyList> {
    fn map_py(&self, py: Python) -> PyResult<ssbh_data::Vector4> {
        let values: [f32; 4] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<Py<PyList>> for ssbh_data::Vector4 {
    fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
        PyList::new(py, self.to_array()).map(Into::into)
    }
}

impl MapPy<PyObject> for ssbh_data::Vector4 {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(self.to_array().into_pyobject(py)?.into_any().into())
    }
}

impl MapPy<ssbh_data::Vector4> for PyObject {
    fn map_py(&self, py: Python) -> PyResult<ssbh_data::Vector4> {
        let values: [f32; 4] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<ssbh_data::Vector3> for Py<PyList> {
    fn map_py(&self, py: Python) -> PyResult<ssbh_data::Vector3> {
        let values: [f32; 3] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<Py<PyList>> for ssbh_data::Vector3 {
    fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
        PyList::new(py, self.to_array()).map(Into::into)
    }
}

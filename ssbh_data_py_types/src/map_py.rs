use crate::create_py_list_from_slice;
use num_traits::AsPrimitive;
use numpy::{ndarray::Dim, PyArray, PyArray2};
use pyo3::{exceptions::PyValueError, prelude::*, types::PyList};

// Define a mapping between types.
// This allows for deriving the Python <-> Rust conversion.
// The derive macro is mainly to automate mapping field names.
pub trait MapPy<T> {
    fn map_py(&self, py: Python, use_numpy: bool) -> PyResult<T>;
}

// We want a conversion from Vec<T> -> Py<PyList>.
// We can't implement ToPyObject for ssbh_lib types in ssbh_data_py.
// Use MapPy<PyObject> instead to utilize the ssbh_data -> ssbh_data_py conversion.
impl<T: MapPy<PyObject>> MapPy<Py<PyList>> for Vec<T> {
    fn map_py(&self, py: Python, use_numpy: bool) -> PyResult<Py<PyList>> {
        Ok(PyList::new(
            py,
            self.iter()
                .map(|e| e.map_py(py, use_numpy))
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
    fn map_py(&self, py: Python, use_numpy: bool) -> PyResult<Vec<T>> {
        self.as_ref(py)
            .iter()
            .map(|e| PyObject::from(e).map_py(py, use_numpy))
            .collect::<Result<Vec<_>, _>>()
    }
}

// Implement for primitive types.
macro_rules! map_py_impl {
    ($($t:ty),*) => {
        $(
            impl MapPy<$t> for $t {
                fn map_py(&self, _py: Python, _use_numpy: bool) -> PyResult<$t> {
                    Ok(self.clone())
                }
            }

            // Define the Rust <-> Python conversion to support the Vec <-> PyList conversion.
            impl MapPy<PyObject> for $t {
                fn map_py(
                    &self,
                    py: Python,
                    _use_numpy: bool
                ) -> PyResult<PyObject> {
                    Ok(self.into_py(py))
                }
            }

            impl MapPy<$t> for PyObject {
                fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<$t> {
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
                fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<$t> {
                    self.extract(py)
                }
            }
        )*
    }
}

fn mat4x4<T: AsPrimitive<f32> + numpy::Element>(
    arr: &PyArray<T, Dim<[usize; 2]>>,
) -> PyResult<[[f32; 4]; 4]> {
    let a = arr.readonly();
    if let [4, 4] = a.shape() {
        let a = a.as_slice().unwrap();
        // Use AsPrimitive to allow truncating types like f64.
        // TODO: Is there a cleaner way of doing this?
        Ok([
            [a[0].as_(), a[1].as_(), a[2].as_(), a[3].as_()],
            [a[4].as_(), a[5].as_(), a[6].as_(), a[7].as_()],
            [a[8].as_(), a[9].as_(), a[10].as_(), a[11].as_()],
            [a[12].as_(), a[13].as_(), a[14].as_(), a[15].as_()],
        ])
    } else {
        Err(PyValueError::new_err(format!(
            "Expected shape [4, 4] but found {:?}",
            a.shape()
        )))
    }
}

impl MapPy<[[f32; 4]; 4]> for PyObject {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<[[f32; 4]; 4]> {
        self.extract::<[[f32; 4]; 4]>(py)
            .or_else(|_| self.extract::<&PyArray2<f32>>(py).and_then(mat4x4))
            .or_else(|_| self.extract::<&PyArray2<f64>>(py).and_then(mat4x4))
            .or_else(|_| self.extract::<&PyArray2<i8>>(py).and_then(mat4x4))
            .or_else(|_| self.extract::<&PyArray2<i16>>(py).and_then(mat4x4))
            .or_else(|_| self.extract::<&PyArray2<i32>>(py).and_then(mat4x4))
    }
}

impl MapPy<PyObject> for [[f32; 4]; 4] {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<PyObject> {
        Ok(create_py_list_from_slice(py, self).into())
    }
}

map_py_pyobject_impl!(Vec<u32>);
impl MapPy<PyObject> for Vec<u32> {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<PyObject> {
        // TODO: Use numpy?
        Ok(create_py_list_from_slice(py, self).into())
    }
}

map_py_pyobject_impl!(Vec<i16>);
impl MapPy<PyObject> for Vec<i16> {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<PyObject> {
        Ok(create_py_list_from_slice(py, self).into())
    }
}

impl<T: MapPy<U>, U> MapPy<Option<U>> for Option<T> {
    fn map_py(&self, py: Python, use_numpy: bool) -> PyResult<Option<U>> {
        match self {
            Some(x) => Ok(Some(x.map_py(py, use_numpy)?)),
            None => Ok(None),
        }
    }
}

impl MapPy<ssbh_data::Color4f> for PyObject {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<ssbh_data::Color4f> {
        let [r, g, b, a] = self.extract::<[f32; 4]>(py)?;
        Ok(ssbh_data::Color4f { r, g, b, a })
    }
}

impl MapPy<PyObject> for ssbh_data::Color4f {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<PyObject> {
        Ok(PyList::new(py, [self.r, self.g, self.b, self.a]).into())
    }
}

impl MapPy<ssbh_data::Vector4> for Py<PyList> {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<ssbh_data::Vector4> {
        let values: [f32; 4] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<Py<PyList>> for ssbh_data::Vector4 {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<Py<PyList>> {
        Ok(PyList::new(py, self.to_array()).into())
    }
}

impl MapPy<PyObject> for ssbh_data::Vector4 {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<PyObject> {
        Ok(self.to_array().into_py(py))
    }
}

impl MapPy<ssbh_data::Vector4> for PyObject {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<ssbh_data::Vector4> {
        let values: [f32; 4] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<ssbh_data::Vector3> for Py<PyList> {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<ssbh_data::Vector3> {
        let values: [f32; 3] = self.extract(py)?;
        Ok(values.into())
    }
}

impl MapPy<Py<PyList>> for ssbh_data::Vector3 {
    fn map_py(&self, py: Python, _use_numpy: bool) -> PyResult<Py<PyList>> {
        Ok(PyList::new(py, self.to_array()).into())
    }
}
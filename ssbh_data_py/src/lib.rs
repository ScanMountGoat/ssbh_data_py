use pyo3::type_object::PyBorrowFlagLayout;
use pyo3::PyClass;
use pyo3::{prelude::*, types::PyList};

mod anim_data;
mod mesh_data;
mod modl_data;
mod skel_data;

#[pymodule]
fn ssbh_data_py(py: Python, module: &PyModule) -> PyResult<()> {
    crate::mesh_data::mesh_data(py, module)?;
    crate::modl_data::modl_data(py, module)?;
    crate::skel_data::skel_data(py, module)?;
    crate::anim_data::anim_data(py, module)?;
    Ok(())
}

fn create_py_list<T, C: PyClass, U: Into<PyClassInitializer<C>>, F: Fn(Python, &T) -> PyResult<U>>(
    py: Python,
    elements: &[T],
    create_p: F,
) -> PyResult<Py<PyList>>
where
    C::BaseLayout: PyBorrowFlagLayout<C::BaseType>,
{
    let items: Result<Vec<_>, _> = elements
        .iter()
        .map(|e| Py::new(py, create_p(py, e)?))
        .collect();

    Ok(PyList::new(py, items?).into())
}

fn create_py_list_from_slice<T: IntoPy<U> + Copy, U: ToPyObject>(
    py: Python,
    elements: &[T],
) -> Py<PyList> {
    PyList::new(py, elements.iter().map(|m| m.into_py(py))).into()
}

fn create_vec<T, P: PyClass + Clone, F: Fn(Python, &P) -> PyResult<T>>(
    py: Python,
    elements: &Py<PyList>,
    create_t: F,
) -> PyResult<Vec<T>> {
    let python_elements: Result<Vec<P>, _> = elements
        .as_ref(py)
        .iter()
        .map(|i| i.extract::<P>())
        .collect();

    let rust_elements: Result<Vec<T>, _> =
        python_elements?.iter().map(|i| create_t(py, i)).collect();

    rust_elements
}

// Define a mapping between types.
// This allows for deriving the Python <-> Rust conversion.
// TODO: It may be possible to use Pyo3 for this in the future.
// The derive macro is mainly to automate mapping field names.
trait MapPy<T> {
    fn map_py(&self, py: Python) -> PyResult<T>;
}

macro_rules! map_py_impl {
    ($($t:ty),*) => {
        $(
            impl MapPy<$t> for $t {
                fn map_py(&self, _py: Python) -> PyResult<$t> {
                    Ok(self.clone())
                }
            }
        )*
    }
}

pub(crate) use map_py_impl;

map_py_impl!(bool, u8, u16, u32, u64, f32, f64, String);

// TODO: This can be a blanket implementation for anything that is MapPy and FromPyObject?
macro_rules! map_py_pylist_impl {
    ($t:ty) => {
        impl MapPy<Py<PyList>> for Vec<$t> {
            fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
                Ok(PyList::new(py, self.iter().map(|m| m.map_py(py).unwrap())).into())
            }
        }

        impl MapPy<Vec<$t>> for Py<PyList> {
            fn map_py(&self, py: Python) -> PyResult<Vec<$t>> {
                self.extract(py)
            }
        }
    };
    ($t:ty,$u:ty) => {
        impl MapPy<Vec<$t>> for Py<PyList> {
            fn map_py(&self, py: Python) -> PyResult<Vec<$t>> {
                // TODO: Avoid unwrap.
                Ok(self
                    .as_ref(py)
                    .iter()
                    .map(|i| i.extract::<$u>().unwrap().map_py(py).unwrap())
                    .collect())
            }
        }

        impl MapPy<Py<PyList>> for Vec<$t> {
            // TODO: Avoid unwrap.
            fn map_py(&self, py: Python) -> PyResult<Py<PyList>> {
                Ok(PyList::new(
                    py,
                    self.iter()
                        .map(|e| Py::new(py, e.map_py(py).unwrap()).unwrap()),
                )
                .into())
            }
        }
    };
}

pub(crate) use map_py_pylist_impl;

map_py_pylist_impl!(String);
map_py_pylist_impl!(u32);
map_py_pylist_impl!(f32);
map_py_pylist_impl!(bool);


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

pub(crate) use map_py_pyobject_impl;

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

impl<T: Clone> MapPy<Option<T>> for Option<T> {
    fn map_py(&self, py: Python) -> PyResult<Option<T>> {
        Ok(self.clone())
    }
}

#[cfg(test)]
fn run_python_code(code: &str) -> PyResult<()> {
    use pyo3::types::IntoPyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::new(py, "ssbh_data_py").unwrap();
    ssbh_data_py(py, module).unwrap();
    let ctx = [("ssbh_data_py", module)].into_py_dict(py);
    py.run(code, None, Some(ctx))
}

#[cfg(test)]
fn run_python_code_numpy(code: &str) -> PyResult<()> {
    use pyo3::types::IntoPyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::new(py, "ssbh_data_py").unwrap();
    ssbh_data_py(py, module).unwrap();

    // TODO: This requires numpy to be in the current Python environment,
    // which may require some configuration to run tests with github actions.
    let ctx = [
        ("ssbh_data_py", module),
        ("numpy", PyModule::import(py, "numpy").unwrap()),
    ]
    .into_py_dict(py);

    py.run(code, None, Some(ctx))
}

#[cfg(test)]
fn eval_python_code<F: Fn(Python, &PyAny)>(code: &str, f: F) {
    use pyo3::types::IntoPyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::new(py, "ssbh_data_py").unwrap();
    ssbh_data_py(py, module).unwrap();
    let ctx = [("ssbh_data_py", module)].into_py_dict(py);

    let result = py.eval(code, None, Some(ctx)).unwrap();
    f(py, result);
}

#[cfg(test)]
fn eval_python_code_numpy<F: Fn(Python, &PyAny)>(code: &str, f: F) {
    use pyo3::types::IntoPyDict;

    let gil = Python::acquire_gil();
    let py = gil.python();

    let module = PyModule::new(py, "ssbh_data_py").unwrap();
    ssbh_data_py(py, module).unwrap();

    // TODO: This requires numpy to be in the current Python environment,
    // which may require some configuration to run tests with github actions.
    let ctx = [
        ("ssbh_data_py", module),
        ("numpy", PyModule::import(py, "numpy").unwrap()),
    ]
    .into_py_dict(py);

    let result = py.eval(code, None, Some(ctx)).unwrap();
    f(py, result);
}

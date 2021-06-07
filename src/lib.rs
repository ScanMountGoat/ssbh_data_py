use pyo3::type_object::PyBorrowFlagLayout;
use pyo3::PyClass;
use pyo3::{prelude::*, types::PyList};

mod mesh_data;

#[pymodule]
fn ssbh_data_py(py: Python, module: &PyModule) -> PyResult<()> {
    crate::mesh_data::mesh_data(py, module)?;
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
        python_elements?.iter().map(|i| create_t(py, &i)).collect();

    rust_elements
}

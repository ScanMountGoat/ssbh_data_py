use pyo3::{prelude::*, types::IntoPyDict};

#[pymodule]
mod ssbh_data_py {
    #[pymodule_export]
    use ssbh_data_py_types::adj_data::adj_data;

    #[pymodule_export]
    use ssbh_data_py_types::anim_data::anim_data;

    #[pymodule_export]
    use ssbh_data_py_types::hlpb_data::hlpb_data;

    #[pymodule_export]
    use ssbh_data_py_types::matl_data::matl_data;

    #[pymodule_export]
    use ssbh_data_py_types::mesh_data::mesh_data;

    #[pymodule_export]
    use ssbh_data_py_types::meshex_data::meshex_data;

    #[pymodule_export]
    use ssbh_data_py_types::modl_data::modl_data;

    #[pymodule_export]
    use ssbh_data_py_types::skel_data::skel_data;

    #[pymodule_export]
    use adj_data::AdjDataError;

    #[pymodule_export]
    use anim_data::AnimDataError;

    #[pymodule_export]
    use hlpb_data::HlpbDataError;

    #[pymodule_export]
    use matl_data::MatlDataError;

    #[pymodule_export]
    use mesh_data::MeshDataError;

    #[pymodule_export]
    use meshex_data::MeshExDataError;

    #[pymodule_export]
    use modl_data::ModlDataError;

    #[pymodule_export]
    use skel_data::SkelDataError;
}

pub fn run_python_code(code: &str) -> PyResult<()> {
    // Check to avoid initializing twice in tests.
    if unsafe { pyo3::ffi::Py_IsInitialized() } == 0 {
        pyo3::append_to_inittab!(ssbh_data_py);
    }
    Python::initialize();
    Python::attach(|py| {
        // This requires numpy to be in the current Python environment.
        // This may require some configuration to run tests with github actions.
        let ctx = [
            (
                "ssbh_data_py",
                PyModule::import(py, "ssbh_data_py").unwrap(),
            ),
            ("numpy", PyModule::import(py, "numpy").unwrap()),
        ]
        .into_py_dict(py)
        .unwrap();

        py.run(&std::ffi::CString::new(code).unwrap(), None, Some(&ctx))
    })
}

pub fn eval_python_code<F: Fn(Python, Bound<'_, PyAny>)>(code: &str, f: F) {
    // Check to avoid initializing twice in tests.
    if unsafe { pyo3::ffi::Py_IsInitialized() } == 0 {
        pyo3::append_to_inittab!(ssbh_data_py);
    }
    Python::initialize();
    Python::attach(|py| {
        // This requires numpy to be in the current Python environment.
        // This may require some configuration to run tests with github actions.
        let ctx = [
            (
                "ssbh_data_py",
                PyModule::import(py, "ssbh_data_py").unwrap(),
            ),
            ("numpy", PyModule::import(py, "numpy").unwrap()),
        ]
        .into_py_dict(py)
        .unwrap();

        let result = py
            .eval(&std::ffi::CString::new(code).unwrap(), None, Some(&ctx))
            .unwrap();
        f(py, result);
    });
}

use crate::{MapPy, PyRepr, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data_py_derive::{MapPy, PyInit, PyRepr, Pyi};

create_exception!(ssbh_data_py, HlpbDataError, pyo3::exceptions::PyException);

pub fn hlpb_data(py: Python, module: &PyModule) -> PyResult<()> {
    let hlpb_data = PyModule::new(py, "hlpb_data")?;
    hlpb_data.add_class::<HlpbData>()?;
    hlpb_data.add_class::<AimConstraintData>()?;
    hlpb_data.add_class::<OrientConstraintData>()?;

    hlpb_data.add_function(wrap_pyfunction!(read_hlpb, hlpb_data)?)?;

    module.add_submodule(hlpb_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.hlpb_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::hlpb_data::HlpbData)]
#[pyrepr("ssbh_data_py.hlpb_data")]
#[pyi(has_methods = true)]
pub struct HlpbData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[AimConstraintData]")]
    pub aim_constraints: Py<PyList>,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[OrientConstraintData]")]
    pub orient_constraints: Py<PyList>,
}

#[pymethods]
impl HlpbData {
    #[new]
    #[pyo3(signature = (major_version = 1, minor_version = 0))]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(HlpbData {
            major_version,
            minor_version,
            aim_constraints: PyList::empty(py).into(),
            orient_constraints: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py, false)?
            .write_to_file(path)
            .map_err(PyErr::from)
    }

    fn __repr__(&self) -> String {
        self.py_repr()
    }
}

// TODO: Can we document the actual default value here?
// Add the default to some sort of derive attribute?
impl PyiMethods for HlpbData {
    fn pyi_methods() -> String {
        r#"    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 0,
    ) -> None: ...
    
    def save(self, path: str) -> None: ..."#
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.hlpb_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::hlpb_data::AimConstraintData)]
#[pyrepr("ssbh_data_py.hlpb_data")]
pub struct AimConstraintData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub aim_bone_name1: String,

    #[pyo3(get, set)]
    pub aim_bone_name2: String,

    #[pyo3(get, set)]
    pub target_bone_name1: String,

    #[pyo3(get, set)]
    pub target_bone_name2: String,

    #[pyo3(get, set)]
    #[pyinit(default = "\"DEFAULT\".into()")]
    #[pyi(default = "'Default'")]
    pub aim_type1: String,

    #[pyo3(get, set)]
    #[pyinit(default = "\"DEFAULT\".into()")]
    #[pyi(default = "'Default'")]
    pub aim_type2: String,

    #[pyo3(get, set)]
    #[pyinit(default = "0")]
    #[pyi(default = "0")]
    pub unk1: u32,

    #[pyo3(get, set)]
    #[pyinit(default = "0")]
    #[pyi(default = "0")]
    pub unk2: u32,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::new(py, [1.0, 0.0, 0.0]).into()")]
    #[pyi(python_type = "list[float]", default = "[1.0, 0.0, 0.0]")]
    pub aim: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::new(py, [0.0, 1.0, 0.0]).into()")]
    #[pyi(python_type = "list[float]", default = "[0.0, 1.0, 0.0]")]
    pub up: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::new(py, [0.0, 0.0, 0.0, 1.0]).into()")]
    #[pyi(python_type = "list[float]", default = "[0.0, 0.0, 0.0, 1.0]")]
    pub quat1: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::new(py, [0.0, 0.0, 0.0, 1.0]).into()")]
    #[pyi(python_type = "list[float]", default = "[0.0, 0.0, 0.0, 1.0]")]
    pub quat2: Py<PyList>,
}

#[pyclass(module = "ssbh_data_py.hlpb_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::hlpb_data::OrientConstraintData)]
#[pyrepr("ssbh_data_py.hlpb_data")]
pub struct OrientConstraintData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    pub parent_bone_name1: String,

    #[pyo3(get, set)]
    pub parent_bone_name2: String,

    #[pyo3(get, set)]
    pub source_bone_name: String,

    #[pyo3(get, set)]
    pub target_bone_name: String,

    #[pyo3(get, set)]
    pub unk_type: u32,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[float]")]
    pub constraint_axes: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::new(py, [0.0, 0.0, 0.0, 1.0]).into()")]
    #[pyi(python_type = "list[float]", default = "[0.0, 0.0, 0.0, 1.0]")]
    pub quat1: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::new(py, [0.0, 0.0, 0.0, 1.0]).into()")]
    #[pyi(python_type = "list[float]", default = "[0.0, 0.0, 0.0, 1.0]")]
    pub quat2: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::new(py, [-180.0, -180.0, -180.0]).into()")]
    #[pyi(python_type = "list[float]", default = "[-180.0, -180.0, -180.0]")]
    pub range_min: Py<PyList>,

    #[pyo3(get, set)]
    #[pyinit(default = "PyList::new(py, [180.0, 180.0, 180.0]).into()")]
    #[pyi(python_type = "list[float]", default = "[180.0, 180.0, 180.0]")]
    pub range_max: Py<PyList>,
}

#[pyfunction]
fn read_hlpb(py: Python, path: &str) -> PyResult<HlpbData> {
    ssbh_data::hlpb_data::HlpbData::from_file(path)
        .map_err(|e| HlpbDataError::new_err(format!("{}", e)))?
        .map_py(py, false)
}

#[cfg(test)]
mod tests {
    use crate::run_python_code;
    use indoc::indoc;

    #[test]
    fn read_hlpb() {
        // Test exceptions.
        run_python_code(indoc! {r#"
            try:
                ssbh_data_py.hlpb_data.read_hlpb("invalid")
            except ssbh_data_py.HlpbDataError as e:
                assert True
        "#})
        .unwrap();
    }

    #[test]
    fn create_hlpb() {
        run_python_code(indoc! {r#"
            h = ssbh_data_py.hlpb_data.HlpbData(3, 4)
            assert h.major_version == 3
            assert h.minor_version == 4
            assert h.aim_constraints == []
            assert h.orient_constraints == []

            m = ssbh_data_py.hlpb_data.HlpbData(3)
            assert m.major_version == 3
            assert m.minor_version == 0

            m = ssbh_data_py.hlpb_data.HlpbData()
            assert m.major_version == 1
            assert m.minor_version == 0
        "#})
        .unwrap();
    }

    // TODO: Test orient and aim constraint constructors.
    // TODO: Add defaults for fields like range min, range max, etc?
    #[test]
    fn create_aim_constraint() {
        run_python_code(indoc! {r#"
            a = ssbh_data_py.hlpb_data.AimConstraintData('a', 'p1', 'p2', 't1', 't2')
            assert a.name == 'a'
            assert a.aim_bone_name1 == 'p1'
            assert a.aim_bone_name2 == 'p2'
            assert a.target_bone_name1 == 't1'
            assert a.target_bone_name2 == 't2'
            assert a.aim_type1 == 'DEFAULT'
            assert a.aim_type2 == 'DEFAULT'
            assert a.unk1 == 0
            assert a.unk2 == 0
            assert a.aim == [1, 0, 0]
            assert a.up == [0, 1, 0]
            assert a.quat1 == [0, 0, 0, 1]
            assert a.quat2 == [0, 0, 0, 1]
        "#})
        .unwrap();
    }

    #[test]
    fn create_orient_constraint() {
        run_python_code(indoc! {r#"
            o = ssbh_data_py.hlpb_data.OrientConstraintData('a', 'p1', 'p2', 's', 't', 1, [0.1, 0.2, 0.3])
            assert o.name == 'a'
            assert o.parent_bone_name1 == 'p1'
            assert o.parent_bone_name2 == 'p2'
            assert o.source_bone_name == 's'
            assert o.target_bone_name == 't'
            assert o.unk_type == 1
            assert o.constraint_axes == [0.1, 0.2, 0.3]
            assert o.quat1 == [0, 0, 0, 1]
            assert o.quat2 == [0, 0, 0, 1]
            assert o.range_min == [-180, -180, -180]
            assert o.range_max == [180, 180, 180]
        "#})
        .unwrap();
    }
}

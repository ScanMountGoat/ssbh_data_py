use crate::{python_enum, MapPy, PyRepr, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data::SsbhData;
use ssbh_data_py_derive::{MapPy, PyInit, PyRepr, Pyi};

use crate::create_py_list_from_slice;

mod enums;

create_exception!(ssbh_data_py, SkelDataError, pyo3::exceptions::PyException);

pub fn skel_data(py: Python, module: &PyModule) -> PyResult<()> {
    let skel_data = PyModule::new(py, "skel_data")?;
    skel_data.add_class::<SkelData>()?;
    skel_data.add_class::<BoneData>()?;
    skel_data.add_class::<BillboardType>()?;

    skel_data.add_function(wrap_pyfunction!(read_skel, skel_data)?)?;
    skel_data.add_function(wrap_pyfunction!(calculate_relative_transform, skel_data)?)?;

    module.add_submodule(skel_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.skel_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
#[map(ssbh_data::skel_data::SkelData)]
#[pyrepr("ssbh_data_py.skel_data")]
#[pyi(has_methods = true)]
pub struct SkelData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[BoneData]")]
    pub bones: Py<PyList>,
}

#[pyclass(module = "ssbh_data_py.skel_data")]
#[derive(Debug, Clone, MapPy, Pyi, PyRepr, PyInit)]
#[map(ssbh_data::skel_data::BoneData)]
#[pyrepr("ssbh_data_py.skel_data")]
pub struct BoneData {
    #[pyo3(get, set)]
    pub name: String,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[list[float]]")]
    pub transform: PyObject,

    #[pyo3(get, set)]
    #[pyinit(default = "None")]
    #[pyi(default = "None")]
    pub parent_index: Option<usize>,

    #[pyo3(get, set)]
    #[pyinit(default = "ssbh_data::skel_data::BillboardType::None.into()")]
    #[pyi(default = "BillboardType.None")]
    pub billboard_type: BillboardType,
}

impl PyiMethods for SkelData {
    fn pyi_methods() -> String {
        r#"    def __init__(
        self,
        major_version: int = 1,
        minor_version: int = 0,
    ) -> None: ...

    def save(self, path: str) -> None: ...

    def calculate_world_transform(
        self, bone: BoneData) -> list[list[float]]: ..."#
            .to_string()
    }
}

#[pymethods]
impl SkelData {
    #[new]
    #[args(major_version = 1, minor_version = 0)]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(SkelData {
            major_version,
            minor_version,
            bones: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py, false)?
            .write_to_file(path)
            .map_err(|e| SkelDataError::new_err(format!("{}", e)))
    }

    fn calculate_world_transform(&self, py: Python, bone: &BoneData) -> PyResult<Py<PyList>> {
        let data: ssbh_data::skel_data::SkelData = self.map_py(py, false)?;
        let bone_data: ssbh_data::skel_data::BoneData = bone.map_py(py, false)?;
        let transform = data
            .calculate_world_transform(&bone_data)
            .map_err(|e| SkelDataError::new_err(format!("{}", e)))?;
        Ok(create_py_list_from_slice(py, &transform))
    }
}

python_enum!(
    BillboardType,
    ssbh_data::skel_data::BillboardType,
    SkelDataError,
    "ssbh_data_py.skel_data"
);

#[pyfunction]
fn read_skel(py: Python, path: &str) -> PyResult<SkelData> {
    ssbh_data::skel_data::SkelData::from_file(path)
        .map_err(|e| SkelDataError::new_err(format!("{}", e)))?
        .map_py(py, false)
}

#[pyfunction]
fn calculate_relative_transform(
    py: Python,
    world_transform: &PyAny,
    parent_world_transform: Option<&PyAny>,
) -> PyResult<Py<PyList>> {
    let world_transform = world_transform.extract()?;
    let transform = match parent_world_transform {
        Some(m) => ssbh_data::skel_data::calculate_relative_transform(
            &world_transform,
            Some(&m.extract()?),
        ),
        None => ssbh_data::skel_data::calculate_relative_transform(&world_transform, None),
    };
    Ok(create_py_list_from_slice(py, &transform))
}

#[cfg(test)]
mod tests {
    use crate::{run_python_code, run_python_code_numpy};
    use indoc::indoc;

    #[test]
    fn create_skel() {
        run_python_code(indoc! {r#"
            s = ssbh_data_py.skel_data.SkelData()
            assert s.major_version == 1
            assert s.minor_version == 0
            assert s.bones == []
        "#})
        .unwrap();
    }

    #[test]
    fn create_bone_data() {
        // TODO: Fix assertions to compare enums.
        // TODO: None doesn't work as a variant in Python.
        run_python_code(indoc! {r#"
            b = ssbh_data_py.skel_data.BoneData("abc", [[0,0,0,0]]*4, 5, ssbh_data_py.skel_data.BillboardType.YAxisViewPlaneAligned)
            assert b.name == "abc"
            assert b.transform == [[0,0,0,0]]*4
            assert b.parent_index == 5
            #assert b.billboard_type == ssbh_data_py.skel_data.BillboardType.YAxisViewPlaneAligned

            b = ssbh_data_py.skel_data.BoneData("abc", [[1,1,1,1]]*4, None)
            assert b.name == "abc"
            assert b.transform == [[1,1,1,1]]*4
            assert b.parent_index == None
            #assert b.billboard_type == ssbh_data_py.skel_data.BillboardType.None
            # Test mutability.
            b.transform[1][2] = 3
            assert b.transform[1] == [1,1,3,1]
        "#})
        .unwrap();
    }

    #[test]
    fn create_bone_data_tuples() {
        run_python_code(indoc! {r#"
            billboard = ssbh_data_py.skel_data.BillboardType.YAxisViewPlaneAligned
            b = ssbh_data_py.skel_data.BoneData("abc", [(0,0,0,0)]*4, 5, billboard)
            assert b.name == "abc"
            assert b.transform == [[0,0,0,0]]*4
            assert b.parent_index == 5

            b = ssbh_data_py.skel_data.BoneData("abc", [(1,1,1,1)]*4)
            assert b.name == "abc"
            assert b.transform == [[1,1,1,1]]*4
            assert b.parent_index == None
            # Test mutability.
            b.transform[1][2] = 3
            assert b.transform[1] == [1,1,3,1]
        "#})
        .unwrap();
    }

    #[test]
    fn create_bone_data_numpy() {
        run_python_code_numpy(indoc! {r#"
            b = ssbh_data_py.skel_data.BoneData("abc", numpy.zeros((4,4)), 5)
            assert b.name == "abc"
            assert b.transform.tolist() == [[0,0,0,0]]*4
            assert b.parent_index == 5

            b = ssbh_data_py.skel_data.BoneData("abc", numpy.ones((4,4)), None)
            assert b.name == "abc"
            assert b.transform.tolist() == [[1,1,1,1]]*4
            assert b.parent_index == None
            # Test mutability.
            b.transform[1][2] = 3
            assert b.transform[1].tolist() == [1,1,3,1]
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_world_transform_no_parent() {
        run_python_code(indoc! {r#"
            s = ssbh_data_py.skel_data.SkelData()
            b0 = ssbh_data_py.skel_data.BoneData("b0", [[0,0,0,0]]*4, None)
            b1 = ssbh_data_py.skel_data.BoneData("b1", [[1,1,1,1]]*4, None)
            s.bones = [b0, b1]

            assert s.calculate_world_transform(b1) == b1.transform
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_world_transform_with_parent() {
        run_python_code(indoc! {r#"
            s = ssbh_data_py.skel_data.SkelData()
            b0 = ssbh_data_py.skel_data.BoneData("b0", [[1,1,1,1]]*4, None)
            b1 = ssbh_data_py.skel_data.BoneData("b0", [[2,2,2,2]]*4, 0)
            s.bones = [b0, b1]

            assert s.calculate_world_transform(b1) == [[8,8,8,8]]*4
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_world_transform_with_parent_ndarray() {
        // TODO: This can also return a numpy array in the future.
        run_python_code_numpy(indoc! {r#"
            s = ssbh_data_py.skel_data.SkelData()
            b0 = ssbh_data_py.skel_data.BoneData("b0", numpy.ones((4,4)), None)
            b1 = ssbh_data_py.skel_data.BoneData("b0", numpy.ones((4,4))*2, 0)
            s.bones = [b0, b1]

            assert s.calculate_world_transform(b1) == [[8,8,8,8]]*4
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_relative_transform_with_parent() {
        run_python_code(indoc! {r#"
            world_transform = [
                [2, 0, 0, 0],
                [0, 4, 0, 0],
                [0, 0, 8, 0],
                [0, 0, 0, 1],
            ]
            parent_world_transform = [
                [1, 0, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 1, 0],
                [1, 2, 3, 1],
            ]
            relative_transform = [
                [2.0, 0, 0, 0],
                [0, 4, 0, 0],
                [0, 0, 8, 0],
                [-2, -8, -24, 1],
            ]
            assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, parent_world_transform) == relative_transform
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_relative_transform_no_parent() {
        run_python_code(indoc! {r#"
            world_transform = [
                [0, 1, 2, 3],
                [4, 5, 6, 7],
                [8, 9, 10, 11],
                [12, 13, 14, 15],
            ]
            assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None) == world_transform
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_relative_transform_no_parent_ndarray() {
        // TODO: This can also return a numpy array in the future.
        run_python_code_numpy(indoc! {r#"
            world_transform = numpy.array([
                [0, 1, 2, 3],
                [4, 5, 6, 7],
                [8, 9, 10, 11],
                [12, 13, 14, 15],
            ])
            assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None) == world_transform.tolist()
        "#})
        .unwrap();
    }

    #[test]
    fn calculate_relative_transform_no_parent_tuple() {
        // Tuples should be treated like sequences.
        run_python_code(indoc! {r#"
            world_transform = [
                (0, 1, 2, 3),
                (4, 5, 6, 7),
                (8, 9, 10, 11),
                (12, 13, 14, 15),
            ]
            expected = [
                [0, 1, 2, 3],
                [4, 5, 6, 7],
                [8, 9, 10, 11],
                [12, 13, 14, 15],
            ]
            assert ssbh_data_py.skel_data.calculate_relative_transform(world_transform, None) == expected
        "#})
        .unwrap();
    }
}

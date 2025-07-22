use crate::python_enum;
use pyo3::{create_exception, prelude::*};

create_exception!(ssbh_data_py, SkelDataError, pyo3::exceptions::PyException);

python_enum!(
    BillboardType,
    ssbh_data::skel_data::BillboardType,
    SkelDataError,
    "ssbh_data_py.skel_data",
    Disabled,
    XAxisViewPointAligned,
    YAxisViewPointAligned,
    Unk3,
    XYAxisViewPointAligned,
    YAxisViewPlaneAligned,
    XYAxisViewPlaneAligned
);

#[pymodule]
pub mod skel_data {
    pub use super::*;

    use crate::{PyRepr, Pyi, PyiMethods};
    use map_py::{MapPy, TypedList};
    use numpy::PyArray2;

    #[pymodule_export]
    pub use super::BillboardType;

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::skel_data::SkelData)]
    #[pyrepr("ssbh_data_py.skel_data")]
    #[pyi(has_methods = true)]
    pub struct SkelData {
        pub major_version: u16,
        pub minor_version: u16,
        pub bones: TypedList<BoneData>,
    }

    #[pyclass(get_all, set_all)]
    #[derive(Debug, Clone, MapPy, Pyi, PyRepr)]
    #[map(ssbh_data::skel_data::BoneData)]
    #[pyrepr("ssbh_data_py.skel_data")]
    pub struct BoneData {
        pub name: String,

        pub transform: Py<PyArray2<f32>>,

        #[pyi(default = "None")]
        pub parent_index: Option<usize>,

        #[pyi(default = "BillboardType.Disabled")]
        pub billboard_type: BillboardType,
    }

    // TODO: Derive this?
    #[pymethods]
    impl BoneData {
        #[new]
        #[pyo3(signature = (name, transform, parent_index=None::<usize>, billboard_type=None))]
        fn new(
            _py: Python,
            name: String,
            transform: Py<PyArray2<f32>>,
            parent_index: Option<Option<usize>>,
            billboard_type: Option<BillboardType>,
        ) -> PyResult<Self> {
            Ok(Self {
                name,
                transform,
                parent_index: parent_index.unwrap_or(None),
                billboard_type: billboard_type
                    .unwrap_or(ssbh_data::skel_data::BillboardType::Disabled.into()),
            })
        }
        fn __repr__(&self) -> String {
            self.py_repr()
        }
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
        self, bone: BoneData) -> numpy.ndarray: ..."#
                .to_string()
        }
    }

    #[pymethods]
    impl SkelData {
        #[new]
        #[pyo3(signature = (major_version = 1, minor_version = 0))]
        fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
            Ok(SkelData {
                major_version,
                minor_version,
                bones: TypedList::empty(py),
            })
        }

        fn save(&self, py: Python, path: &str) -> PyResult<()> {
            self.clone()
                .map_py(py)?
                .write_to_file(path)
                .map_err(|e| SkelDataError::new_err(format!("{e}")))
        }

        fn calculate_world_transform(
            &self,
            py: Python,
            bone: BoneData,
        ) -> PyResult<Py<PyArray2<f32>>> {
            let data: ssbh_data::skel_data::SkelData = self.clone().map_py(py)?;
            let bone_data: ssbh_data::skel_data::BoneData = bone.map_py(py)?;
            let transform = data
                .calculate_world_transform(&bone_data)
                .map_err(|e| SkelDataError::new_err(format!("{e}")))?;
            transform.map_py(py)
        }
    }

    #[pyfunction]
    fn read_skel(py: Python, path: &str) -> PyResult<SkelData> {
        ssbh_data::skel_data::SkelData::from_file(path)
            .map_err(|e| SkelDataError::new_err(format!("{e}")))?
            .map_py(py)
    }

    #[pyfunction]
    fn calculate_relative_transform(
        py: Python,
        world_transform: Py<PyArray2<f32>>,
        parent_world_transform: Option<Py<PyArray2<f32>>>,
    ) -> PyResult<Py<PyArray2<f32>>> {
        let world_transform = world_transform.map_py(py)?;
        let transform = match parent_world_transform {
            Some(m) => ssbh_data::skel_data::calculate_relative_transform(
                &world_transform,
                Some(&m.map_py(py)?),
            ),
            None => ssbh_data::skel_data::calculate_relative_transform(&world_transform, None),
        };
        transform.map_py(py)
    }
}

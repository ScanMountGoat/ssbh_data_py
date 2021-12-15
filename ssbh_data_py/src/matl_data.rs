use crate::{python_enum, MapPy, PyTypeString, PyiMethods};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::{prelude::*, types::PyList};
use ssbh_data::SsbhData;
use ssbh_data_py_derive::{MapPy, Pyi};

mod enums;

create_exception!(ssbh_data_py, MatlDataError, pyo3::exceptions::PyException);

// TODO: No constructor for SamplerData, BlendStateData, RasterizerStateData?
pub fn matl_data(py: Python, module: &PyModule) -> PyResult<()> {
    let matl_data = PyModule::new(py, "matl_data")?;
    // TODO: Automatically register classes?
    matl_data.add_class::<MatlData>()?;
    matl_data.add_class::<MatlEntryData>()?;

    matl_data.add_class::<ParamId>()?;
    matl_data.add_class::<BlendFactor>()?;
    matl_data.add_class::<FillMode>()?;
    matl_data.add_class::<CullMode>()?;
    matl_data.add_class::<WrapMode>()?;
    matl_data.add_class::<MinFilter>()?;
    matl_data.add_class::<MagFilter>()?;
    matl_data.add_class::<MaxAnisotropy>()?;

    matl_data.add_class::<BlendStateParam>()?;
    matl_data.add_class::<FloatParam>()?;
    matl_data.add_class::<BooleanParam>()?;
    matl_data.add_class::<Vector4Param>()?;
    matl_data.add_class::<RasterizerStateParam>()?;
    matl_data.add_class::<SamplerParam>()?;
    matl_data.add_class::<TextureParam>()?;

    matl_data.add_class::<RasterizerStateData>()?;
    matl_data.add_class::<SamplerData>()?;
    matl_data.add_class::<BlendStateData>()?;

    matl_data.add_function(wrap_pyfunction!(read_matl, matl_data)?)?;

    module.add_submodule(matl_data)?;
    Ok(())
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::MatlData)]
#[pyi(has_methods = true)]
pub struct MatlData {
    #[pyo3(get, set)]
    pub major_version: u16,

    #[pyo3(get, set)]
    pub minor_version: u16,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[MatlEntryData]")]
    pub entries: Py<PyList>,
}

#[pymethods]
impl MatlData {
    #[new]
    #[args(major_version = 1, minor_version = 6)]
    fn new(py: Python, major_version: u16, minor_version: u16) -> PyResult<Self> {
        Ok(MatlData {
            major_version,
            minor_version,
            entries: PyList::empty(py).into(),
        })
    }

    fn save(&self, py: Python, path: &str) -> PyResult<()> {
        self.map_py(py)?
            .write_to_file(path)
            .map_err(|e| MatlDataError::new_err(format!("{}", e)))
    }
}

impl PyiMethods for MatlData {
    fn pyi_methods() -> String {
        r#"
    def __init__(
            self,
            major_version: int = ...,
            minor_version: int = ...,
        ) -> None: ...
    
    def save(self, path: str) -> None: ..."#
            .to_string()
    }
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::MatlEntryData)]
pub struct MatlEntryData {
    #[pyo3(get, set)]
    pub material_label: String,

    #[pyo3(get, set)]
    pub shader_label: String,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[BlendStateParam]")]
    pub blend_states: Py<PyList>,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[FloatParam]")]
    pub floats: Py<PyList>,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[BooleanParam]")]
    pub booleans: Py<PyList>,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[Vector4Param]")]
    pub vectors: Py<PyList>,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[RasterizerStateParam]")]
    pub rasterizer_states: Py<PyList>,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[SamplerParam]")]
    pub samplers: Py<PyList>,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[TextureParam]")]
    pub textures: Py<PyList>,
}

#[pymethods]
impl MatlEntryData {
    #[new]
    fn new(py: Python, material_label: String, shader_label: String) -> PyResult<Self> {
        Ok(MatlEntryData {
            material_label,
            shader_label,
            blend_states: PyList::empty(py).into(),
            floats: PyList::empty(py).into(),
            booleans: PyList::empty(py).into(),
            vectors: PyList::empty(py).into(),
            rasterizer_states: PyList::empty(py).into(),
            samplers: PyList::empty(py).into(),
            textures: PyList::empty(py).into(),
        })
    }
}

macro_rules! param_new_impl {
    ($(($py_class:ty,$data:ty)),*) => {
        $(
            #[pymethods]
            impl $py_class {
                #[new]
                fn new(_py: Python, param_id: ParamId, data: $data) -> PyResult<Self> {
                    Ok(Self { param_id, data })
                }
            }

            // TODO: Find a better place to generate the methods.
            impl crate::PyiMethods for $py_class {
                fn pyi_methods() -> String {
                    format!(r#"
    def __init__(
            self,
            param_id: ParamId = ...,
            data: {} = ...,
        ) -> None: ..."#, <$data>::py_type_string())
                }
            }
        )*
    };
}

param_new_impl!(
    (BlendStateParam, BlendStateData),
    (FloatParam, f32),
    (BooleanParam, bool),
    (Vector4Param, PyObject),
    (RasterizerStateParam, RasterizerStateData),
    (SamplerParam, SamplerData),
    (TextureParam, String)
);

// TODO: Is there a workaround for MapPy not supporting generic structs?
// Have type aliases for each variant in Rust and separate ParamData types in Python?
#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::BlendStateParam)]
#[pyi(has_methods = true)]
pub struct BlendStateParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: BlendStateData,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::FloatParam)]
#[pyi(has_methods = true)]
pub struct FloatParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: f32,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::BooleanParam)]
#[pyi(has_methods = true)]
pub struct BooleanParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: bool,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::Vector4Param)]
#[pyi(has_methods = true)]
pub struct Vector4Param {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    #[pyi(python_type = "list[float]")]
    pub data: PyObject,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::RasterizerStateParam)]
#[pyi(has_methods = true)]
pub struct RasterizerStateParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: RasterizerStateData,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::SamplerParam)]
#[pyi(has_methods = true)]
pub struct SamplerParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: SamplerData,
}

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::TextureParam)]
#[pyi(has_methods = true)]
pub struct TextureParam {
    #[pyo3(get, set)]
    pub param_id: ParamId,

    #[pyo3(get, set)]
    pub data: String,
}

python_enum!(
    ParamId,
    ssbh_data::matl_data::ParamId,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::BlendStateData)]
pub struct BlendStateData {
    pub source_color: BlendFactor,
    pub destination_color: BlendFactor,
    pub alpha_sample_to_coverage: bool,
}

python_enum!(
    BlendFactor,
    ssbh_data::matl_data::BlendFactor,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::RasterizerStateData)]
pub struct RasterizerStateData {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub depth_bias: f32,
}

python_enum!(
    FillMode,
    ssbh_data::matl_data::FillMode,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

// TODO: "None" doesn't work as a variant?
python_enum!(
    CullMode,
    ssbh_data::matl_data::CullMode,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

#[pyclass(module = "ssbh_data_py.matl_data")]
#[derive(Debug, Clone, MapPy, Pyi)]
#[map(ssbh_data::matl_data::SamplerData)]
pub struct SamplerData {
    pub wraps: WrapMode,
    pub wrapt: WrapMode,
    pub wrapr: WrapMode,
    pub min_filter: MinFilter,
    pub mag_filter: MagFilter,
    #[pyi(python_type = "list[float]")]
    pub border_color: PyObject,
    pub lod_bias: f32,
    pub max_anisotropy: Option<MaxAnisotropy>,
}

impl MapPy<ssbh_data::matl_data::Color4f> for PyObject {
    fn map_py(&self, py: Python) -> PyResult<ssbh_data::matl_data::Color4f> {
        let [r, g, b, a] = self.extract::<[f32; 4]>(py)?;
        Ok(ssbh_data::matl_data::Color4f { r, g, b, a })
    }
}

impl MapPy<PyObject> for ssbh_data::matl_data::Color4f {
    fn map_py(&self, py: Python) -> PyResult<PyObject> {
        Ok(PyList::new(py, [self.r, self.g, self.b, self.a]).into())
    }
}

python_enum!(
    WrapMode,
    ssbh_data::matl_data::WrapMode,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

python_enum!(
    MinFilter,
    ssbh_data::matl_data::MinFilter,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

python_enum!(
    MagFilter,
    ssbh_data::matl_data::MagFilter,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

python_enum!(
    MaxAnisotropy,
    ssbh_data::matl_data::MaxAnisotropy,
    MatlDataError,
    "ssbh_data_py.matl_data"
);

#[pyfunction]
fn read_matl(py: Python, path: &str) -> PyResult<MatlData> {
    ssbh_data::matl_data::MatlData::from_file(path)
        .map_err(|e| MatlDataError::new_err(format!("{}", e)))?
        .map_py(py)
}

#[cfg(test)]
mod tests {
    use crate::run_python_code;
    use indoc::indoc;

    #[test]
    fn create_matl() {
        run_python_code(indoc! {r#"
            m = ssbh_data_py.matl_data.MatlData(3, 4)
            assert m.major_version == 3
            assert m.minor_version == 4
            assert m.entries == []

            m = ssbh_data_py.matl_data.MatlData(1)
            assert m.major_version == 1
            assert m.minor_version == 6

            m = ssbh_data_py.matl_data.MatlData()
            assert m.major_version == 1
            assert m.minor_version == 6
        "#})
        .unwrap();
    }

    #[test]
    fn create_matl_entry() {
        run_python_code(indoc! {r#"
            m = ssbh_data_py.matl_data.MatlEntryData("a", "b")
            assert m.material_label == "a"
            assert m.shader_label == "b"
            assert m.blend_states == []
            assert m.floats == []
            assert m.booleans == []
            assert m.vectors == []
            assert m.rasterizer_states == []
            assert m.samplers == []
            assert m.textures == []
        "#})
        .unwrap();
    }
}

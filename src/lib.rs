use pyo3::wrap_pyfunction;
use pyo3::{prelude::*, PyObjectProtocol};
use ssbh_data::mesh_data::{
    read_colorsets, read_normals, read_positions, read_texture_coordinates, read_vertex_indices,
};
use ssbh_lib::SsbhFile;

macro_rules! ssbh_pyproto_impl {
    ($($ty:ident),*) => {
        $(
            #[pyproto]
            impl<'a> PyObjectProtocol<'a> for $ty {
                fn __str__(&self) -> String {
                    format!("{:?}", self.data)
                }
            
                fn __repr__(&self) -> String {
                    self.__str__()
                }
            }
        )*
    };
}


#[pyclass]
struct Mesh {
    data: ssbh_lib::formats::mesh::Mesh,
}

#[pyclass]
struct Skel {
    data: ssbh_lib::formats::skel::Skel,
}

#[pyclass]
struct Matl {
    data: ssbh_lib::formats::matl::Matl,
}

ssbh_pyproto_impl!(Mesh, Skel, Matl);

#[pyclass]
struct MeshData {
    #[pyo3(get)]
    pub name: String,

    #[pyo3(get)]
    pub positions: Vec<[f32; 3]>,

    #[pyo3(get)]
    pub normals: Vec<[f32; 3]>,

    #[pyo3(get)]
    pub texcoords: Vec<Vec<[f32; 2]>>,

    #[pyo3(get)]
    pub colorsets: Vec<Vec<[f32; 4]>>,

    #[pyo3(get)]
    pub indices: Vec<(u32, u32, u32)>,

    #[pyo3(get)]
    pub transform: [(f32, f32, f32, f32); 4],
}

#[pyfunction]
fn read_mesh(path: &str) -> PyResult<Mesh> {
    // TODO: How to handle errors or return None?
    match ssbh_lib::read_ssbh(path).unwrap().data {
        SsbhFile::Mesh(data) => Ok(Mesh { data }),
        _ => panic!("Failed to read mesh."),
    }
}

#[pyfunction]
fn read_skel(path: &str) -> PyResult<Skel> {
    // TODO: How to handle errors or return None?
    match ssbh_lib::read_ssbh(path).unwrap().data {
        SsbhFile::Skel(data) => Ok(Skel { data }),
        _ => panic!("Failed to read mesh."),
    }
}

#[pyfunction]
fn read_meshes(mesh: &Mesh, skel: &Skel) -> PyResult<Vec<MeshData>> {
    let mesh = &mesh.data;
    let skel = &skel.data;

    let mut meshes = Vec::new();

    for mesh_object in &mesh.objects.elements {
        // TODO: Avoid unwrap?
        let indices = read_vertex_indices(&mesh, &mesh_object).unwrap();
        let positions = read_positions(&mesh, &mesh_object).unwrap();
        let normals = read_normals(&mesh, &mesh_object).unwrap();
        let texcoords = read_texture_coordinates(&mesh, &mesh_object, true).unwrap();
        let colorsets = read_colorsets(&mesh, &mesh_object, true).unwrap();

        // TODO: This isn't safe and may panic.
        let indices = indices[..].chunks(3).map(|s| (s[0], s[1], s[2])).collect();

        // Use an identity matrix if not found or the skel failed to load.
        let transform = match ssbh_data::skel_data::get_single_bind_transform(&skel, &mesh_object) {
            Some(matrix) => matrix,
            None => [
                (1f32, 0f32, 0f32, 0f32),
                (0f32, 1f32, 0f32, 0f32),
                (0f32, 0f32, 1f32, 0f32),
                (0f32, 0f32, 0f32, 1f32),
            ],
        };

        meshes.push(MeshData {
            name: mesh_object.name.get_string().unwrap().into(),
            positions,
            normals,
            texcoords,
            colorsets,
            indices,
            transform,
        });
    }

    Ok(meshes)
}

#[pymodule]
fn ssbh_data_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MeshData>()?;
    m.add_function(wrap_pyfunction!(read_meshes, m)?)?;
    m.add_function(wrap_pyfunction!(read_mesh, m)?)?;
    m.add_function(wrap_pyfunction!(read_skel, m)?)?;

    Ok(())
}

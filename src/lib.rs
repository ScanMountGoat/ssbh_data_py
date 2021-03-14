use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use ssbh_data::mesh_data::{
    read_colorsets, read_normals, read_positions, read_texture_coordinates, read_vertex_indices,
};
use ssbh_lib::SsbhFile;

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
fn read_meshes(mesh_path: &str, skel_path: &str) -> PyResult<Vec<MeshData>> {
    // TODO: don't unwrap and convert to python error or return none.
    let mesh = match ssbh_lib::read_ssbh(mesh_path).unwrap().data {
        SsbhFile::Mesh(mesh) => Some(mesh),
        _ => None,
    }
    .unwrap();

    // TODO: Provide a convenience method to get a particular ssbh_type?
    let skel = match ssbh_lib::read_ssbh(skel_path) {
        Ok(ssbh) => match ssbh.data {
            SsbhFile::Skel(skel) => Some(skel),
            _ => None,
        },
        _ => None,
    };

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
        let transform = match &skel {
            Some(skel) => {
                match ssbh_data::skel_data::get_single_bind_transform(&skel, &mesh_object) {
                    Some(matrix) => matrix,
                    None => [
                        (1f32, 0f32, 0f32, 0f32),
                        (0f32, 1f32, 0f32, 0f32),
                        (0f32, 0f32, 1f32, 0f32),
                        (0f32, 0f32, 0f32, 1f32),
                    ],
                }
            }
            _ => [
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

/// A Python module implemented in Rust.
#[pymodule]
fn ssbh_data_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<MeshData>()?;
    m.add_function(wrap_pyfunction!(read_meshes, m)?)?;
    Ok(())
}

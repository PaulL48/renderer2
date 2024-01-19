// A mesh is a collection of vertices
// A vertex is a collection of attributes, usually at minimum containing a position
// but optionally containing UV coordinates, normals, color, etc...

// We want to guarantee uniqueness for vertex buffers. We really don't want to
// create duplicate buffers for the same vertex data

// We also don't want to create and destroy the vertex buffer every frame
//

use std::slice::Iter;

use crate::sub_mesh::{SubMesh, SubMeshSource};

pub struct Mesh {
    name: String,
    sub_meshes: Vec<SubMesh>,
}

pub trait MeshSource {
    fn sub_meshes(&self) -> Iter<&dyn SubMeshSource>;
}

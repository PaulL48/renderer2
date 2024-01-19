use crate::{mesh::MeshSource, sub_mesh::SubMesh};
use slot_map::{SlotMap, SlotMapIndex};
use wgpu::Device;

pub type MeshHandle = SlotMapIndex;

pub struct MeshCache {
    meshes: SlotMap<Vec<SubMesh>>,
}

impl MeshCache {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            meshes: SlotMap::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, device: &Device, mesh_source: &dyn MeshSource) -> MeshHandle {
        let mut sub_meshes = Vec::new();
        for sub_mesh in mesh_source.sub_meshes() {
            let sub_mesh = SubMesh::from_source(device, *sub_mesh);
            sub_meshes.push(sub_mesh);
        }
        self.meshes.push(sub_meshes)
    }
}

use std::collections::HashMap;

use crate::{mesh::{MeshSource, Mesh}, sub_mesh::SubMesh, pipeline_configuration::PipelineConfiguration};
use slot_map::{SlotMap, SlotMapIndex};
use wgpu::Device;

pub type MeshHandle = SlotMapIndex;

pub struct MeshCache {
    meshes: SlotMap<Mesh>,
}

impl MeshCache {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            meshes: SlotMap::with_capacity(capacity),
        }
    }

    pub fn insert(
        &mut self, 
        device: &Device, 
        pipeline_lookup: &HashMap<PipelineConfiguration, SlotMapIndex>, 
        mesh_source: &dyn MeshSource
    ) -> MeshHandle {
        let mesh = Mesh::from_source(device, pipeline_lookup, mesh_source);
        self.meshes.push(mesh)
    }

    // pub fn insert(&mut self, device: &Device, mesh_source: &dyn MeshSource) -> MeshHandle {
    //     // let mut sub_meshes = Vec::new();
    //     // for sub_mesh in mesh_source.sub_meshes() {
    //     //     let sub_mesh = SubMesh::from_source(device, *sub_mesh);
    //     //     sub_meshes.push(sub_mesh);
    //     // }
    //     // self.meshes.push(sub_meshes)
    // }

    pub fn meshes_mut(&mut self) -> &mut SlotMap<Mesh> {
        &mut self.meshes
    }
}

use crate::material_cache::MaterialCache;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, IndexFormat, RenderPass,
};

pub struct SubMeshSource {
    pub vertices: Vec<u8>,
    pub indices: Vec<u8>,
    pub index_count: u32,
    pub index_type: IndexFormat, // Creates a codependency on wgpu
    pub material_id: u64,
}

pub struct SubMesh {
    vertices: Buffer,
    indices: Buffer,
    index_count: u32,
    index_type: IndexFormat,
    material_id: u64,
}

impl SubMesh {
    pub fn from_source(device: &Device, source: &SubMeshSource) -> SubMesh {
        let vertices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &source.vertices,
            usage: BufferUsages::VERTEX,
        });

        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &source.indices,
            usage: BufferUsages::INDEX,
        });

        Self {
            vertices,
            indices,
            index_count: source.index_count,
            index_type: source.index_type,
            material_id: source.material_id,
        }
    }

    pub fn record_commands<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        first_available_bind_group: u32,
        material_cache: &'a MaterialCache,
    ) {
        let material = material_cache.get(&self.material_id).unwrap();
        render_pass.set_bind_group(first_available_bind_group, material.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertices.slice(..));
        render_pass.set_index_buffer(self.indices.slice(..), self.index_type);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

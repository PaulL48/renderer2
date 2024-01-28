use crate::{
    material_cache::MaterialCache,
    pipeline_configuration::PipelineConfiguration,
    sub_mesh::{SubMesh, SubMeshSource},
    uniform_group::UniformGroup,
    UniformGroupSource,
};
use slot_map::SlotMapIndex;
use std::collections::HashMap;
use wgpu::{Device, RenderPass};

pub type MeshHandle = SlotMapIndex;

pub struct Mesh {
    name: String,
    sub_meshes: Vec<SubMesh>,
    mesh_uniform_group: UniformGroup,
    pipeline: SlotMapIndex,
}

pub struct MeshSource {
    pub name: String,
    pub sub_meshes: Vec<SubMeshSource>,
    pub mesh_uniform_group: UniformGroupSource,
    pub pipeline_configuration: PipelineConfiguration,
}

impl Mesh {
    pub fn from_source(
        device: &Device,
        pipeline_lookup: &HashMap<PipelineConfiguration, SlotMapIndex>,
        source: &MeshSource,
    ) -> Self {
        let mut sub_meshes = Vec::new();
        for sub_mesh in &source.sub_meshes {
            let sub_mesh = SubMesh::from_source(device, sub_mesh);
            sub_meshes.push(sub_mesh);
        }

        Self {
            name: source.name.clone(),
            sub_meshes,
            mesh_uniform_group: UniformGroup::from_source(&source.mesh_uniform_group, device),
            pipeline: *pipeline_lookup.get(&source.pipeline_configuration).unwrap(),
        }
    }

    pub fn pipeline(&self) -> &SlotMapIndex {
        &self.pipeline
    }

    pub fn record_commands<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        mut first_available_bind_group: u32,
        material_cache: &'a MaterialCache,
    ) {
        render_pass.set_bind_group(
            first_available_bind_group,
            self.mesh_uniform_group.bind_group(),
            &[],
        );
        first_available_bind_group += 1;
        for sub_mesh in &self.sub_meshes {
            sub_mesh.record_commands(render_pass, first_available_bind_group, material_cache);
        }
    }
}

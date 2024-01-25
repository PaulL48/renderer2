// A mesh is a collection of vertices
// A vertex is a collection of attributes, usually at minimum containing a position
// but optionally containing UV coordinates, normals, color, etc...

// We want to guarantee uniqueness for vertex buffers. We really don't want to
// create duplicate buffers for the same vertex data

// We also don't want to create and destroy the vertex buffer every frame
//

use std::{collections::HashMap, slice::Iter};

use slot_map::SlotMapIndex;
use wgpu::{Device, RenderPass};

use crate::{
    material::Material,
    pipeline_configuration::PipelineConfiguration,
    sub_mesh::{SubMesh, SubMeshSource},
};

pub type MeshHandle = SlotMapIndex;

pub struct Mesh {
    name: String,
    sub_meshes: Vec<SubMesh>,
    pipeline: SlotMapIndex,
}

pub trait MeshSource {
    fn name(&self) -> String;
    fn sub_meshes(&self) -> Iter<&dyn SubMeshSource>;
    fn pipeline_configuration(&self) -> PipelineConfiguration;
}

impl Mesh {
    pub fn from_source(
        device: &Device,
        pipeline_lookup: &HashMap<PipelineConfiguration, SlotMapIndex>,
        source: &dyn MeshSource,
    ) -> Self {
        let mut sub_meshes = Vec::new();
        for sub_mesh in source.sub_meshes() {
            let sub_mesh = SubMesh::from_source(device, *sub_mesh);
            sub_meshes.push(sub_mesh);
        }

        Self {
            name: source.name(),
            sub_meshes,
            pipeline: *pipeline_lookup
                .get(&source.pipeline_configuration())
                .unwrap(),
        }
    }

    pub fn pipeline(&self) -> &SlotMapIndex {
        &self.pipeline
    }

    pub fn record_commands<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        first_available_bind_group: u32,
        material_cache: &'a HashMap<&'static str, Material>,
    ) {
        for sub_mesh in &self.sub_meshes {
            sub_mesh.record_commands(render_pass, first_available_bind_group, material_cache);
        }
    }
}

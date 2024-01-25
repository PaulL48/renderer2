// Create a renderer-side list of submeshes that be reused in the render command generation.

// A mesh can be a resource associated with an ID generated from the mesh name?
// Submeshes don't have names

// Based on the gamedev overflow answer:
// Create the vertex buffers as early as possible and release them as late as possible

// I think the real challenge is figuring out the API to expose

// At the end of a day a mesh is a currency the user can manage (we don't want to expose submeshes)
// And submeshes are implicitly tied with a material
// So the the user *should* only have to specify: The mesh that is desired to be rendered
// This contains the submesh-material pairs
// The mesh is also tied to a pipeline

// renderer.submit(mesh)
// But what is 'mesh' in this case?
// a name? then how does the data actually make it to the renderer in the first place?
// All the data? and then the renderer performs the necessary operations to create the buffers
//     or check if this data already exists.
//     this seems like it would be moving a lot of data from user program to renderer.
//     So instead we can create a constraint that all meshes to be drawn need to be
//     pre-submitted to the renderer.
//     From there we can have a handle to a renderer-side resource (the vertex buffers, texture buffers,
//     etc...) that we then supply to the submit function

// just to distinguish the language:
// submit is used to describe something that corresponds to a command buffer command
// so what is used to mean to move data into the renderer: cache,
// impl Renderer {
//     fn cache_mesh(mesh: Mesh) -> MeshHandle;
//     fn flush_mesh(mesh: MeshHandle);
//     fn submit_mesh(mesh: MeshHandle);
// }

// Then is the decision as to whether to require the same caching of material data
// prior to submit calls

// So what then is a material? it is a set of textures and samplers that are manipulated in
// the shader program using vertex and lighting data to generate fragment colors

// A material is similar to a vertex in that it is of a constant layout for a shader program
// but may have variants in other shader programs, so it could be more of a contract
// that a user data type can fulfil

use std::collections::HashMap;

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device, IndexFormat, RenderPass,
};

use crate::material::Material;

pub trait SubMeshSource {
    // Return the vertex data as it will be present in the GPU buffer
    fn vertices(&self) -> &[u8];

    // Return the index data as it will be present in the GPU buffer
    fn indices(&self) -> &[u8];

    // Return the number of indices for this sub mesh
    fn index_count(&self) -> u32;

    // Return the width of the indices
    fn index_type(&self) -> IndexFormat;

    // Return the material identifier
    fn material(&self) -> &'static str;
}

pub struct SubMesh {
    vertices: Buffer,
    indices: Buffer,
    index_count: u32,
    index_type: IndexFormat,
    material: &'static str,
}

impl SubMesh {
    pub fn from_source(device: &Device, source: &dyn SubMeshSource) -> SubMesh {
        let vertices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: source.vertices(),
            usage: BufferUsages::VERTEX,
        });

        let indices = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: source.indices(),
            usage: BufferUsages::INDEX,
        });

        Self {
            vertices,
            indices,
            index_count: source.index_count(),
            index_type: source.index_type(),
            material: source.material(),
        }
    }

    pub fn record_commands<'a>(
        &'a self,
        render_pass: &mut RenderPass<'a>,
        first_available_bind_group: u32,
        material_cache: &'a HashMap<&'static str, Material>,
    ) {
        let material = material_cache.get(self.material).unwrap();
        render_pass.set_bind_group(first_available_bind_group, material.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, self.vertices.slice(..));
        render_pass.set_index_buffer(self.indices.slice(..), self.index_type);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}

// At some point we need to provide a way to convert some CPU side data into the buffers
// This relates to the idea of vertices as a contract

// The pieces we need to extract are listed as the members of the SubMesh struct
// We need to get the vertices as a raw buffer, the indices the same, the number or indices
// the type of the indices and the name of the material

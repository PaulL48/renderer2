// A material is an abstract data type that specifies a bind group
// and a way to transmit the data into the GPU

// One point of heterogeneity is that the cardinality of the Material
// is expected to vary. For some vertex colored mesh the material
// might be nothing, a full PBR material might be eight textures
// a PBR without parallax mapping might be seven

// Unlike vertices where all the vertices are stored in the same
// array, a material must store its textures in different members

// we could impose a restriction that a material is laid out
// using an iterator within a bind group with a size equal to
// the size of the iterator

// Then we know when binding the textures and samplers that
// they are bound in a numerical sequence

// The thing that gets bound is a single BindGroup though

// Much like a submesh stores a single buffer for the vertices
// we end up with a single BindGroup for all the textures of a material

// The difference with this is that it is intuitive to
// express the vertex and index buffers as binary blobs

// The user has images. So let them deal in images

use std::slice::Iter;

use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Device, Queue, SamplerBindingType,
    ShaderStages, TextureSampleType, TextureViewDimension,
};

use crate::texture::{BinaryTexture, Texture};

pub trait MaterialSource {
    fn name(&self) -> &'static str;
    fn texture_sources(&self) -> Iter<BinaryTexture>;
}

pub struct Material {
    name: &'static str,
    textures: Vec<Texture>,
    bind_group: BindGroup,
}

impl Material {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn from_source(source: &dyn MaterialSource, device: &Device, queue: &Queue) -> Self {
        let mut textures = Vec::new();
        let mut bind_group_layout_entries = Vec::new();
        let mut bind_group_entries = Vec::new();
        let mut binding_index = 0;
        for binary_texture in source.texture_sources() {
            let texture = Texture::from_binary(device, queue, source.name(), binary_texture);

            textures.push(texture);

            let texture_layout_entry = BindGroupLayoutEntry {
                binding: binding_index,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            };

            binding_index += 1;

            let sampler_layout_entry = BindGroupLayoutEntry {
                binding: binding_index,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Sampler(SamplerBindingType::Filtering),
                count: None,
            };

            binding_index += 1;

            bind_group_layout_entries.push(texture_layout_entry);
            bind_group_layout_entries.push(sampler_layout_entry);
        }

        // Separate loop needed to create BindGroupEntry vec to extend lifetime
        // of reference into textures past creation of bind group layout
        binding_index = 0;
        for texture in &textures {
            let texture_entry = BindGroupEntry {
                binding: binding_index,
                resource: BindingResource::TextureView(texture.view()),
            };

            binding_index += 1;

            let sampler_entry = BindGroupEntry {
                binding: binding_index,
                resource: BindingResource::Sampler(texture.sampler()),
            };

            binding_index += 1;

            bind_group_entries.push(texture_entry);
            bind_group_entries.push(sampler_entry);
        }

        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some(source.name()),
            entries: &bind_group_layout_entries,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some(source.name()),
            layout: &layout,
            entries: &bind_group_entries,
        });

        Self {
            name: source.name(),
            textures,
            bind_group,
        }
    }
}

use crate::texture::{Texture, TextureSource};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Device, Queue, SamplerBindingType,
    ShaderStages, TextureSampleType, TextureViewDimension,
};

pub struct MaterialSource {
    pub id: u64,
    pub texture_sources: Vec<TextureSource>,
}

pub struct Material {
    id: u64,
    _textures: Vec<Texture>,
    bind_group: BindGroup,
}

impl Material {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn from_source(source: &MaterialSource, device: &Device, queue: &Queue) -> Self {
        let mut textures = Vec::new();
        let mut bind_group_layout_entries = Vec::new();
        let mut bind_group_entries = Vec::new();
        let mut binding_index = 0;
        for binary_texture in &source.texture_sources {
            let texture = Texture::from_source(device, queue, binary_texture);

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
            label: None,
            entries: &bind_group_layout_entries,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &bind_group_entries,
        });

        Self {
            id: source.id,
            _textures: textures,
            bind_group,
        }
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}

use crate::uniform::{Uniform, UniformSource};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBinding, BufferBindingType, Device,
    ShaderStages,
};

pub struct UniformGroupSource {
    pub name: String,
    pub uniform_sources: Vec<UniformSource>,
}

pub struct UniformGroup {
    name: String,
    uniforms: Vec<Uniform>,
    bind_group: BindGroup,
}

impl UniformGroup {
    pub fn from_source(source: &UniformGroupSource, device: &Device) -> Self {
        let mut uniforms = Vec::new();
        let mut layout_entries = Vec::new();
        let mut binding_index = 0;
        for uniform_source in &source.uniform_sources {
            let uniform = Uniform::from_source(uniform_source, device);

            let uniform_layout_entry = BindGroupLayoutEntry {
                binding: binding_index,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            };

            binding_index += 1;
            uniforms.push(uniform);
            layout_entries.push(uniform_layout_entry);
        }

        binding_index = 0;
        let mut entries = Vec::new();
        for uniform in &uniforms {
            let uniform_entry = BindGroupEntry {
                binding: binding_index,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: uniform.buffer(),
                    offset: 0,
                    size: None,
                }),
            };
            entries.push(uniform_entry);
            binding_index += 1;
        }

        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some(&source.name),
            entries: &layout_entries,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some(&source.name),
            layout: &layout,
            entries: &entries,
        });

        Self {
            name: source.name.clone(),
            uniforms,
            bind_group,
        }
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}

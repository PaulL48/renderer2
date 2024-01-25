// So what is the problem?

// The user needs a way to register and update data sent to the shader programs

// What are the existing structures involved in this problem?

// The Shaders accept data via bind groups which contain bind group entries

use std::slice::Iter;

use wgpu::{BindGroup, BindGroupLayoutEntry, ShaderStages, BindingType, BufferBindingType, BindingResource, Buffer, BufferBinding, BindGroupEntry, Device, Queue, util::{DeviceExt, BufferInitDescriptor}, BufferUsages};

pub trait BindGroupSource {
    fn data_sources(&self) -> Iter<&[u8]>;
}

pub fn bind_group_from_source(source: &dyn BindGroupSource, device: &Device, queue: &Queue) -> BindGroup {

    // BindGroupLayoutEntry requires details regarding the structure of the data

    // BindGroupEntry requires a BindingResource

    // create_bind_group_layout requires BindGroupLayoutDescriptor which requires:
    //      Vec<BindGroupLayoutEntry>

    // create_bind_group requires BindGroupDescriptor which requires
    //      BindGroupLayout
    //      Vec<BindGroupEntry>


    let mut bind_group_layout_entries = Vec::new();
    let mut bind_group_entries = Vec::new();
    let mut binding_index = 0;

    for data_source in source.data_sources() {
        let buffer_layout_entry = BindGroupLayoutEntry {
            binding: binding_index,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer { 
                ty: BufferBindingType::Uniform, 
                has_dynamic_offset: false, 
                min_binding_size: None, 
            },
            count: None,
        };

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: data_source,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let buffer_entry = BindGroupEntry {
            binding: binding_index,
            resource: BindingResource::Buffer(BufferBinding { 
                buffer: todo!(), 
                offset: todo!(), 
                size: todo!() 
            }),
        }

        binding_index += 1;

        bind_group_layout_entries.push(buffer_layout_entry);
    }



    todo!()
}

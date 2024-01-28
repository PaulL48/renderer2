use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Buffer, BufferUsages, Device,
};

pub struct UniformSource {
    pub data: Vec<u8>,
}

pub struct Uniform {
    staging_buffer: Buffer,
    buffer: Buffer,
}

impl Uniform {
    pub fn from_source(binary: &UniformSource, device: &Device) -> Self {
        let staging_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &binary.data,
            usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC,
        });

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &binary.data,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        });

        Self {
            staging_buffer,
            buffer,
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }
}

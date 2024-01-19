
// This is testing struct to fuse the lifetime of an encoder and a render pass

use wgpu::{CommandEncoder, RenderPass, Device, RenderPassDescriptor, CommandEncoderDescriptor};

struct RenderPassEncoder<'a> {
    encoder: CommandEncoder,
    render_pass: RenderPass<'a>,
}

impl<'a> RenderPassEncoder<'a> {
    pub fn new(device: &Device, name: &str, descriptor: RenderPassDescriptor<'a, '_>) -> Self {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { 
            label: Some(name),
        });

        Self {
            encoder,
            render_pass: encoder.begin_render_pass(&descriptor)
        }
    }
}

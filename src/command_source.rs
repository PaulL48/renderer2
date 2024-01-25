// An initial idea for how to allow things to generate commands

use wgpu::{Queue, RenderPass};

trait RenderPassCommandSource {
    fn data_transfer_commands(&self, queue: &Queue);
    fn record_render_commands(&self, render_pass: &RenderPass);
}

// So now we get to the part where we need to keep data on the GPU up to date
// with data on the CPU

// This is data usually relating to the world space transform of a model or the camera

// The data on the GPU side is managed in a buffer that needs to encode translate, rotate and scale
// for each model

// We can write to these buffers via the queue or via a render pass

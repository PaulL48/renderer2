use derive_builder::Builder;
use wgpu::InstanceFlags;

#[derive(Debug, Copy, Clone, Builder)]
pub struct RendererConfiguration {
    pub instance_flags: InstanceFlags,
    pub window_width: u32,
    pub window_height: u32,
}

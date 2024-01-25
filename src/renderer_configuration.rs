use wgpu::InstanceFlags;

pub struct RendererConfiguration {
    pub instance_flags: InstanceFlags,
    pub window_width: u32,
    pub window_height: u32,
}

impl RendererConfiguration {
    pub fn new() -> Self {
        Self {
            instance_flags: InstanceFlags::VALIDATION,
            window_width: 800,
            window_height: 600,
        }
    }
}

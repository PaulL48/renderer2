#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PipelineConfiguration {
    pub shader_path: String,
    pub vertex_shader_entrypoint: String,
    pub vertex_buffer_layouts: Vec<wgpu::VertexBufferLayout<'static>>,
    pub topology: wgpu::PrimitiveTopology,
    pub strip_index_format: Option<wgpu::IndexFormat>,
    pub front_face: wgpu::FrontFace,
    pub cull_mode: Option<wgpu::Face>,
    pub polygon_mode: wgpu::PolygonMode,
    pub depth_write_enabled: bool,
    pub depth_compare_function: wgpu::CompareFunction,
    pub fragment_shader_entrypoint: String,
    pub fragment_shader_blend_mode: Option<wgpu::BlendState>,
    pub fragment_shader_write_mask: wgpu::ColorWrites,
    pub bind_group_layouts: Vec<Vec<wgpu::BindGroupLayoutEntry>>,
}

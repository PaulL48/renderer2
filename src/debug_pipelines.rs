use once_cell::sync::Lazy;
use wgpu::{BindGroupLayoutEntry, BindingType, BufferBindingType, ColorWrites, CompareFunction, FrontFace, IndexFormat, PolygonMode, PrimitiveTopology, ShaderStages, VertexAttribute, VertexBufferLayout, VertexFormat, VertexStepMode};

use crate::PipelineConfiguration;

const LINE_LIST_VERTEX_BUFFER_LAYOUT: VertexBufferLayout = VertexBufferLayout {
    array_stride: std::mem::size_of::<[f32; 2]>() as u64,
    step_mode: VertexStepMode::Vertex,
    attributes: &[
        VertexAttribute {
            format: VertexFormat::Float32x2,
            offset: 0,
            shader_location: 0,
        }
    ],
};

pub static LINE_WORLD_SPACE_DEPTH_ENABLED_DEBUG_PIPELINE: Lazy<PipelineConfiguration> = Lazy::new(|| PipelineConfiguration {
    shader_path: "./debug_shaders/".to_string(),
    vertex_shader_entrypoint: "vs_main".to_string(),
    vertex_buffer_layouts: vec![LINE_LIST_VERTEX_BUFFER_LAYOUT],
    topology: PrimitiveTopology::LineList,
    strip_index_format: None,
    front_face: FrontFace::Ccw,
    cull_mode: None,
    polygon_mode: PolygonMode::Fill,
    depth_write_enabled: true,
    depth_compare_function: CompareFunction::LessEqual,
    fragment_shader_entrypoint: "fs_main".to_string(),
    fragment_shader_blend_mode: None,
    fragment_shader_write_mask: ColorWrites::ALL,
    bind_group_layouts: vec![vec![
        // Camera
        BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStages::VERTEX_FRAGMENT,
            ty: BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None
            }, 
            count: None
        }
    ]],
});
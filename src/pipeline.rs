// With the previous renderer we specified a list of mesh-submesh groups for a pipeline and
// then in the render function expose the this list for the user to specify calls for as they like

// This might be better off as a higher level API where the user simply asks the renderer to render
// a mesh-submesh group when they want and not store anything persistent on the renderer side

// This leave open how a particular model will specify how to render itself if we only have a
// .draw() method on it (or submit_draw())
// We could supply a state modifying function that is run per-submesh to configure the vertex buffer
// the bind groups for materials and the indices. Instancing would seem to be better either left out
// or handled transparently in the renderer

// Some challenges:
// The render pass is a temporary object that only exists briefly to allow command buffer creation

use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, PipelineLayoutDescriptor, RenderPipeline,
    RenderPipelineDescriptor, TextureFormat, VertexState,
};

use crate::{pipeline_configuration::PipelineConfiguration, Renderer};

pub struct Pipeline {
    pipeline: RenderPipeline,
    configuration: PipelineConfiguration,
    bind_group_layouts: Vec<BindGroupLayout>,

    
}

impl Pipeline {
    pub fn from_configuration(
        configuration: PipelineConfiguration,
        device: &wgpu::Device,
        surface_format: &TextureFormat,
    ) -> Result<Self, String> {
        let label = format!("pipeline({})", configuration.shader_path);
        let mut bind_group_layouts = Vec::new();
        for (i, bind_group_layout) in configuration.bind_group_layouts.iter().enumerate() {
            let label = format!("{}/bind_group_layout({})", label, i);
            let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                entries: bind_group_layout,
                label: Some(&label),
            });
            bind_group_layouts.push(layout);
        }

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some(&format!("{}/render_pipeline_layout", label)),
            bind_group_layouts: &bind_group_layouts.iter().collect::<Vec<_>>(),
            push_constant_ranges: &[],
        });

        let shader_source = std::fs::read_to_string(configuration.shader_path.clone())
            .map_err(|e| e.to_string())?;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(&format!("{}/vertex_shader", label)),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some(&label),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: &configuration.vertex_shader_entrypoint,
                buffers: &configuration.vertex_buffer_layouts,
            },
            primitive: wgpu::PrimitiveState {
                topology: configuration.topology,
                strip_index_format: configuration.strip_index_format,
                front_face: configuration.front_face,
                cull_mode: configuration.cull_mode,
                polygon_mode: configuration.polygon_mode,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Renderer::DEPTH_FORMAT,
                depth_write_enabled: configuration.depth_write_enabled,
                depth_compare: configuration.depth_compare_function,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: &configuration.fragment_shader_entrypoint,
                targets: &[Some(wgpu::ColorTargetState {
                    format: *surface_format,
                    blend: configuration.fragment_shader_blend_mode,
                    write_mask: configuration.fragment_shader_write_mask,
                })],
            }),
            multiview: None,
        });

        Ok(Self {
            pipeline,
            configuration,
            bind_group_layouts,
        })
    }
}

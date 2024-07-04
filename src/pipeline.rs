use crate::{
    material_cache::MaterialCache,
    mesh::{Mesh, MeshHandle},
    pipeline_configuration::PipelineConfiguration,
    uniform_group::{UniformGroup, UniformGroupSource},
    Renderer,
};
use slot_map::SlotMap;
use wgpu::{
    BindGroupLayout, BindGroupLayoutDescriptor, CommandEncoderDescriptor, Device,
    PipelineCompilationOptions, PipelineLayoutDescriptor, Queue, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, StoreOp, TextureFormat, TextureView, VertexState,
};

pub struct Pipeline {
    pipeline: RenderPipeline,
    configuration: PipelineConfiguration,
    bind_group_layouts: Vec<BindGroupLayout>,
    global_bind_groups: Vec<UniformGroup>,
    draw_queue: Vec<MeshHandle>,
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
                compilation_options: PipelineCompilationOptions::default(),
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
                compilation_options: PipelineCompilationOptions::default(),
            }),
            multiview: None,
        });

        Ok(Self {
            pipeline,
            configuration,
            bind_group_layouts,
            global_bind_groups: Vec::new(),
            draw_queue: Vec::new(),
        })
    }

    pub fn submit_mesh(
        &mut self,
        device: &Device,
        queue: &Queue,
        render_target: &TextureView,
        depth_texture: &TextureView,
        mesh: MeshHandle,
        mesh_cache: &SlotMap<Mesh>,
        material_cache: &MaterialCache,
    ) {
        self.draw_queue.push(mesh);

        // Check the heuristic for submission
        if self.draw_queue.len() >= 5 {
            self.submit_pending_draws(
                device,
                queue,
                render_target,
                depth_texture,
                mesh_cache,
                material_cache,
            );
        }
    }

    pub fn add_global_bind_group(&mut self, source: &UniformGroupSource, device: &Device) {
        let uniform_group = UniformGroup::from_source(source, device);
        self.global_bind_groups.push(uniform_group);
    }

    pub fn flush_queue(
        &mut self,
        device: &Device,
        queue: &Queue,
        render_target: &TextureView,
        depth_texture: &TextureView,
        mesh_cache: &SlotMap<Mesh>,
        material_cache: &MaterialCache,
    ) {
        if self.draw_queue.is_empty() {
            return;
        }

        self.submit_pending_draws(
            device,
            queue,
            render_target,
            depth_texture,
            mesh_cache,
            material_cache,
        );
    }

    fn submit_pending_draws(
        &mut self,
        device: &Device,
        queue: &Queue,
        render_target: &TextureView,
        depth_texture: &TextureView,
        mesh_cache: &SlotMap<Mesh>,
        material_cache: &MaterialCache,
    ) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor { label: None });
        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Set pipeline
            render_pass.set_pipeline(&self.pipeline);

            // Set global bind groups
            for (i, global_bind_group) in self.global_bind_groups.iter().enumerate() {
                render_pass.set_bind_group(i as u32, global_bind_group.bind_group(), &[]);
            }

            // Record commands
            for mesh_handle in &self.draw_queue {
                let mesh = mesh_cache.get(mesh_handle).unwrap();
                mesh.record_commands(
                    &mut render_pass,
                    self.global_bind_groups.len() as u32,
                    material_cache,
                );
            }
        }

        let command_buffer = encoder.finish();
        queue.submit([command_buffer]);
    }
}

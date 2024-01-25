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

use std::collections::HashMap;

use slot_map::SlotMap;
use wgpu::{
    BindGroup, BindGroupLayout, BindGroupLayoutDescriptor,
    CommandEncoderDescriptor, Device, PipelineLayoutDescriptor, Queue, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, StoreOp, TextureFormat, TextureView, VertexState,
};

use crate::{
    material::Material,
    mesh::{Mesh, MeshHandle},
    pipeline_configuration::PipelineConfiguration,
    Renderer,
};

pub struct Pipeline {
    pipeline: RenderPipeline,
    configuration: PipelineConfiguration,
    bind_group_layouts: Vec<BindGroupLayout>,
    global_bind_groups: Vec<BindGroup>,
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
        material_cache: &HashMap<&'static str, Material>,
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

    pub fn add_global_bind_group(&mut self) {
        todo!()
    }

    pub fn flush_queue(
        &mut self,
        device: &Device,
        queue: &Queue,
        render_target: &TextureView,
        depth_texture: &TextureView,
        mesh_cache: &SlotMap<Mesh>,
        material_cache: &HashMap<&'static str, Material>,
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
        material_cache: &HashMap<&'static str, Material>,
    ) {
        // Submit this work
        // create

        // Do any data synchronization that the models need

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
                render_pass.set_bind_group(i as u32, global_bind_group, &[]);
            }

            // Record commands
            for mesh_handle in &self.draw_queue {
                let mesh = mesh_cache.get(mesh_handle).unwrap();
                mesh.record_commands(&mut render_pass, 1, material_cache);
            }
        }

        let command_buffer = encoder.finish();
        queue.submit([command_buffer]);
    }
}

// This seems logical:
// Each variant of BindingType is being covered in different pathways
// to do their own things

// For Sampler/Texture we've got Material

// For Buffer we'll have its own thing too

// We want to make it rather more generic than the previous way the camera was setup
// but at the same time we don't want to make setting the camera as a pipeline global
// annoying for *every* pipeline

// What do we need? We need a &[u8] and an entry layout for every buffer entry
// Though even then the layout for a buffer entry is pretty simple and the same for
// every single one, unless it wants to specify an offset

// The other unique consideration here is that we're likely updating these buffers
// every frame for the camera.

// We could use a handle: BufferHandle or UniformHandle

// The other consideration is that we don't have a handle to a buffer,
// we have a handle to a bind group element

// but in the end we update buffers and bind bind groups

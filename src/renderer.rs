use crate::{
    material::{Material, MaterialSource}, material_cache::MaterialCache, mesh::{Mesh, MeshHandle, MeshSource}, pipeline::Pipeline, pipeline_configuration::PipelineConfiguration, renderer_configuration::RendererConfiguration, texture::Texture, uniform_group::UniformGroupSource
};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use slot_map::{SlotMap, SlotMapIndex};
use std::collections::{HashMap, HashSet};
use wgpu::{
    Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Dx12Compiler, Features,
    Gles3MinorVersion, Instance, InstanceDescriptor, Limits, PowerPreference, PresentMode, Queue,
    RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration,
    SurfaceTexture, TextureFormat, TextureView,
};

pub struct Renderer {
    surface: Surface,
    surface_configuration: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    depth_texture: Texture,
    pipelines: SlotMap<Pipeline>,
    pipeline_lookup: HashMap<PipelineConfiguration, SlotMapIndex>,

    mesh_cache: SlotMap<Mesh>, // The meshes/sub_meshes need to be accessed when the mesh handle is returned
    material_cache: MaterialCache,

    surface_texture: Option<SurfaceTexture>,
    surface_view: Option<TextureView>,
}

impl Renderer {
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn new<W>(window: &W, configuration: &RendererConfiguration) -> Result<Self, String>
    where
        W: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let instance_descriptor = InstanceDescriptor {
            backends: /*wgpu::Backends::DX11
                | wgpu::Backends::DX12
                | wgpu::Backends::GL */
                /*|*/ wgpu::Backends::VULKAN,
                //| wgpu::Backends::METAL,
            flags: configuration.instance_flags,
            dx12_shader_compiler: Dx12Compiler::Fxc,
            gles_minor_version: Gles3MinorVersion::Automatic,
        };
        let instance = Instance::new(instance_descriptor);
        let surface = unsafe { instance.create_surface(window).unwrap() };
        let adapter = instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        });
        let adapter = pollster::block_on(adapter)
            .ok_or(String::from("Failed to acquire graphics adapter"))?;

        let device_queue = adapter.request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::downlevel_defaults(),
            },
            None,
        );

        let (device, queue) = pollster::block_on(device_queue).map_err(|e| e.to_string())?;
        let capabilities = surface.get_capabilities(&adapter);
        let desired_surface_formats = {
            let mut h = HashSet::new();
            h.insert(TextureFormat::Bgra8UnormSrgb);
            h.insert(TextureFormat::Rgba8UnormSrgb);
            h
        };

        let format = capabilities
            .formats
            .iter()
            .find(|f| desired_surface_formats.contains(f))
            .ok_or(String::from("Failed to acquire compatible surface"))?;

        let surface_configuration = if capabilities.present_modes.contains(&PresentMode::Mailbox) {
            wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: *format,
                width: configuration.window_width,
                height: configuration.window_height,
                present_mode: wgpu::PresentMode::Mailbox,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: [].to_vec(),
            }
        } else {
            wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: *format,
                width: configuration.window_width,
                height: configuration.window_height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                view_formats: [].to_vec(),
            }
        };

        surface.configure(&device, &surface_configuration);

        let depth_texture = Texture::new_depth_texture(
            &device,
            configuration.window_width,
            configuration.window_height,
            "depth_texture",
            &Renderer::DEPTH_FORMAT,
        );

        let output = match surface.get_current_texture() {
            Ok(output) => output,
            Err(err) => panic!("Could not get surface for rendering: {}", err),
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(surface_configuration.format),
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        Ok(Self {
            surface,
            surface_configuration,
            device,
            queue,
            depth_texture,
            pipelines: SlotMap::with_capacity(12),
            pipeline_lookup: HashMap::new(),
            mesh_cache: SlotMap::with_capacity(12),
            material_cache: MaterialCache::new(),
            surface_texture: Some(output),
            surface_view: Some(view),
        })
    }

    // Functions to preload meshes, materials and pipelines.
    // Necessary to render a mesh and material
    pub fn register_pipeline(
        &mut self,
        configuration: &PipelineConfiguration,
    ) -> Result<(), String> {
        let pipeline = Pipeline::from_configuration(
            configuration.clone(),
            &self.device,
            &self.surface_configuration.format,
        )?;

        let index = self.pipelines.push(pipeline);
        self.pipeline_lookup.insert(configuration.clone(), index);
        Ok(())
    }

    // Having separate mesh and material registration might be
    // problematic.
    pub fn register_mesh(&mut self, mesh_source: &MeshSource) -> MeshHandle {
        let mesh = Mesh::from_source(&self.device, &self.pipeline_lookup, mesh_source);
        self.mesh_cache.push(mesh)
    }

    pub fn register_material(&mut self, material_source: &MaterialSource) {
        let material = Material::from_source(material_source, &self.device, &self.queue);
        self.material_cache.insert(material.id(), material);
    }

    pub fn unregister_mesh() {
        todo!()
    }
    pub fn unregister_material() {
        todo!()
    }

    pub fn submit_mesh(&mut self, mesh_handle: MeshHandle) {
        let mesh = self.mesh_cache.get(&mesh_handle).unwrap();

        self.pipelines
            .get_mut(mesh.pipeline())
            .unwrap()
            .submit_mesh(
                &self.device,
                &self.queue,
                self.surface_view.as_ref().unwrap(),
                self.depth_texture.view(),
                mesh_handle,
                &self.mesh_cache,
                &self.material_cache,
            );
    }

    pub fn add_pipeline_global(
        &mut self,
        pipeline: &PipelineConfiguration,
        uniform_group: &UniformGroupSource,
    ) {
        let pipeline = self.pipeline_lookup.get(pipeline).unwrap();
        let pipeline = self.pipelines.get_mut(pipeline).unwrap();
        pipeline.add_global_bind_group(uniform_group, &self.device);
    }

    pub fn render(&mut self) {
        // Force all pipelines to submit now
        for pipeline in &mut self.pipelines {
            pipeline.flush_queue(
                &self.device,
                &self.queue,
                self.surface_view.as_ref().unwrap(),
                self.depth_texture.view(),
                &self.mesh_cache,
                &self.material_cache,
            )
        }

        {
            // Take and present the built surface
            let texture = self.surface_texture.take().unwrap();
            self.surface_view.take().unwrap();
            texture.present();
        }

        // Reacquire surfaces
        let texture = self
            .surface
            .get_current_texture()
            .expect("Could not get next frame buffer");

        let view = texture.texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(self.surface_configuration.format),
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        self.surface_texture = Some(texture);
        self.surface_view = Some(view);

        // Provide a basic clear op immediately
        // It is unlikely that the GPU is busy at this point so there should
        // be low performance impact
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Clear"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: self.surface_view.as_ref().unwrap(),
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(Color {
                        r: 0.6,
                        g: 0.6,
                        b: 0.6,
                        a: 0.6,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: self.depth_texture.view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let commands = encoder.finish();
        self.queue.submit([commands]);
    }
}

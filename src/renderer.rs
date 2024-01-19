use crate::{
    material::{MaterialSource, Material},
    mesh::MeshSource,
    mesh_cache::{MeshCache, MeshHandle},
    pipeline::Pipeline,
    pipeline_configuration::PipelineConfiguration,
    renderer_configuration::RendererConfiguration,
    texture::Texture,
};
use log::warn;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use slot_map::{SlotMap, SlotMapIndex};
use std::collections::{HashMap, HashSet};
use wgpu::{
    CommandBuffer, Device, DeviceDescriptor, Dx12Compiler, Features, Gles3MinorVersion, Instance,
    InstanceDescriptor, Limits, PowerPreference, PresentMode, Queue, RequestAdapterOptions,
    Surface, SurfaceConfiguration, TextureFormat,
};

pub struct Renderer {
    surface: Surface,
    surface_configuration: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    depth_texture: Texture,
    command_buffers: Vec<CommandBuffer>,

    pipelines: SlotMap<Pipeline>,
    pipeline_lookup: HashMap<PipelineConfiguration, SlotMapIndex>,


    mesh_cache: MeshCache, // The meshes/sub_meshes need to be accessed when the mesh handle is returned
    material_cache: HashMap<&'static str, Material>,
}

impl Renderer {
    pub const DEPTH_FORMAT: TextureFormat = TextureFormat::Depth32Float;

    pub fn new<W>(window: &W, configuration: &RendererConfiguration) -> Result<Self, String>
    where
        W: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let instance_descriptor = InstanceDescriptor {
            backends: wgpu::Backends::DX11
                | wgpu::Backends::DX12
                | wgpu::Backends::GL
                | wgpu::Backends::VULKAN
                | wgpu::Backends::METAL,
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

        Ok(Self {
            surface,
            surface_configuration,
            device,
            queue,
            depth_texture,
            command_buffers: Vec::new(),
            pipelines: SlotMap::with_capacity(12),
            pipeline_lookup: HashMap::new(),
            mesh_cache: MeshCache::with_capacity(12),
            material_cache: HashMap::new(),
        })
    }

    // Functions to preload meshes, materials and pipelines.
    // Necessary to render a mesh and material
    pub fn register_pipeline(
        &mut self,
        configuration: PipelineConfiguration,
    ) -> Result<(), String> {
        let pipeline = Pipeline::from_configuration(
            configuration.clone(),
            &self.device,
            &self.surface_configuration.format,
        )?;

        let index = self.pipelines.push(pipeline);
        self.pipeline_lookup.insert(configuration, index);
        Ok(())
    }

    // Having separate mesh and material registration might be
    // problematic.
    pub fn register_mesh(&mut self, mesh_source: &dyn MeshSource) -> MeshHandle {
        self.mesh_cache.insert(&self.device, mesh_source)
    }

    pub fn register_material(&mut self, material_source: &dyn MaterialSource) {
        let material = Material::from_source(material_source, &self.device, &self.queue);
        self.material_cache.insert(material.name(), material);
    }

    pub fn unregister_mesh() { todo!() }
    pub fn unregister_material() { todo!() }

    pub fn submit_mesh() {}

    pub fn render(&mut self) {
        let output = match self.surface.get_current_texture() {
            Ok(output) => output,
            Err(err) => {
                warn!("Could not get surface for rendering: {}", err);
                return;
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor {
            label: None,
            format: Some(self.surface_configuration.format),
            dimension: None,
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: None,
            base_array_layer: 0,
            array_layer_count: None,
        });

        // Instead of assigning a pipeline to create the command to clear the
        // view we should do that right here

        // Here we need to generate all the command buffers needed to render
        // the frame

        // CommandEncoders are generated lifetime-free so this is the break
        // point
        
        // Nvidia recommends 5-10 equivalents of queue.submit
        // And it recommends that they are dispatched *throughout* the frame generation
        // rather than only at the end

        // So to start we can submit a CB for clearing the render target and depth
        // buffers (we would actually want to do that AFTER we submit all the work
        // for the current frame so it can process the clear as soon as possible)

        // Now throughout the frame what do we do

        // We collect calls to submit_mesh() throughout the frame
        // Each of these call will be associated with a particular pipeline
        // each pipeline in turn needs its own command buffer (technically
        // not since we can switch pipelines in the command buffer. But
        // indeed pipeline switches are HEAVY)
        // So how does submit mesh work then?
        // if a submit comes in for a model such that they interleave pipelines
        // Then we're dead in the water

        // So each pipeline should have a separate set of command buffers
        // each following the heuristic pattern below?

        // We will ignore multithreading for now since we would need to
        // share threads with a library

        // The process to create and submit a command buffer is:
        // Create an encoder using Device
        // Then to record commands, create a render pass from the encoder
        // commands are recorded
        // the render pass is dropped
        // optionally more passes (computer, or more render) can be created
        // A CommandBuffer is created by .finish()ing the encoder
        // the CommandBuffer is submitted to the GPU via Queue::submit()

        // It feels like we want an encoder to have an active render pass
        // for every pipeline which we can then somehow sleep (the lifetime)
        // and wait for more commands to arrive, until we hit a threshold
        // then we destroy and finish the render pass/encoder and submit it

        // The main challenge is:
        // How do we create a render pass such that it lasts exactly as long
        // as we need it to. Ie. Create it in one scope, submit to it in another
        // and then decide that it is time to destroy it in a third...
        //
        // Feels like we could store the pass in an option in a small struct
        // that bounds the lifetimes correctly. Then we can cause a drop
        // at any time by reset()ing the option but this add the matching
        // overhead to every mesh_submit call
        //
        // Lets think: So technically if we name a lifetime for the encoder
        // and the render pass, they are identical, so the 
    }
}

// As a side note we could use a heuristic check on the number of submitted buffers
// to increase or decrease the number of work submissions that happen per frame
//
// For example we have a mesh_submits_per_gpu_submit and target_gpu_submits
// Then as we record our commands we use mesh_submits_per_gpu_submit to truncate
// command buffers and submit them to the GPU, recording each gpu submit
// Then at the end of a frame we check if we're over or under target_gpu_submits
// and then modify mesh_submits_per_gpu_submit to inch closer to that target

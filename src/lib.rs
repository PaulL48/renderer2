mod command_buffer_heuristics;
mod command_source;
mod foo;
mod material;
mod material_cache;
mod mesh;
mod pipeline;
mod pipeline_configuration;
mod renderer;
mod renderer_configuration;
mod sub_mesh;
mod texture;

pub use renderer::Renderer;
pub use renderer_configuration::RendererConfiguration;

// Two traits and two types are emerging as an interface
// SubMeshSource, MaterialSource, Renderer and DebugRenderer

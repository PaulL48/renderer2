use slot_map::{SlotMap, SlotMapIndex};
use wgpu::Device;
use crate::{mesh::MeshHandle, uniform_group::UniformGroup, UniformGroupSource, UniformSource};

pub struct DebugPrimitive {
    mesh_handle: MeshHandle,
    primitive_handle: SlotMapIndex,
    expiry: u64,
}

impl DebugPrimitive {
    pub fn mesh(&self) -> MeshHandle {
        self.mesh_handle
    }

    pub fn expiry(&self) -> u64 {
        self.expiry
    }

    pub fn primitive_handle(&self) -> SlotMapIndex {
        self.primitive_handle
    }
}

pub struct DebugData {
    camera: UniformGroup,
    pub current_simulation_time: u64,
    pub primitives: SlotMap<DebugPrimitive>,
    // The purpose of this is to hold the debug primitives to be rendered
}

impl DebugData {
    pub fn new(device: &Device) -> Self {
        let default_matrix = [
            // view
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0,

            // projection
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ];
        let uniform_source = UniformSource {
            data: bytemuck::cast_slice(&default_matrix).to_vec(),
        };

        let uniform_group_source = UniformGroupSource {
            name: "debug_camera".to_string(),
            uniform_sources: vec![uniform_source],
        };

        let uniform_group = UniformGroup::from_source(&uniform_group_source, device);

        Self {
            camera: uniform_group,
            current_simulation_time: 0,
            primitives: SlotMap::with_capacity(12),
        }
    }

    pub fn set_debug_camera() {

    }

    pub fn set_debug_time() {
        
    }
}

// Steps needed to implement debug rendering
// Add way for user to update debug camera and debug time
// 
// Similar to how a user stores mesh handles, create a list of mesh handles
// Add an expiry timestamp to the stored mesh handles
// Add a scan loop that removes expired debug primitives
// Add pipelines as needed



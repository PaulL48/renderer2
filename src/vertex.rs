// A vertex is user-space defined collection of attributes that is sent to the GPU on a per-vertex basis
// We setup a contract that specifies how we transport a vertex to the GPU 

// Here we should consider how the different types of data are sent to the GPU
// We have:
//      pipeline configuration
//          Data: shader entry point names, vertex buffer layouts, topology type
//          winding, cull mode, polygon mode, depth write enable/disable, fragment shader blend mode
//
//          constraints: Pipeline configuration must be constant for the lifetime of the pipeline
//      
//          Handling: As in the current renderer, this data is supplied as a static (or not) struct
//          that is hashable to create a way to uniquely identify a particular pipeline config.
//          one modification that might be good is allowing a render submit to specify a pipeline
//          with an interned string ID
//
//      vertex buffer data
//          Data: a binary buffer of data whose contents is interpreted according to the vertex
//          buffer layout which is updated once or periodically to contain up to date vertex data
//          for a particular draw call
//
//          constraints: Data must be managed as a buffer on the GPU and so a mechanism must be
//          present to make the GPU-side buffer current with the CPU-side data
//
//          handling: The previous renderer used the AsPod trait to make sure the data can be
//          converted into a binary buffer consistent with the desired vertex data. as well a
//          buffer layout is supplied on the user-side (through the pipeline configuration)
//          that specifies how the GPU should interpret the contents of the buffer
//
//      uniforms/bind group data
//          Data: This is either arbitrary binary data or image data (as textures and samplers)
//          whose purpose is determined by the bind group layout supplied in the pipeline config
//
//          constraints: Data must be managed as a bind group (wgpu::BindGroup) which is similar
//          to a raw buffer
//
//          handling: The current renderer uses a BufferedData struct that takes a T that is the
//          data it will buffer. I do not like this implementation. It creates a large constraint
//          on how the cpu-side data is stored and it reimplements AsPod
//

// Unfortunately the layouts for the vertex buffer and the bind groups have a different type
// otherwise it would seem like a good idea to create a unified cpu-type -> layout + binary repr

// Another thing to note is that lots of structures follow a pattern of core-data + support data
// ex. A mesh is a list of vertices, but to get a mesh onto the GPU we need buffers for the vertex
// data and index data. These buffers are auxiliary to the mesh but necessary for it to function

struct ExampleVertex {
    position: f32,
    color: f32
}

impl ExampleVertex {

    // This or a static data member. Return the VertexBufferLayout for this vertex
    fn vertex_buffer_layout() -> () {

    }

    // Return the vertex as it will appear in the vertex buffer on the GPU
    fn into_vertex_buffer_data() {

    }
}


struct ExampleBindGroup {

}

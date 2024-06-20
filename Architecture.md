## Low Level Objects

The following sections describe objects that abstractions over lower level graphics primitives and as such items such as shaders, bind groups and synchronization primitives are not described here. These objects are considered low level due to them directly storing graphics API objects.

### Sub-Mesh
A sub-mesh is a grouping of vertices that describes a three dimensional shape or line sequence that are rendered in one draw command using one primitive drawing mode, such as triangle strip, triangle fans, lines, etc...

A mesh is a collection of sub meshes that belong to the same semantic object. For example, a light fixture could have one sub-mesh for the light bulb and another for the stand. It is possible for a mesh to have only one sub-mesh, for example a cup.

### Texture
A texture is usually an image that contains color or other data used to render the surface of a sub-mesh. A material is used to group several textures that all contribute to the rendering of a single surface. For example, a surface might have an albedo texture that represents the colors of the surface material, a roughness texture that represents how rough the surface material is rendered and a metal-ness texture that represents areas of the surface that are metallic.

Not all textures contain direct surface information and can be used to store arbitrary data that is passed to the GPU as a matrix that can be sampled.

### Uniform
A uniform is a block of structured data that is passed to the GPU. It can represent a variety of information such as the current camera matrices, positions of lights in the scene, etc... Uniforms are grouped together into uniform groups, which correspond with bind groups on the GPU.

## High Level Objects

Meshes, materials and uniform groups are described in the sub-mesh, texture and uniform sections respectively since they are so tightly linked.

### Pipeline
A pipeline is a concept that groups together shaders, uniform groups and other rendering parameters to render sub-meshes and their materials in a single fixed way. Each mesh is authored to be rendered by a specific rendering pipeline. The parameters used to configure a pipeline are stored in a pipeline configuration.

### Material Cache
The material cache stores materials as they are loaded along side meshes. The cache allows sub meshes to store a handle into the cache to retrieve the material resources when necessary

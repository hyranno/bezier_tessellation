use vulkano::{
    buffer::BufferContents,
    pipeline::graphics::vertex_input::Vertex,
};


#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct VertexData {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
}
#[derive(BufferContents)]
#[repr(C)]
pub struct Normal {
    pub normal: [f32; 4],  /* include padding */
}
#[derive(BufferContents)]
#[repr(C)]
pub struct Edge {
    pub vertices: [u32; 2],
    pub normals: [u32; 2],
}
#[derive(BufferContents)]
#[repr(C)]
pub struct Face {
    pub edges: [u32; 3],
}

pub struct Mesh {
    pub vertices: Vec<VertexData>,
    pub normals: Vec<Normal>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub vertex_indices: Vec<u32>,
}

#[derive(Default, Copy, Clone, PartialEq)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
    pub uv: [f32; 2]
}
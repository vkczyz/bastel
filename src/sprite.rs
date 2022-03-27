use crate::shaders::Shader;
use crate::vertex::Vertex;

pub struct Sprite {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub shader: Shader,
}

impl Sprite {
    pub fn new(position: (f32, f32), size: (f32, f32), shader: Option<Shader>) -> Self {
        let vertices = vec!(
            Vertex { position: [position.0, position.1] },
            Vertex { position: [position.0, position.1 + size.1] },
            Vertex { position: [position.0 + size.0, position.1 + size.1] },
            Vertex { position: [position.0 + size.0, position.1] },
        );
        let indices = vec!(0, 1, 2, 2, 3, 0);

        let shader = match shader {
            Some(s) => s,
            None => Shader::Solid,
        };

        Sprite {
            position,
            size,
            vertices,
            indices,
            shader,
        }
    }
}
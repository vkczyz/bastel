use crate::vertex::Vertex;

pub struct Sprite {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub vertices: Vec<Vertex>,
}

impl Sprite {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Self {
        let vertices = vec!(
            Vertex { position: [position.0, position.1] },
            Vertex { position: [position.0, position.1 + size.1] },
            Vertex { position: [position.0 + size.0, position.1] },
            Vertex { position: [position.0 + size.0, position.1 + size.1] },
        );

        Sprite {
            position,
            size,
            vertices,
        }
    }
}
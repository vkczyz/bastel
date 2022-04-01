use crate::shaders::Shader;
use crate::vertex::Vertex;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Clone, PartialEq)]
pub struct Sprite {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub shader: Shader,
    pub texture: Option<Vec<u8>>,
}

impl Sprite {
    pub fn new(position: (f32, f32), size: (f32, f32), shader: Option<Shader>) -> Self {
        let vertices = vec!(
            Vertex {
                position: [position.0, position.1],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [position.0, position.1 + size.1],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1 + size.1],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1],
                uv: [1.0, 0.0],
            },
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
            texture: None,
        }
    }

    pub fn add_texture(&mut self, path: &Path) -> io::Result<()> {
        let path = path.to_str().ok_or(()).unwrap();
        let mut f = File::open(path)?;

        let mut data = vec![];
        f.read_to_end(&mut data)?;

        self.texture = Some(data);
        Ok(())
    }

    pub fn get_left_pos(&self) -> f32 {
        self.position.0
    }

    pub fn get_right_pos(&self) -> f32 {
        self.position.0 + self.size.0
    }

    pub fn get_top_pos(&self) -> f32 {
        self.position.1
    }

    pub fn get_bottom_pos(&self) -> f32 {
        self.position.1 + self.size.1
    }
}
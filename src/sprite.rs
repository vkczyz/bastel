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
    pub color: [f32; 3],
    pub shader: Shader,
    pub texture: Option<Vec<u8>>,
}

impl Sprite {
    pub fn with_color(position: (f32, f32), size: (f32, f32), color: [f32; 3]) -> Self {
        let vertices = vec!(
            Vertex {
                position: [position.0, position.1],
                color,
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [position.0, position.1 + size.1],
                color,
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1 + size.1],
                color,
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1],
                color,
                uv: [1.0, 0.0],
            },
        );
        let indices = vec!(0, 1, 2, 2, 3, 0);

        Sprite {
            position,
            size,
            vertices,
            indices,
            color,
            shader: Shader::Solid,
            texture: None,
        }
    }

    pub fn with_texture(position: (f32, f32), size: (f32, f32), texture_path: &Path) -> io::Result<Self> {
        let color = [0.0, 0.0, 0.0];
        let vertices = vec!(
            Vertex {
                position: [position.0, position.1],
                color,
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [position.0, position.1 + size.1],
                color,
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1 + size.1],
                color,
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1],
                color,
                uv: [1.0, 0.0],
            },
        );
        let indices = vec!(0, 1, 2, 2, 3, 0);

        let texture = Sprite::read_file(texture_path)?;

        Ok(Sprite {
            position,
            size,
            vertices,
            indices,
            color,
            shader: Shader::Texture,
            texture: Some(texture),
        })
    }

    pub fn invisible(position: (f32, f32), size: (f32, f32)) -> Self {
        let vertices = vec!(
            Vertex {
                position: [position.0, position.1],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [position.0, position.1 + size.1],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1 + size.1],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 0.0],
            },
        );
        let indices = vec!(0, 1, 2, 2, 3, 0);

        Sprite {
            position,
            size,
            vertices,
            indices,
            color: [0.0, 0.0, 0.0],
            shader: Shader::Invisible,
            texture: None,
        }
    }

    pub fn rainbow(position: (f32, f32), size: (f32, f32)) -> Self {
        let vertices = vec!(
            Vertex {
                position: [position.0, position.1],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [position.0, position.1 + size.1],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1 + size.1],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [position.0 + size.0, position.1],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 0.0],
            },
        );
        let indices = vec!(0, 1, 2, 2, 3, 0);

        Sprite {
            position,
            size,
            vertices,
            indices,
            color: [0.0, 0.0, 0.0],
            shader: Shader::Rainbow,
            texture: None,
        }
    }

    pub fn change_position(&mut self, pos: (f32, f32)) {
        self.position = pos;
        self.vertices = vec!(
            Vertex {
                position: [pos.0, pos.1],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 0.0],
            },
            Vertex {
                position: [pos.0, pos.1 + self.size.1],
                color: [0.0, 0.0, 0.0],
                uv: [0.0, 1.0],
            },
            Vertex {
                position: [pos.0 + self.size.0, pos.1 + self.size.1],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 1.0],
            },
            Vertex {
                position: [pos.0 + self.size.0, pos.1],
                color: [0.0, 0.0, 0.0],
                uv: [1.0, 0.0],
            },
        );
    }

    pub fn read_file(path: &Path) -> io::Result<Vec<u8>> {
        let path = path.to_str().ok_or(()).unwrap();
        let mut f = File::open(path)?;

        let mut data = vec![];
        f.read_to_end(&mut data)?;

        Ok(data)
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
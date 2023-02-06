use crate::shaders::Shader;
use crate::vertex::Vertex;

use std::fs;
use std::path::Path;

use miniserde;
use miniserde::json;

pub struct SpriteComponent {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub color: [f32; 3],
    pub shader: Shader,
    pub texture: Option<Vec<u8>>,
}

impl SpriteComponent {
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

        SpriteComponent {
            position,
            size,
            vertices,
            indices,
            color,
            shader: Shader::Solid,
            texture: None,
        }
    }

    pub fn with_texture(position: (f32, f32), size: (f32, f32), texture_path: &Path) -> Self {
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

        let (shader, texture) = match fs::read(texture_path) {
            Ok(t) => (Shader::Texture, Some(t)),
            Err(e) => {
                println!("{}", e);
                (Shader::Rainbow, None)
            },
        };

        SpriteComponent {
            position,
            size,
            vertices,
            indices,
            color,
            shader,
            texture,
        }
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

        SpriteComponent {
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

        SpriteComponent {
            position,
            size,
            vertices,
            indices,
            color: [0.0, 0.0, 0.0],
            shader: Shader::Rainbow,
            texture: None,
        }
    }

    #[cfg(feature = "json")]
    pub fn from_json(data: &json::Value) -> Result<Self, &str> {
        let data = match data {
            json::Value::Object(o) => o,
            _ => return Err("Malformed JSON data: expected object"),
        };
        
        let position = match data.get("position") {
            Some(json::Value::Array(a)) => a.iter()
                .map(|n| match n {
                    json::Value::Number(json::Number::F64(i)) => Ok(*i as f32),
                    _ => Err("Malformed JSON data: expected float"),
                })
                .collect::<Result<Vec<f32>, &str>>(),
            _ => return Err("Malformed JSON data: expected array"),
        }?;

        let size = match data.get("size") {
            Some(json::Value::Array(a)) => a.iter()
                .map(|n| match n {
                    json::Value::Number(json::Number::F64(i)) => Ok(*i as f32),
                    _ => Err("Malformed JSON data: expected number"),
                })
                .collect::<Result<Vec<f32>, &str>>(),
            _ => return Err("Malformed JSON data: expected array"),
        }?;

        match data.get("shader") {
            Some(json::Value::String(s)) => {
                match s.as_str() {
                    "solid" => {
                        let color = match data.get("color") {
                            Some(json::Value::Array(a)) => a.iter()
                                .map(|n| match n {
                                    json::Value::Number(json::Number::F64(i)) => Ok(*i as f32),
                                    _ => Err("Malformed JSON data: expected number"),
                                })
                                .collect::<Result<Vec<f32>, &str>>(),
                            _ => Err("Malformed JSON data: expected array"),
                        }?;

                        Ok(SpriteComponent::with_color(
                            (position[0], position[1]),
                            (size[0], size[1]),
                            [color[0], color[1], color[2]],
                        ))
                    },
                    "invisible" => {
                        Ok(SpriteComponent::invisible(
                            (position[0], position[1]),
                            (size[0], size[1]),
                        ))
                    },
                    "rainbow" => {
                        Ok(SpriteComponent::rainbow(
                            (position[0], position[1]),
                            (size[0], size[1]),
                        ))
                    },
                    "texture" => {
                        let texture = match data.get("texture") {
                            Some(json::Value::String(s)) => s,
                            _ => return Err("Malformed JSON data: expected string"),
                        };

                        Ok(SpriteComponent::with_texture(
                            (position[0], position[1]),
                            (size[0], size[1]),
                            Path::new(texture),
                        ))
                    },
                    _ => return Err("Malformed JSON data: unknown shader type"),
                }
            },
            _ => Err("Malformed JSON data: expected valid shader")
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
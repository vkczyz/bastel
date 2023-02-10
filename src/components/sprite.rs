use crate::shaders::Shader;
use crate::components::Component;

use std::fs;
use std::num::ParseIntError;
use std::path::Path;

pub struct SpriteComponent {
    pub shader: Shader,
    pub color: [f32; 3],
    pub texture: Option<Vec<u8>>,
}

impl SpriteComponent {
    /*
    pub fn with_color(position: (f32, f32), size: (f32, f32), color: [f32; 3]) -> Component {
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

        Component::Sprite(
            SpriteComponent {
                position,
                size,
                vertices,
                indices,
                color,
                shader: Shader::Solid,
                texture: None,
            }
        )
    }

    pub fn with_texture(position: (f32, f32), size: (f32, f32), texture_path: &Path) -> Component {
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

        Component::Sprite(
            SpriteComponent {
                position,
                size,
                vertices,
                indices,
                color,
                shader,
                texture,
            }
        )
    }

    pub fn rainbow(position: (f32, f32), size: (f32, f32)) -> Component {
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

        Component::Sprite(
            SpriteComponent {
                position,
                size,
                vertices,
                indices,
                color: [0.0, 0.0, 0.0],
                shader: Shader::Rainbow,
                texture: None,
            }
        )
    }
    */

    pub fn from_xml(data: roxmltree::Node) -> Component {
        let mut shader = Shader::Rainbow;
        let mut color = "#000000";
        let mut texture = None;

        data.attributes()
            .map(|a| {
                match a.name() {
                    "shader" => match a.value() {
                        "solid" => shader = Shader::Solid,
                        "texture" => shader = Shader::Texture,
                        "rainbow" => shader = Shader::Rainbow,
                        _ => shader = Shader::Rainbow,
                    },
                    "color" => color = a.value(),
                    "texture" => texture = fs::read(Path::new(a.value())).ok(),
                    _ => (),
                }
            }
        ).for_each(drop);

        let color = decode_hex(color).unwrap_or(vec![0.0, 0.0, 0.0]);
        let color = [color[0], color[1], color[2]];

        Component::Sprite(
            SpriteComponent {
                shader,
                color,
                texture,
            }
        )
    }

    /*
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
    */
}

fn decode_hex(s: &str) -> Result<Vec<f32>, ParseIntError> {
    (1..s.len())
        .step_by(2)
        .map(|i| match u8::from_str_radix(&s[i..i + 2], 16) {
            Ok(n) => Ok(n as f32),
            Err(e) => Err(e),
        })
        .collect()
}
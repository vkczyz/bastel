use crate::components::Component;
use crate::vertex::Vertex;

pub struct PositionComponent {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl PositionComponent {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Component {
        Component::Position(
            PositionComponent {
                vertices: generate_vertices(position, size),
                indices: generate_indices(),
                position,
                size,
            }
        )
    }

    pub fn from_xml(data: roxmltree::Node) -> Component {
        let mut x = f32::NAN;
        let mut y = f32::NAN;
        let mut width = f32::NAN;
        let mut height = f32::NAN;

        data.attributes()
            .map(|a| {
                match a.name() {
                    "x" => match a.value().parse::<f32>() {
                        Ok(d) => x = d,
                        Err(_) => (),
                    },
                    "y" => match a.value().parse::<f32>() {
                        Ok(d) => y = d,
                        Err(_) => (),
                    },
                    "width" => match a.value().parse::<f32>() {
                        Ok(d) => width = d,
                        Err(_) => (),
                    },
                    "height" => match a.value().parse::<f32>() {
                        Ok(d) => height = d,
                        Err(_) => (),
                    },
                    _ => (),
                }
            }
        ).for_each(drop);

        let position = (x, y);
        let size = (width, height);

        Component::Position(
            PositionComponent {
                vertices: generate_vertices(position, size),
                indices: generate_indices(),
                position,
                size,
            }
        )
    }

    pub fn shift(&mut self, x: f32, y: f32) {
        self.position.0 += x;
        self.position.1 += y;
        self.vertices = generate_vertices(self.position, self.size);
    }
}

fn generate_vertices(position: (f32, f32), size: (f32, f32)) -> Vec<Vertex> {
    vec!(
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
    )
}

fn generate_indices() -> Vec<u16> {
    vec!(0, 1, 2, 2, 3, 0)
}
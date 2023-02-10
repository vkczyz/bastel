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

    pub fn from_xml(data: roxmltree::Node) -> Result<Component, ()> {
        let mut x = f32::NAN;
        let mut y = f32::NAN;
        let mut width = f32::NAN;
        let mut height = f32::NAN;

        data.children()
            .filter(|n| n.is_element())
            .map(|n| {
                match n.tag_name().name() {
                    "x" => match n.attribute("x") {
                        Some(n) => match n.parse::<f32>() {
                            Ok(n) => x = n,
                            Err(_) => (),
                        },
                        None => (),
                    },
                    "y" => match n.attribute("y") {
                        Some(n) => match n.parse::<f32>() {
                            Ok(n) => y = n,
                            Err(_) => (),
                        },
                        None => (),
                    },
                    "width" => match n.attribute("width") {
                        Some(n) => match n.parse::<f32>() {
                            Ok(n) => width = n,
                            Err(_) => (),
                        },
                        None => (),
                    },
                    "height" => match n.attribute("height") {
                        Some(n) => match n.parse::<f32>() {
                            Ok(n) => height = n,
                            Err(_) => (),
                        },
                        None => (),
                    },
                    _ => (),
                }
            }
        ).for_each(drop);

        if x.is_nan() || y.is_nan() || width.is_nan() || height.is_nan() {
            return Err(());
        }

        let position = (x, y);
        let size = (width, height);

        Ok(Component::Position(
            PositionComponent {
                vertices: generate_vertices(position, size),
                indices: generate_indices(),
                position,
                size,

            }
        ))
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
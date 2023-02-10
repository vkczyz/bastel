use crate::components::Component;

pub struct PositionComponent {
    pub position: (f32, f32),
    pub size: (f32, f32),
}

impl PositionComponent {
    pub fn new(position: (f32, f32), size: (f32, f32)) -> Component {
        Component::Position(
            PositionComponent {
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

        Ok(Component::Position(
            PositionComponent {
                position: (x, y),
                size: (width, height),
            }
        ))
    }
}
use crate::components::audio::AudioComponent;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::components::{Component, self};
use std::sync::{Arc, Mutex};
use std::mem::{discriminant, Discriminant};

pub struct Entity {
    pub id: u32,
    pub components: Vec<Component>,
}

impl Entity {
    pub fn new(id: u32, components: Vec<Component>) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(
            Entity {
                id,
                components,
            }
        ))
    }

    pub fn add_component(&mut self, component: Component) {
        self.components.push(component);
    }

    pub fn from_xml(data: roxmltree::Node) -> Arc<Mutex<Self>> {
        let mut id: u32 = 0;
        let mut components = vec![];

        data.children()
            .filter(|n| n.is_element())
            .map(|n| {
                match n.attribute("id") {
                    Some(i) => id = i.as_bytes().iter().sum::<u8>() as u32,
                    None => (),
                }

                match n.tag_name().name() {
                    "audio" => components.push(AudioComponent::from_xml(n)),
                    "position" => components.push(PositionComponent::from_xml(n).unwrap()),
                    "sprite" => components.push(SpriteComponent::from_xml(n)),
                    _ => (),
                }
            }
        ).for_each(drop);

        Arc::new(Mutex::new(
            Entity {
                id,
                components,
            }
        ))
    }
}

pub enum Axis {
    X,
    Y,
}

pub enum Edge {
    Left,
    Right,
    Top,
    Bottom,
}

fn min_f32(a: f32, b: f32) -> f32 {
    match a <= b {
        true => a,
        false => b,
    }
}

fn max_f32(a: f32, b: f32) -> f32 {
    match a >= b {
        true => a,
        false => b,
    }
}
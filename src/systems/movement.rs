use crate::global::Global;
use crate::entity::Entity;
use crate::components::Component;
use crate::components::physics::PhysicsComponent;
use crate::systems::System;

use std::sync::{Arc, Mutex};

pub struct MovementSystem {
    global: Arc<Mutex<Global>>,
}

impl MovementSystem {
    pub fn new(global: Arc<Mutex<Global>>) -> Self {
        MovementSystem {
            global,
        }
    }

    pub fn handle_movement(&self, physics: &mut PhysicsComponent, strength: (f32, f32)) {
        let global = self.global.clone();
        let global = global.lock().expect("Could not unlock global object");

        let up = global.signals.get("up_pressed").unwrap_or(&false);
        let down = global.signals.get("down_pressed").unwrap_or(&false);
        let left = global.signals.get("left_pressed").unwrap_or(&false);
        let right = global.signals.get("right_pressed").unwrap_or(&false);

        let force = get_vector_normalised((
            0.0 + (*right as i32 as f32) - (*left as i32 as f32),
            0.0 + (*down as i32 as f32) - (*up as i32 as f32),
        ));
        let force = (force.0 * strength.0, force.1 * strength.1);
        physics.apply_force(force);
    }
}

impl System for MovementSystem {
    fn run(&mut self, entities: &mut [Arc<Mutex<Entity>>]) {
        for entity in entities {
            let mut moveable = None;
            let mut physics = None;

            let unlocked_entity = entity.clone();
            let mut unlocked_entity = unlocked_entity.lock().expect("Could not acquire entity");
            let components = &mut unlocked_entity.components;

            for component in components.iter_mut() {
                match component {
                    Component::Moveable(c) => moveable = Some(c),
                    Component::Physics(c) => physics = Some(c),
                    _ => {},
                }
            }

            if let (Some(_), Some(physics)) = (moveable, physics) {
                self.handle_movement(physics, (0.001, 0.001));
            }
        }
    }
}

fn get_vector_magnitude(v: (f32, f32)) -> f32 {
    ((v.0 * v.0) + (v.1 * v.1)).sqrt()
}

fn get_vector_normalised(v: (f32, f32)) -> (f32, f32) {
    let mag = get_vector_magnitude(v);
    match mag == 0.0 {
        true => (0.0, 0.0),
        false => (
            v.0 / mag,
            v.1 / mag,
        ),
    }
}
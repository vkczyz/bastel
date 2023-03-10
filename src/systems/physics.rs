use crate::global::Global;
use crate::components::physics::PhysicsComponent;
use crate::components::position::PositionComponent;
use crate::entity::Entity;
use crate::components::Component;
use crate::systems::System;
use std::sync::{Arc, Mutex};

pub struct PhysicsSystem {
    pub external_force: (f32, f32),
    global: Arc<Mutex<Global>>,
}

impl PhysicsSystem {
    pub fn new(global: Arc<Mutex<Global>>) -> Self {
        PhysicsSystem {
            external_force: (0.0, 0.0),
            global,
        }
    }

    fn update_position(&mut self, physics: &mut PhysicsComponent, position: &mut PositionComponent) {
        // Apply external forces (e.g. gravity)
        physics.apply_force(self.external_force);

        /*
        // Set units
        let global = self.global.clone();
        let global = global.lock().expect("Could not unlock global object");
        let units = (
            1.0 / global.view_size.0 as f32,
            1.0 / global.view_size.1 as f32,
        );
        drop(global);


        // Apply jump (if requested)
        if input.up {
            let curve = 1.0;
            let force = (
                0.0,
                units.1 * -12.0 / (curve + entity.airtime as f32),
            );
            if force.1 < 1.0 {
                entity.physics.apply_force(force);
            }
        }
        */

        //entity.airtime += 1;

        physics.update();

        let displ = physics.get_displacement();
        position.shift(displ.0, displ.1);

        physics.reset();
    }
}

impl System for PhysicsSystem {
    fn run(&mut self, entities: &mut [Arc<Mutex<Entity>>]) {
        for entity in entities {
            let mut position = None;
            let mut physics = None;

            let unlocked_entity = entity.clone();
            let mut unlocked_entity = unlocked_entity.lock().expect("Could not acquire entity");
            let components = &mut unlocked_entity.components;

            for component in components.iter_mut() {
                match component {
                    Component::Position(c) => position = Some(c),
                    Component::Physics(c) => physics = Some(c),
                    _ => {},
                }
            }

            if let (Some(physics), Some(position)) = (physics, position) {
                self.update_position(physics, position)
            }
        }
    }
}
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
    pub fn new(global: Arc<Mutex<Global>>, external_force: (f32, f32)) -> Self {
        PhysicsSystem {
            external_force,
            global,
        }
    }

    fn update_position(&mut self, physics: &mut PhysicsComponent, position: &mut PositionComponent) {
        let global = self.global.clone();
        let global = global.lock().expect("Could not unlock global object");
        let units = (
            1.0 / global.view_size.0 as f32,
            1.0 / global.view_size.1 as f32,
        );
        drop(global);

        // Apply external forces (e.g. gravity)
        physics.apply_force(self.external_force);

        /*
        // Apply input forces
        input.handle_movement(
            entity,
            (
                units.0,
                units.1,
            ),
        );

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
            let unlocked_entity = entity.clone();
            let mut unlocked_entity = unlocked_entity.lock().expect("Could not acquire entity");
            let components = &mut unlocked_entity.components.iter_mut();

            // Process entities with PhysicsComponent
            if let Some(Component::Position(position)) = components.find(|c| if let Component::Position(_) = c { true } else { false }) {
                if let Some(Component::Physics(physics)) = components.find(|c| if let Component::Physics(_) = c { true } else { false }) {
                    self.update_position(physics, position)
                }
            }
        }
    }
}
use crate::entity::Entity;
use crate::systems::System;

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub struct Scene {
    pub systems: Vec<Box<dyn System>>,
    pub entities: Vec<Arc<Mutex<Entity>>>,
}

impl Scene {
    pub fn new(entities: Vec<Arc<Mutex<Entity>>>) -> Self {
        Scene {
            entities,
            systems: vec![],
        }
    }

    pub fn add_entity(&mut self, entity: Arc<Mutex<Entity>>) {
        self.entities.push(entity);
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn from_xml(data: &str) -> Self {
        let data = roxmltree::Document::parse(data).expect("Could not parse scene XML");

        let mut entities = vec![];

        data.root_element().children()
            .filter(|n| n.is_element())
            .map(|n| {
                match n.tag_name().name() {
                    "entity" => entities.push(Entity::from_xml(n)),
                    _ => (),
                }
            }
        ).for_each(drop);

        Scene {
            systems: vec![],
            entities,
        }
    }
}
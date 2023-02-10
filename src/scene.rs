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

    /*
    #[cfg(feature = "json")]
    pub fn from_json(data: &json::Value) -> Result<Self, &str> {
        let data = match data {
            json::Value::Object(o) => o,
            _ => return Err("Malformed JSON data: expected object"),
        };

        let force = match data.get("force") {
            Some(json::Value::Array(a)) => a.iter()
                .map(|n| match n {
                    json::Value::Number(json::Number::F64(i)) => Ok(*i as f32),
                    _ => Err("Malformed JSON data: expected float"),
                })
                .collect::<Result<Vec<f32>, &str>>(),
            _ => Ok(vec![0.0, 0.0]),
        }?;
        let force = (force[0], force[1]);

        Ok(Scene {
            entities: match data.get("entities") {
                Some(json::Value::Array(a)) => a.iter()
                    .map(|e| Entity::from_json(e))
                    .collect::<Result<Vec<Entity>, &str>>()?,
                _ => return Err("Malformed JSON data: expected array"),
            },
            systems: vec![],
        })
    }
    */
}
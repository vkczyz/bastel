use crate::entity::Entity;

use std::path::PathBuf;

use miniserde;
use miniserde::json;

pub struct Scene {
    pub entities: Vec<Entity>,
}

impl Scene {
    pub fn new(entities: Vec<Entity>, player_index: usize) -> Self {
        Scene {
            entities,
        }
    }

    /*
    pub fn with_force(entities: Vec<Entity>, player_index: usize, force: (f32, f32)) -> Self {
        Scene {
            entities,
        }
    }
    */

    #[cfg(feature = "json")]
    pub fn from_json(data: &json::Value) -> Result<Self, &str> {
        let data = match data {
            json::Value::Object(o) => o,
            _ => return Err("Malformed JSON data: expected object"),
        };

        /*
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
        */

        Ok(Scene {
            entities: match data.get("entities") {
                Some(json::Value::Array(a)) => a.iter()
                    .map(|e| Entity::from_json(e))
                    .collect::<Result<Vec<Entity>, &str>>()?,
                _ => return Err("Malformed JSON data: expected array"),
            },
        })
    }
}
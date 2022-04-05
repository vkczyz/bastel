use crate::entity::Entity;
use crate::physics::Physics;

use std::path::PathBuf;

use miniserde;
use miniserde::json;

pub struct Scene {
    pub physics: Physics,
    pub entities: Vec<Entity>,
    pub player_index: usize,
    pub bgm: Option<PathBuf>,
}

impl Scene {
    pub fn new(entities: Vec<Entity>, player_index: usize) -> Self {
        let mut physics = Physics::new();
        physics.acceleration.1 = 0.001;

        Scene {
            entities,
            player_index,
            physics,
            bgm: None,
        }
    }

    #[cfg(feature = "json")]
    pub fn from_json(data: &json::Value) -> Result<Self, &str> {
        let data = match data {
            json::Value::Object(o) => o,
            _ => return Err("Malformed JSON data: expected object"),
        };

        Ok(Scene {
            entities: match data.get("entities") {
                Some(json::Value::Array(a)) => a.iter()
                    .map(|e| Entity::from_json(e))
                    .collect::<Result<Vec<Entity>, &str>>()?,
                _ => return Err("Malformed JSON data: expected array"),
            },
            player_index: match data.get("player_index") {
                Some(json::Value::Number(n)) => match n {
                    json::Number::U64(n) => *n as usize,
                    json::Number::I64(n) => *n as usize,
                    _ => return Err("Malformed JSON data: expected integer"),
                }
                _ => return Err("Malformed JSON data: expected number"),
            },
            physics: Physics::new(),
            bgm: match data.get("bgm") {
                Some(json::Value::String(s)) => Some(PathBuf::from(s)),
                _ => None,
            }
        })
    }
}
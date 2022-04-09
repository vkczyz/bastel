use crate::physics::Physics;
use crate::sprite::Sprite;

use miniserde::json;

#[derive(Clone, PartialEq)]
pub struct Entity {
    pub sprite: Sprite,
    pub physics: Physics,
    pub collideable: bool,
}

impl Entity {
    pub fn new(sprite: Sprite, collideable: bool) -> Self {
        Entity {
            sprite,
            physics: Physics::new(1.0),
            collideable,
        }
    }

    pub fn with_mass(sprite: Sprite, collideable: bool, mass: f32) -> Self {
        Entity {
            sprite,
            physics: Physics::new(mass),
            collideable,
        }
    }

    #[cfg(feature = "json")]
    pub fn from_json(data: &json::Value) -> Result<Self, &str> {
        let data = match data {
            json::Value::Object(o) => o,
            _ => return Err("Malformed JSON data: expected object"),
        };

        Ok(Entity {
            sprite: match data.get("sprite") {
                Some(s) => Sprite::from_json(s)?,
                _ => return Err("Malformed JSON data: expected object"),
            },
            collideable: match data.get("collideable") {
                Some(json::Value::Bool(b)) => *b,
                _ => return Err("Malformed JSON data: expected bool")
            },
            physics: match data.get("mass") {
                Some(json::Value::Number(json::Number::F64(n))) => Physics::new(*n as f32),
                _ => Physics::new(1.0)
            },
        })
    }

    pub fn are_colliding(a: &Entity, b: &Entity) -> bool {
        let a = &a.sprite;
        let b = &b.sprite;

        let a_right_b = a.get_left_pos() > b.get_right_pos();
        let a_left_b = a.get_right_pos() < b.get_left_pos();
        let a_above_b = a.get_bottom_pos() < b.get_top_pos();
        let a_below_b = a.get_top_pos() > b.get_bottom_pos();

        !(a_right_b || a_left_b || a_above_b || a_below_b)
    }

    pub fn get_collision_intersection(a: &Entity, b: &Entity) -> [f32; 4] {
        let (left, right, top, bottom) = (0, 1, 2, 3);

        let a_edges = [
            a.sprite.position.0,
            a.sprite.position.0 + a.sprite.size.0,
            a.sprite.position.1,
            a.sprite.position.1 + a.sprite.size.1,
        ];
        let b_edges = [
            b.sprite.position.0,
            b.sprite.position.0 + b.sprite.size.0,
            b.sprite.position.1,
            b.sprite.position.1 + b.sprite.size.1,
        ];

        let intersection_edges = [
            max_f32(a_edges[left], b_edges[left]),
            min_f32(a_edges[right], b_edges[right]),
            max_f32(a_edges[top], b_edges[top]),
            min_f32(a_edges[bottom], b_edges[bottom]),
        ];

        intersection_edges
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

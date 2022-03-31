use crate::physics::Physics;
use crate::sprite::Sprite;

use std::cmp::Ordering::Equal;

#[derive(PartialEq)]
pub struct Entity {
    pub sprite: Sprite,
    pub physics: Physics,
    pub collideable: bool,
}

impl Entity {
    pub fn new(sprite: Sprite, collideable: bool) -> Self {
        Entity {
            sprite,
            physics: Physics::new(),
            collideable,
        }
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

    pub fn get_collision_direction(a: &Entity, b: &Entity) -> Direction {
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

        let distances = a_edges.iter()
            .zip(b_edges.iter())
            .map(|(a, b)| (a - b).abs());

        println!("{:#?}", distances.clone().collect::<Vec<f32>>());

        let shortest_distance_index = distances
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Equal))
            .unwrap().0;

        println!("{:#?}", shortest_distance_index);

        match shortest_distance_index {
            0 => Direction::Left,
            1 => Direction::Right,
            2 => Direction::Up,
            3 => Direction::Down,
            _ => panic!("Received unknown cardinal direction"),
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use crate::components::physics::PhysicsComponent;
use crate::components::position::PositionComponent;
use crate::entity::Entity;
use crate::components::Component;
use crate::systems::System;

use std::sync::Arc;
use std::sync::Mutex;

pub struct CollisionSystem { }

impl CollisionSystem {
    pub fn new() -> Self {
        CollisionSystem {}
    }

    pub fn handle_collision(&self, intersection: &[f32; 4], a: (&mut PositionComponent, &mut PhysicsComponent), b: &mut PositionComponent) {
        let pos_a = a.0;
        let pos_b = b;
        let phys_a = a.1;
        
        let x_dist = intersection[1] - intersection[0];
        let y_dist = intersection[3] - intersection[2];

        let collision_axis = if x_dist < y_dist { Axis::X } else { Axis::Y };
        let edge = match collision_axis {
            Axis::X => {
                phys_a.bounce_x();
                phys_a.friction_y();
                if pos_b.position.0 == intersection[0] { Edge::Left } else { Edge::Right }
            },
            Axis::Y => {
                phys_a.bounce_y();
                phys_a.friction_x();
                if pos_b.position.1 == intersection[2] { Edge::Top } else { Edge::Bottom }
            },
        };

        match edge {
            Edge::Left => {
                pos_a.position.0 -= x_dist;
            },
            Edge::Right => {
                pos_a.position.0 += x_dist;
            },
            Edge::Top => {
                pos_a.position.1 -= y_dist;
                /*
                if phys_a.velocity.1.abs() < global.1.abs() {
                    phys_a.airtime = 0;
                }
                */
            },
            Edge::Bottom => {
                pos_a.position.1 += y_dist;
            },
        }
    }
}

impl System for CollisionSystem {
    fn run(&mut self, entities: &mut [Arc<Mutex<Entity>>]) {
        let entities = entities.to_vec();

        // Check for collisions between all eligible entities
        for a in entities.iter() {
            for b in entities.iter() {
                // Don't check an entity against itself
                if Arc::ptr_eq(a, b) { continue }

                // Only entity A needs a PhysicsComponent, as it is the collider
                let mut pos_a = None;
                let mut pos_b = None;
                let mut phys_a = None;
                let mut coll_a = false;
                let mut coll_b = false;

                let unlocked_a = a.clone();
                let unlocked_b = b.clone();

                let mut unlocked_a = unlocked_a.lock().expect("Could not acquire entity");
                let mut unlocked_b = unlocked_b.lock().expect("Could not acquire entity");

                let components_a = &mut unlocked_a.components;
                let components_b = &mut unlocked_b.components;

                // Extract relevant components
                for component in components_a.iter_mut() {
                    match component {
                        Component::Collision(_) => coll_a = true,
                        Component::Position(c) => pos_a = Some(c),
                        Component::Physics(c) => phys_a = Some(c),
                        _ => {},
                    }
                }

                for component in components_b.iter_mut() {
                    match component {
                        Component::Collision(_) => coll_b = true,
                        Component::Position(c) => pos_b = Some(c),
                        _ => {},
                    }
                }

                // Both entities must be collideable
                if !(coll_a && coll_b) { continue }

                if let (Some(pos_a), Some(pos_b), Some(phys_a)) = (pos_a, pos_b, phys_a) {
                    if !are_colliding(pos_a, pos_b) { continue }

                    let intersection = get_collision_intersection(pos_a, pos_b);
                    self.handle_collision(&intersection, (pos_a, phys_a), pos_b);
                }
            }
        }
    }
}

pub fn are_colliding(a: &PositionComponent, b: &PositionComponent) -> bool {
    let a_right_b = a.get_left_pos() > b.get_right_pos();
    let a_left_b = a.get_right_pos() < b.get_left_pos();
    let a_above_b = a.get_bottom_pos() < b.get_top_pos();
    let a_below_b = a.get_top_pos() > b.get_bottom_pos();

    !(a_right_b || a_left_b || a_above_b || a_below_b)
}

pub fn get_collision_intersection(a: &PositionComponent, b: &PositionComponent) -> [f32; 4] {
    let (left, right, top, bottom) = (0, 1, 2, 3);

    let a_edges = [
        a.position.0,
        a.position.0 + a.size.0,
        a.position.1,
        a.position.1 + a.size.1,
    ];
    let b_edges = [
        b.position.0,
        b.position.0 + b.size.0,
        b.position.1,
        b.position.1 + b.size.1,
    ];

    let intersection_edges = [
        max_f32(a_edges[left], b_edges[left]),
        min_f32(a_edges[right], b_edges[right]),
        max_f32(a_edges[top], b_edges[top]),
        min_f32(a_edges[bottom], b_edges[bottom]),
    ];

    intersection_edges
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
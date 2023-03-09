pub mod audio;
pub mod collision;
pub mod input;
pub mod physics;
pub mod render;
pub mod movement;

use crate::entity::Entity;
use std::sync::Arc;
use std::sync::Mutex;

pub trait System {
    fn run(&mut self, entities: &mut [Arc<Mutex<Entity>>]);
}
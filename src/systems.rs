pub mod audio;
pub mod collision;
pub mod input;
pub mod physics;
pub mod render;
pub mod sprite;

pub trait System {
    fn run(&mut self);
}
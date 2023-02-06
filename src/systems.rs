//pub mod audio;
//pub mod collision;
//pub mod input;
//pub mod physics;
//pub mod sprite;

pub trait System {
    fn run(&mut self);
}
pub mod audio;
pub mod collision;
pub mod input;
pub mod moveable;
pub mod physics;
pub mod sprite;

use crate::components::{
    audio::AudioComponent,
    collision::CollisionComponent,
    input::InputComponent,
    moveable::MoveableComponent,
    physics::PhysicsComponent,
    sprite::SpriteComponent,
};

pub enum Component {
    Audio(AudioComponent),
    Collision(CollisionComponent),
    Input(InputComponent),
    Moveable(MoveableComponent),
    Physics(PhysicsComponent),
    Sprite(SpriteComponent),
}
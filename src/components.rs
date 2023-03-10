pub mod audio;
pub mod collision;
pub mod moveable;
pub mod physics;
pub mod position;
pub mod sprite;

use crate::components::{
    audio::AudioComponent,
    collision::CollisionComponent,
    moveable::MoveableComponent,
    physics::PhysicsComponent,
    position::PositionComponent,
    sprite::SpriteComponent,
};

#[derive(PartialEq)]
pub enum Component {
    Audio(AudioComponent),
    Collision(CollisionComponent),
    Moveable(MoveableComponent),
    Physics(PhysicsComponent),
    Position(PositionComponent),
    Sprite(SpriteComponent),
}
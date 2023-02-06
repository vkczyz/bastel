pub mod audio;
pub mod collision;
pub mod input;
pub mod moveable;
pub mod physics;
pub mod render;
pub mod sprite;

use crate::components::{
    audio::AudioComponent,
    collision::CollisionComponent,
    input::InputComponent,
    moveable::MoveableComponent,
    physics::PhysicsComponent,
    render::RenderComponent,
    sprite::SpriteComponent,
};

pub enum Component {
    Audio(AudioComponent),
    Collision(CollisionComponent),
    Input(InputComponent),
    Moveable(MoveableComponent),
    Physics(PhysicsComponent),
    Render(RenderComponent),
    Sprite(SpriteComponent),
}
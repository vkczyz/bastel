pub mod audio;
pub mod collision;
pub mod sprite;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Component {
    AudioComponent,
    CollisionComponent,
    InputComponent,
    PhysicsComponent,
    SpriteComponent,
}
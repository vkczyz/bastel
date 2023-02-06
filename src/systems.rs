#[derive(Clone, Copy, PartialEq, Eq)]
pub enum System {
    AudioSystem,
    CollisionSystem,
    InputSystem,
    PhysicsSystem,
    SpriteSystem,
}
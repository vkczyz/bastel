#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Component {
    AudioComponent,
    CollisionComponent,
    InputComponent,
    MoveableComponent,
    PhysicsComponent,
    SpriteComponent,
}
use crate::components::Component;

pub struct PhysicsComponent {
    pub mass: f32,
    pub acceleration: (f32, f32),
    pub velocity: (f32, f32),
    friction: f32,
    bounciness: f32,
}

impl Component for PhysicsComponent { }
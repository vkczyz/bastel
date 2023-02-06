use crate::components::Component;

pub struct PhysicsComponent {
    pub mass: f32,
    pub acceleration: (f32, f32),
    pub velocity: (f32, f32),
    friction: f32,
    bounciness: f32,
}

impl Component for PhysicsComponent { }

impl PhysicsComponent {
    pub fn new(mass: f32, friction: f32, bounciness: f32) -> Self {
        PhysicsComponent {
            mass,
            acceleration: (0.0, 0.0),
            velocity: (0.0, 0.0),
            friction,
            bounciness,
        }
    }

    pub fn default() -> Self {
        PhysicsComponent {
            mass: 1.0,
            acceleration: (0.0, 0.0),
            velocity: (0.0, 0.0),
            friction: 0.5,
            bounciness: 1.0,
        }
    }
}
use crate::components::Component;

#[derive(PartialEq)]
pub struct CollisionComponent { }

impl CollisionComponent {
    pub fn new() -> Component {
        Component::Collision(
            CollisionComponent {}
        )
    }
}
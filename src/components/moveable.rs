use crate::components::Component;

pub struct MoveableComponent { }

impl MoveableComponent {
    pub fn new() -> Component {
        Component::Moveable(
            MoveableComponent {}
        )
    }
}
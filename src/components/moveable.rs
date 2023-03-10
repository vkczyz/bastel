use crate::components::Component;

#[derive(PartialEq)]
pub struct MoveableComponent { }

impl MoveableComponent {
    pub fn new() -> Component {
        Component::Moveable(
            MoveableComponent {}
        )
    }
}
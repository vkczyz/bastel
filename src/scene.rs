use crate::entity::Entity;
use crate::physics::Physics;

pub struct Scene {
    pub physics: Physics,
    pub entities: Vec<Entity>,
    pub player_index: usize,
}

impl Scene {
    pub fn new(entities: Vec<Entity>, player_index: usize) -> Self {
        Scene {
            entities,
            player_index,
            physics: Physics::new(),
        }
    }
}
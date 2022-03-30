use crate::entity::Entity;

pub struct Scene {
    pub entities: Vec<Entity>,
    pub player_index: usize,
}

impl Scene {
    pub fn new(entities: Vec<Entity>, player_index: usize) -> Self {
        Scene {
            entities,
            player_index,
        }
    }
}
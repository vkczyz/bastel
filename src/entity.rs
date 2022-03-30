use crate::physics::Physics;
use crate::sprite::Sprite;

pub struct Entity {
    pub sprite: Sprite,
    pub physics: Physics,
}

impl Entity {
    pub fn new(sprite: Sprite) -> Self {
        Entity {
            sprite,
            physics: Physics::new(),
        }
    }
}
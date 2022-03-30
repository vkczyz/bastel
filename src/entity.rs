use crate::sprite::Sprite;

pub struct Entity {
    pub sprite: Sprite,
}

impl Entity {
    pub fn new(sprite: Sprite) -> Self {
        Entity {
            sprite,
        }
    }
}
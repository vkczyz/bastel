use crate::physics::Physics;
use crate::sprite::Sprite;

#[derive(PartialEq)]
pub struct Entity {
    pub sprite: Sprite,
    pub physics: Physics,
    pub collideable: bool,
}

impl Entity {
    pub fn new(sprite: Sprite, collideable: bool) -> Self {
        Entity {
            sprite,
            physics: Physics::new(),
            collideable,
        }
    }

    pub fn are_colliding(a: &Entity, b: &Entity) -> bool {
        let a = &a.sprite;
        let b = &b.sprite;

        let a_right_b = a.get_left_pos() > b.get_right_pos();
        let a_left_b = a.get_right_pos() < b.get_left_pos();
        let a_above_b = a.get_bottom_pos() < b.get_top_pos();
        let a_below_b = a.get_top_pos() > b.get_bottom_pos();

        !(a_right_b || a_left_b || a_above_b || a_below_b)
    }
}
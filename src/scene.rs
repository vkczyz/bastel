use crate::entity::Entity;
use crate::physics::Physics;
use crate::shaders::Shader;
use crate::sprite::Sprite;

pub struct Scene {
    pub physics: Physics,
    pub entities: Vec<Entity>,
    pub player_index: usize,
}

impl Scene {
    pub fn new(entities: Vec<Entity>, player_index: usize) -> Self {
        let mut physics = Physics::new();
        physics.acceleration.1 = 0.001;

        let entities = vec![
            // Background
            Entity::new(
                Sprite::new(
                    (-1.0, -1.0),
                    (2.0, 2.0),
                    None,
                ),
                false,
            ),
            // Player
            Entity::new(
                Sprite::new(
                    (-0.5, -0.5),
                    (0.1, 0.1),
                    Some(Shader::Rainbow),
                ),
                true,
            ),
            // Borders
            Entity::new(
                Sprite::new(
                    (-1.0, -1.0),
                    (0.0, 2.0),
                    Some(Shader::Solid),
                ),
                true,
            ),
            Entity::new(
                Sprite::new(
                    (-1.0, -1.0),
                    (2.0, 0.0),
                    Some(Shader::Solid),
                ),
                true,
            ),
            Entity::new(
                Sprite::new(
                    (-1.0, 1.0),
                    (2.0, 0.0),
                    Some(Shader::Solid),
                ),
                true,
            ),
            Entity::new(
                Sprite::new(
                    (1.0, -1.0),
                    (0.0, 2.0),
                    Some(Shader::Solid),
                ),
                true,
            ),
        ];

        Scene {
            entities,
            player_index,
            physics,
        }
    }
}